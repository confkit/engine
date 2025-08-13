# EventHub 事件中心

## 概述

EventHub 是一个高性能、异步的事件驱动系统，用于在应用程序中实现解耦的事件发布和订阅机制。它提供了全局单例访问，支持多种事件类型和自定义订阅者。

## 核心特性

- **全局访问**: 通过单例模式在程序任意位置发布事件
- **异步处理**: 基于 tokio 异步运行时，支持高并发
- **批处理优化**: 支持事件批量处理，提升性能
- **错误隔离**: 单个订阅者错误不会影响其他订阅者
- **类型安全**: 利用 Rust 类型系统确保编译时安全
- **可扩展性**: 易于添加新的事件类型和订阅者

## 架构设计

```
EventHub
├── Event (事件定义)
│   ├── EventType (事件类型)
│   ├── LogLevel (日志级别)
│   └── metadata (元数据)
├── EventHub (事件中心)
│   ├── 全局单例
│   ├── 事件发布
│   ├── 订阅者管理
│   └── 工作线程池
└── Subscriber (订阅者)
    ├── EventSubscriber (订阅者接口)
    └── LogSubscriber (日志订阅者)
```

## 快速开始

### 1. 初始化事件中心

```rust
use std::sync::Arc;
use crate::infra::event_hub::{EventHub, LogSubscriber};

// 方式一：创建带默认路径的日志订阅者
let log_subscriber = Arc::new(LogSubscriber::with_default_path("logs/app.log"));

// 方式二：创建无默认路径的订阅者（日志路径必须通过事件传递）
let log_subscriber = Arc::new(LogSubscriber::new());

// 注册订阅者
EventHub::global().subscribe(log_subscriber).await;
```

### 2. 发布事件

```rust
use crate::infra::event_hub::{EventHub, Event, EventType, LogLevel};

// 发布日志事件（使用默认路径）
EventHub::global().publish(Event::new_log(
    LogLevel::Info,
    "应用程序启动".to_string(),
    "main".to_string(),
))?;

// 发布带自定义日志路径的事件
EventHub::global().publish(
    Event::new_log(
        LogLevel::Error,
        "数据库连接失败".to_string(),
        "database".to_string(),
    ).with_metadata("log_path".to_string(), "logs/database.log".to_string())
)?;

// 发布自定义事件
EventHub::global().publish(Event::new(
    EventType::Custom("user_action".to_string()),
    "用户点击了按钮".to_string(),
    "ui_handler".to_string(),
))?;
```

### 3. 创建自定义订阅者

```rust
use async_trait::async_trait;
use crate::infra::event_hub::{Event, EventSubscriber};

pub struct MetricsSubscriber;

#[async_trait]
impl EventSubscriber for MetricsSubscriber {
    fn name(&self) -> &'static str {
        "metrics_subscriber"
    }

    fn interested_events(&self) -> Vec<&'static str> {
        vec!["system_metrics", "task_status"]
    }

    async fn handle(&self, event: &Event) -> anyhow::Result<()> {
        // 处理指标事件
        println!("收集指标: {}", event.payload);
        Ok(())
    }
}
```

## 事件类型

### LogLevel (日志级别)

```rust
pub enum LogLevel {
    Trace,   // 追踪信息
    Debug,   // 调试信息
    Info,    // 一般信息
    Warn,    // 警告信息
    Error,   // 错误信息
}
```

### EventType (事件类型)

```rust
pub enum EventType {
    Log(LogLevel),           // 日志事件
    TaskStatus,              // 任务状态事件
    SystemMetrics,           // 系统监控事件
    Custom(String),          // 自定义事件
}
```

## 配置选项

### EventHubConfig

```rust
pub struct EventHubConfig {
    /// 事件队列缓冲区大小 (默认: 1024)
    pub buffer_size: usize,
    
    /// 工作线程数量 (默认: 2)
    pub worker_count: usize,
    
    /// 是否启用事件批处理 (默认: true)
    pub batch_processing: bool,
    
    /// 批处理大小 (默认: 32)
    pub batch_size: usize,
}
```

### 自定义配置

```rust
use crate::infra::event_hub::{EventHub, EventHubConfig};

let config = EventHubConfig {
    buffer_size: 2048,
    worker_count: 4,
    batch_processing: true,
    batch_size: 64,
};

let hub = EventHub::new(config);
```

## 内置订阅者

### LogSubscriber (日志订阅者)

负责将日志事件写入动态指定的文件系统路径。日志路径可以通过事件的 metadata 动态传递，实现更灵活的日志管理。

```rust
use crate::infra::event_hub::LogSubscriber;

// 方式一：无默认路径（日志路径必须通过事件 metadata 传递）
let log_subscriber = LogSubscriber::new();

// 方式二：带默认路径（当事件未指定路径时使用默认路径）
let log_subscriber = LogSubscriber::with_default_path("logs/app.log");

// 方式三：完全自定义配置
let log_subscriber = LogSubscriber::with_config(
    Some("logs/default.log"), // 默认路径（可选）
    true, // 启用时间戳
    "[{timestamp}] [{level}] {source}: {message}".to_string(), // 自定义格式
);
```

#### 动态日志路径

LogSubscriber 支持通过事件的 metadata 动态指定日志路径：

```rust
// 发布到特定日志文件的事件
let event = Event::new_log(
    LogLevel::Error,
    "数据库连接超时".to_string(),
    "database".to_string(),
).with_metadata("log_path".to_string(), "logs/database_errors.log".to_string());

EventHub::global().publish(event)?;
```

**路径解析优先级：**
1. 事件 metadata 中的 `log_path` 字段
2. LogSubscriber 的默认路径（如果设置了）
3. 如果都没有，返回错误

#### 日志格式模板

支持以下占位符：
- `{timestamp}`: 时间戳
- `{level}`: 日志级别
- `{source}`: 事件来源
- `{message}`: 消息内容

## 性能优化

### 1. 批处理

启用批处理可以显著提升性能，特别是在高频事件场景下：

```rust
let config = EventHubConfig {
    batch_processing: true,
    batch_size: 64, // 根据实际情况调整
    ..Default::default()
};
```

### 2. 工作线程数量

根据 CPU 核心数和事件处理复杂度调整工作线程数量：

```rust
let config = EventHubConfig {
    worker_count: num_cpus::get(), // 使用 CPU 核心数
    ..Default::default()
};
```

### 3. 缓冲区大小

适当增加缓冲区大小可以处理突发的高频事件：

```rust
let config = EventHubConfig {
    buffer_size: 4096, // 增加缓冲区
    ..Default::default()
};
```

## 错误处理

EventHub 采用了多层错误处理机制：

1. **事件发布错误**: 通过 `Result` 返回
2. **订阅者处理错误**: 记录日志但不影响其他订阅者
3. **系统级错误**: 通过 tracing 记录详细日志

```rust
// 错误处理示例
match EventHub::global().publish(event) {
    Ok(()) => println!("事件发布成功"),
    Err(e) => eprintln!("事件发布失败: {}", e),
}
```

## 最佳实践

### 1. 事件命名

使用清晰、一致的事件命名规范：

```rust
// 推荐
EventType::Custom("user_login".to_string())
EventType::Custom("task_completed".to_string())

// 不推荐
EventType::Custom("event1".to_string())
EventType::Custom("stuff_happened".to_string())
```

### 2. 订阅者设计

- 保持订阅者处理逻辑简单快速
- 避免在订阅者中执行长时间运行的操作
- 使用适当的错误处理

```rust
#[async_trait]
impl EventSubscriber for MySubscriber {
    async fn handle(&self, event: &Event) -> anyhow::Result<()> {
        // 快速处理
        self.process_quickly(event).await?;
        
        // 对于耗时操作，考虑异步处理
        let event = event.clone();
        tokio::spawn(async move {
            // 耗时操作
        });
        
        Ok(())
    }
}
```

### 3. 内存管理

- 避免在事件中包含大量数据
- 使用引用计数 (Arc) 共享大对象
- 定期清理不再需要的订阅者

## 监控和调试

### 1. 启用日志

```rust
use tracing::{info, debug, warn, error};

// EventHub 内部使用 tracing 进行日志记录
// 确保在应用中初始化 tracing
```

### 2. 监控指标

```rust
// 获取订阅者数量
let count = EventHub::global().subscriber_count().await;
println!("当前订阅者数量: {}", count);
```

## 故障排除

### 常见问题

1. **事件丢失**
   - 检查订阅者是否正确注册
   - 确认事件类型匹配

2. **性能问题**
   - 调整批处理大小
   - 增加工作线程数量
   - 检查订阅者处理逻辑

3. **内存泄漏**
   - 及时取消不需要的订阅者
   - 避免在事件中保存大对象

## 示例项目

完整的使用示例可以参考项目中的测试文件和示例代码。

## 依赖项

- `tokio`: 异步运行时
- `async-trait`: 异步 trait 支持
- `once_cell`: 单例模式
- `tracing`: 日志记录
- `anyhow`: 错误处理
- `chrono`: 时间处理
- `serde`: 序列化支持
- `uuid`: 唯一标识符生成
