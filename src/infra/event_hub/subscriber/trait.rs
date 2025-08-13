//! Author: xiaoyown
//! Created: 2025-08-14
//! Description: trait.rs

use crate::infra::event_hub::Event;
use async_trait::async_trait;

/// 事件订阅者 trait
///
/// 所有事件订阅者都需要实现这个 trait，用于处理特定类型的事件。
///
/// # 示例
///
/// ```rust
/// use async_trait::async_trait;
/// use crate::infra::event_hub::{Event, EventSubscriber};
///
/// pub struct MySubscriber;
///
/// #[async_trait]
/// impl EventSubscriber for MySubscriber {
///     fn name(&self) -> &'static str {
///         "my_subscriber"
///     }
///
///     fn interested_events(&self) -> Vec<&'static str> {
///         vec!["log", "task_status"]
///     }
///
///     async fn handle(&self, event: &Event) -> anyhow::Result<()> {
///         println!("Handling event: {}", event.payload);
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait EventSubscriber: Send + Sync + 'static {
    /// 获取订阅者名称（用于标识和日志）
    fn name(&self) -> &'static str;

    /// 获取订阅者关注的事件类型列表
    /// 返回事件类型字符串列表，如 ["log", "task_status"]
    fn interested_events(&self) -> Vec<&'static str>;

    /// 处理接收到的事件
    ///
    /// # 参数
    /// - `event`: 要处理的事件
    ///
    /// # 返回值
    /// - `Ok(())`: 事件处理成功
    /// - `Err(e)`: 事件处理失败，包含错误信息
    ///
    /// # 注意
    /// - 此方法应该是幂等的，多次调用同一事件应该产生相同结果
    /// - 处理过程中的错误不应该导致整个事件系统崩溃
    /// - 长时间运行的操作应该考虑使用超时机制
    async fn handle(&self, event: &Event) -> anyhow::Result<()>;

    /// 订阅者是否对特定事件感兴趣
    ///
    /// 默认实现会检查事件类型是否在 `interested_events()` 列表中
    fn is_interested(&self, event: &Event) -> bool {
        self.interested_events().contains(&event.type_str())
    }
}
