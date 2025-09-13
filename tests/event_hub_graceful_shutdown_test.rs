// 测试 EventHub 优雅关闭功能
use confkit_engine::infra::event_hub::{Event, EventHub, EventType, LogLevel};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

async_test_attr! {
    #[pretty_async_test("EventHub 优雅关闭测试")]
    async fn test_event_hub_graceful_shutdown() {
        // 获取全局 EventHub
        let event_hub = EventHub::global();

        // 发布一些测试事件
        for i in 0..10 {
            let event = Event::new_log(
                LogLevel::Info,
                format!("Test message {}", i),
                "test".to_string()
            );

            event_hub.publish(event).expect("发布事件失败");
        }

        println!("发布了 10 个测试事件");

        // 模拟一些工作时间
        sleep(Duration::from_millis(100)).await;

        // 等待事件完成处理
        let wait_result = event_hub.wait_for_completion(2).await;
        assert!(wait_result.is_ok(), "等待事件完成失败");
        println!("✅ 事件处理完成");

        // 执行优雅关闭
        let shutdown_result = event_hub.shutdown(3).await;
        assert!(shutdown_result.is_ok(), "优雅关闭失败");
        println!("✅ EventHub 优雅关闭成功");
    }
}

async_test_attr! {
    #[pretty_async_test("EventHub 异步优雅关闭测试")]
    async fn test_event_hub_async_graceful_shutdown() {
        // 获取全局 EventHub
        let event_hub = EventHub::global();

        // 异步发布多个事件
        let publish_handle = tokio::spawn(async move {
            for i in 0..20 {
                let event = Event::new_log(
                    LogLevel::Info,
                    format!("Async test message {}", i),
                    "async_test".to_string()
                );

                if let Err(e) = event_hub.publish(event) {
                    eprintln!("发布事件失败: {}", e);
                }

                // 模拟异步工作
                sleep(Duration::from_millis(10)).await;
            }

            println!("异步发布了 20 个测试事件");
        });

        // 等待发布完成
        publish_handle.await.expect("发布任务失败");

        // 等待事件处理完成
        let wait_result = event_hub.wait_for_completion(3).await;
        assert!(wait_result.is_ok(), "等待事件完成失败");
        println!("✅ 异步事件处理完成");

        // 执行优雅关闭
        let shutdown_result = event_hub.shutdown(5).await;
        assert!(shutdown_result.is_ok(), "异步优雅关闭失败");
        println!("✅ EventHub 异步优雅关闭成功");
    }
}

// 导入测试工具
mod test_utils;
