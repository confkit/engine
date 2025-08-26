//! Author: xiaoyown
//! Created: 2025-08-14
//! Description: hub.rs

use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

use crate::infra::event_hub::{Event, EventSubscriber};

/// 事件中心配置
#[derive(Debug, Clone)]
struct EventHubConfig {
    /// 事件队列缓冲区大小
    // pub buffer_size: usize,
    /// 工作线程数量
    pub worker_count: usize,
    /// 是否启用事件批处理
    pub batch_processing: bool,
    /// 批处理大小
    pub batch_size: usize,
}

impl Default for EventHubConfig {
    fn default() -> Self {
        Self {
            // buffer_size: 1024,
            worker_count: 1,         // 单线程处理
            batch_processing: false, // 禁用批处理
            batch_size: 1,
        }
    }
}

/// 事件中心
///
/// 负责管理事件的发布和订阅，提供全局单例访问
pub struct EventHub {
    /// 事件发送器
    sender: mpsc::UnboundedSender<Event>,
    /// 订阅者列表
    subscribers: Arc<RwLock<Vec<Arc<dyn EventSubscriber>>>>,
    // /// 配置
    // config: EventHubConfig,
    // /// 是否已启动
    // started: Arc<Mutex<bool>>,
}

impl EventHub {
    /// 创建新的事件中心实例
    fn new(config: EventHubConfig) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let subscribers = Arc::new(RwLock::new(Vec::new()));

        let hub = Self {
            sender,
            subscribers: subscribers.clone(),
            // config: config.clone(),
            // started: Arc::new(Mutex::new(false)),
        };

        // 启动事件处理工作线程
        hub.start_workers(receiver, subscribers, config);

        hub
    }

    /// 获取全局事件中心实例
    pub fn global() -> &'static EventHub {
        static INSTANCE: OnceCell<EventHub> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            // tracing::info!("初始化全局事件中心");
            tracing::debug!("Initializing global event hub");
            EventHub::new(EventHubConfig::default())
        })
    }

    /// 发布事件
    ///
    /// # 参数
    /// - `event`: 要发布的事件
    ///
    /// # 返回值
    /// - `Ok(())`: 事件发布成功
    /// - `Err(e)`: 事件发布失败
    pub fn publish(&self, event: Event) -> anyhow::Result<()> {
        self.sender.send(event).map_err(|e| anyhow::anyhow!("发布事件失败: {}", e))?;

        Ok(())
    }

    /// 订阅事件
    ///
    /// # 参数
    /// - `subscriber`: 事件订阅者
    pub async fn subscribe(&self, subscriber: Arc<dyn EventSubscriber>) {
        let mut subscribers = self.subscribers.write().await;
        tracing::debug!("Subscribing to event: {}", subscriber.name());
        subscribers.push(subscriber);
    }

    // /// 取消订阅
    // ///
    // /// # 参数
    // /// - `subscriber_name`: 订阅者名称
    // pub async fn unsubscribe(&self, subscriber_name: &str) {
    //     let mut subscribers = self.subscribers.write().await;
    //     subscribers.retain(|s| s.name() != subscriber_name);
    //     tracing::debug!("Unsubscribing from event: {}", subscriber_name);
    // }

    // /// 获取订阅者数量
    // pub async fn subscriber_count(&self) -> usize {
    //     self.subscribers.read().await.len()
    // }

    /// 启动事件处理工作线程
    fn start_workers(
        &self,
        receiver: mpsc::UnboundedReceiver<Event>,
        subscribers: Arc<RwLock<Vec<Arc<dyn EventSubscriber>>>>,
        config: EventHubConfig,
    ) {
        // 将 receiver 包装在 Arc<Mutex<_>> 中，让多个 worker 可以共享
        let shared_receiver = Arc::new(tokio::sync::Mutex::new(receiver));

        for worker_id in 0..config.worker_count {
            let subscribers = subscribers.clone();
            let config = config.clone();
            let receiver = shared_receiver.clone();

            tokio::spawn(async move {
                tracing::debug!("Starting event processing worker #{}", worker_id);

                let mut events_buffer = Vec::with_capacity(config.batch_size);

                loop {
                    // 收集事件
                    if config.batch_processing {
                        // 批处理模式
                        let event = {
                            let mut rx = receiver.lock().await;
                            rx.recv().await
                        };

                        match event {
                            Some(event) => {
                                events_buffer.push(event);

                                // 尝试收集更多事件直到达到批大小或没有更多事件
                                while events_buffer.len() < config.batch_size {
                                    let next_event = {
                                        let mut rx = receiver.lock().await;
                                        rx.try_recv().ok()
                                    };

                                    match next_event {
                                        Some(event) => events_buffer.push(event),
                                        None => break,
                                    }
                                }

                                // 处理批量事件
                                Self::process_events(&subscribers, &events_buffer).await;
                                events_buffer.clear();
                            }
                            None => {
                                tracing::warn!(
                                    "Event receiver closed, worker #{} exiting",
                                    worker_id
                                );
                                break;
                            }
                        }
                    } else {
                        // 单个事件处理模式
                        let event = {
                            let mut rx = receiver.lock().await;
                            rx.recv().await
                        };

                        match event {
                            Some(event) => {
                                Self::process_events(&subscribers, &[event]).await;
                            }
                            None => {
                                tracing::warn!(
                                    "Event receiver closed, worker #{} exiting",
                                    worker_id
                                );
                                break;
                            }
                        }
                    }
                }
            });
        }
    }

    /// 处理事件列表
    async fn process_events(
        subscribers: &Arc<RwLock<Vec<Arc<dyn EventSubscriber>>>>,
        events: &[Event],
    ) {
        let subscribers = subscribers.read().await;

        for event in events {
            // 并发处理所有感兴趣的订阅者
            let mut handles = Vec::new();

            for subscriber in subscribers.iter() {
                if subscriber.is_interested(event) {
                    let subscriber = subscriber.clone();
                    let event = event.clone();

                    let handle = tokio::spawn(async move {
                        if let Err(e) = subscriber.handle(&event).await {
                            tracing::error!(
                                "Failed to handle event: {} - Error: {}",
                                event.type_str(),
                                e
                            );
                        }
                    });

                    handles.push(handle);
                }
            }

            // 等待所有订阅者处理完成
            for handle in handles {
                if let Err(e) = handle.await {
                    tracing::error!("Failed to handle event: {}", e);
                }
            }
        }
    }
}
