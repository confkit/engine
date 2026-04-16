---
paths:
  - "**/*.rs"
---

# Rust 规范

## 文件头

新建文件必须添加文件头:

```rust
//! Description: ***
//!
//! Author: <username>
//! Created: YYYY-MM-DD
```

## 改动范围

- 优先保证可维护性、可测试性、可读性
- 先做最小正确改动, 避免无关重构
- 默认保持现有行为和外部接口不变, 除非任务明确要求调整
- 避免一次修改过多文件, 优先局部收敛

## 架构约束

- 业务层不要直接依赖裸 `while`、`loop` 或忙轮询
- runtime、driver、event loop 最外层允许保留主循环, 但必须封装
- 业务逻辑优先拆成 `State`、`Event`、`Action` / `Command`、`update` / handler
- 副作用与纯状态推进分离, 不要把 IO、网络、文件、terminal 操作混进状态更新
- 遇到复杂分支时, 优先状态机, 不要堆叠 `if/else`

## 循环与异步

- 不要写空转循环
- 没有事件或状态变化时, 不要持续 redraw
- 长耗时任务不要阻塞主线程或主循环
- 优先事件驱动、消息驱动、channel、iterator、stream、`tokio::select!`
- 保留 `loop` / `while` 时, 退出条件、阻塞点、清理路径必须清晰可见
- async 适合 IO, 不要把 CPU 密集逻辑机械 async 化
- 避免在高频路径堆 `spawn` + `await` + `join`

## TUI / CLI

- 所有退出路径统一收口
- terminal restore、raw mode 退出、cursor 恢复、alternate screen 退出必须一致
- 启用 raw mode 或 alternate screen 后, `disable_raw_mode()`、`LeaveAlternateScreen`、show cursor 必须进入可靠清理路径
- 不要把终端清理动作散落在多个 return 分支或只放在 main 末尾的理想路径
- 输入处理、状态更新、渲染分层, 不要混写在一个大函数里
- redraw 必须有明确触发条件
- 错误输出不能破坏 terminal 状态
- 不要每帧都 draw, 只在状态变化或 tick 时 redraw
- 不要在 UI 线程做 IO, 后台任务通过 channel 回传结果
- tick 频率必须克制, 非动画场景优先 100ms 以上

## 函数与类型

- 函数保持单一职责, 避免超长函数
- 优先小而稳定的类型定义, 不要依赖隐式共享状态
- 公共接口命名必须清晰, 避免 `handle_all`、`process_everything` 这类模糊命名
- 能用类型表达约束时, 不要只靠注释

## 错误处理

- 禁止无理由 `unwrap()` / `expect()`, 示例代码除外
- 返回可传播的错误, 错误信息必须可定位
- 清理逻辑不能依赖理想路径才能执行
- 显式处理 panic 风险点, 尤其是 TUI restore 场景
- 热路径不要构造复杂错误信息, 不要在循环里频繁 `anyhow!()`

## 状态管理

- 减少可变共享状态
- 不要把所有状态堆进一个巨大的 `App` 而不分层
- 临时状态、派生状态、持久状态分清
- 相同语义只保留一个数据源, 避免状态重复
- 优先拆分 UI state、domain state、cache, 不要混成单一巨型状态

## 性能约束

- 热路径避免频繁创建 `String`、`Vec` 和无理由 `clone()`
- 已知容量时使用 `with_capacity`
- 能借用就借用, 避免无意义 `to_string()`、`format!()`、`clone()`
- 避免重复遍历和中间 `collect`, 不要把简单流程拆成多次分配
- 热路径禁止 IO、`println!()`、`dbg!()` 和调试输出
- 锁作用域保持最小, 多读少写场景优先更合适的并发模型
- 小数据优先考虑 `Vec`, 不要机械使用 `HashMap`

## 风格

- 优先标准库和成熟方案, 避免无必要自造轮子
- 代码风格保持一致, 不写炫技式抽象
- 注释解释“为什么”, 不要重复“做了什么”
- 新增代码必须和现有项目风格对齐
- 性能优化先 profiling, 不要靠猜测重写结构
- mod.rs 仅作为模块入口与导出收口层；禁止承载核心业务逻辑

## 风险预防

- 先规避 panic、terminal 状态破坏、清理失败, 再谈性能
- 先规避空转循环、主链路阻塞、状态重复源, 再谈抽象优化
- 先修不可接受异常和退出路径问题, 不把风险留给 audit 才发现
