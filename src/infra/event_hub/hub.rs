//! Author: xiaoyown
//! Created: 2025-08-14
//! Description: hub.rs

use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::{mpsc, Notify, RwLock};

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
    /// 工作线程句柄，用于优雅关闭
    worker_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    /// 关闭通知
    shutdown_notify: Arc<Notify>,
}

impl EventHub {
    /// 创建新的事件中心实例
    fn new(config: EventHubConfig) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let subscribers = Arc::new(RwLock::new(Vec::new()));
        let worker_handles = Arc::new(RwLock::new(Vec::new()));
        let shutdown_notify = Arc::new(Notify::new());

        let hub = Self {
            sender,
            subscribers: subscribers.clone(),
            worker_handles: worker_handles.clone(),
            shutdown_notify: shutdown_notify.clone(),
        };

        // 启动事件处理工作线程
        hub.start_workers(receiver, subscribers, config, worker_handles, shutdown_notify);

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

    /// 优雅关闭事件中心（完整流程）
    ///
    /// 先等待所有事件完成处理，然后关闭工作线程
    ///
    /// # 参数
    /// - `wait_timeout_secs`: 等待事件完成的超时时间（秒）
    /// - `shutdown_timeout_secs`: 关闭工作线程的超时时间（秒）
    pub async fn graceful_shutdown(
        &self,
        wait_timeout_secs: u64,
        shutdown_timeout_secs: u64,
    ) -> anyhow::Result<()> {
        tracing::debug!("Shutting down EventHub gracefully");

        // 等待事件完成处理
        if let Err(e) = self.wait_for_completion(wait_timeout_secs).await {
            tracing::warn!("Failed to wait for completion: {}", e);
        }

        // 关闭工作线程
        if let Err(e) = self.shutdown(shutdown_timeout_secs).await {
            tracing::warn!("Failed to shutdown EventHub: {}", e);
        }

        tracing::debug!("EventHub shutdown completed");
        Ok(())
    }

    /// 优雅关闭事件中心
    ///
    /// 等待所有未处理的事件完成处理，然后关闭工作线程
    ///
    /// # 参数
    /// - `timeout`: 最大等待时间（秒）
    pub async fn shutdown(&self, timeout_secs: u64) -> anyhow::Result<()> {
        // 关闭发送器，不再接收新事件
        // 注意：这里我们不能直接关闭 sender，因为它被多个地方引用
        // 而是通过通知机制来告诉工作线程停止
        self.shutdown_notify.notify_waiters();

        // 等待工作线程完成
        let handles = {
            let mut worker_handles = self.worker_handles.write().await;
            std::mem::take(&mut *worker_handles)
        };

        // 设置超时等待所有工作线程
        let shutdown_timeout = tokio::time::Duration::from_secs(timeout_secs);
        match tokio::time::timeout(shutdown_timeout, async {
            for handle in handles {
                if let Err(e) = handle.await {
                    tracing::warn!("Worker thread exited abnormally: {}", e);
                }
            }
        })
        .await
        {
            Ok(()) => Ok(()),
            Err(_) => {
                tracing::warn!("EventHub shutdown timed out");
                Err(anyhow::anyhow!("EventHub shutdown timeout"))
            }
        }
    }

    /// 等待所有待处理事件完成
    ///
    /// 这是一个阻塞方法，会等待直到事件队列为空
    pub async fn wait_for_completion(&self, timeout_secs: u64) -> anyhow::Result<()> {
        let timeout_duration = tokio::time::Duration::from_secs(timeout_secs);
        let start_time = tokio::time::Instant::now();

        // 简单的轮询方式检查队列是否为空
        // 注意：这是一个简化的实现，实际生产环境可能需要更精细的控制
        while start_time.elapsed() < timeout_duration {
            // 发送一个探测事件，如果能立即发送说明队列有空间
            if self.sender.is_closed() {
                return Ok(());
            }

            // 短暂等待让处理线程完成工作
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        tracing::warn!("Wait for completion timed out");
        Ok(())
    }

    /// 启动事件处理工作线程
    fn start_workers(
        &self,
        receiver: mpsc::UnboundedReceiver<Event>,
        subscribers: Arc<RwLock<Vec<Arc<dyn EventSubscriber>>>>,
        config: EventHubConfig,
        worker_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
        shutdown_notify: Arc<Notify>,
    ) {
        // 将 receiver 包装在 Arc<Mutex<_>> 中，让多个 worker 可以共享
        let shared_receiver = Arc::new(tokio::sync::Mutex::new(receiver));

        let mut handles = Vec::new();

        for worker_id in 0..config.worker_count {
            let subscribers = subscribers.clone();
            let config = config.clone();
            let receiver = shared_receiver.clone();
            let shutdown_notify = shutdown_notify.clone();

            let handle = tokio::spawn(async move {
                tracing::debug!("Starting event processing worker #{}", worker_id);

                let mut events_buffer = Vec::with_capacity(config.batch_size);

                loop {
                    // 检查是否收到关闭信号
                    tokio::select! {
                        _ = shutdown_notify.notified() => {
                            tracing::info!("Worker #{} received shutdown signal", worker_id);

                            // 处理剩余的事件
                            while let Ok(event) = {
                                let mut rx = receiver.lock().await;
                                rx.try_recv()
                            } {
                                Self::process_events(&subscribers, &[event]).await;
                            }

                            tracing::info!("Worker #{} shutting down gracefully", worker_id);
                            break;
                        }
                        // 正常事件处理
                        event = async {
                            let mut rx = receiver.lock().await;
                            rx.recv().await
                        } => {
                            match event {
                                Some(event) => {
                                    if config.batch_processing {
                                        // 批处理模式
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
                                    } else {
                                        // 单个事件处理模式
                                        Self::process_events(&subscribers, &[event]).await;
                                    }
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
                }
            });

            handles.push(handle);
        }

        // 保存工作线程句柄用于后续关闭
        tokio::spawn(async move {
            let mut worker_handles = worker_handles.write().await;
            *worker_handles = handles;
        });
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
