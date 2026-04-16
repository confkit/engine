---
name: basic-rust-tui
description: Use when building or reviewing Rust TUI work. Apply practical lessons for runtime boundaries, redraw strategy, terminal recovery, UTF-8 input, scrolling, and mode-safe state handling.
user-invocable: false
---

# Basic Rust TUI

用于 Rust TUI 项目的基础经验收敛. 适用于 ratatui、crossterm 及类似终端交互项目.

## 渲染模型

- ratatui 是 immediate mode 渲染, 每一帧都应从当前状态重新生成 UI
- 渲染层只读状态, 不持有隐式 widget 状态, 不把业务决策塞进 `draw`
- UI 是否显示、显示什么、如何布局, 都应直接由当前状态决定
- 需要保留的数据放在 App / Component state, 不要依赖“上一次画面”推导当前行为

## 运行时边界

- 主循环只放在 runtime 层, 不要让业务状态直接控制循环生命周期
- App 层优先做纯状态推进, 用 `update(event) -> effects` 表达结果
- 副作用统一延迟到 runtime 执行, 不要把 IO、持久化、terminal 操作混进状态更新
- 退出统一通过 effect 或明确命令收口, 不要散落多个 `running = false`

## 事件与动作

- 小项目可以集中处理输入, 但一旦 keybind、模式、组件增多就要尽快拆层
- 推荐显式区分 `Event -> Action -> Update -> Draw`
- 输入映射、状态更新、渲染职责分离, 不要写成一个巨型 match
- 组件化项目优先让每个组件各自处理事件、更新状态、负责绘制

## 重绘与性能

- 每轮尽量批量处理事件, 不要一个事件就完整 redraw 一次
- redraw 必须建立在脏检查之上, 没有状态变化时不要重绘
- 重操作必须缓存, 尤其是 markdown 解析、语法高亮、复杂 layout 和长列表转换
- 热路径只计算关键状态摘要, 不要为脏检查遍历全部数据

## 终端恢复

- panic 安全是硬约束, 不要假设正常退出一定会发生
- 只要程序启用了 raw mode 或进入了 alternate screen, 就必须为退出路径提供可靠清理
- 使用 panic hook + Drop guard 双保险恢复终端
- 清理逻辑必须幂等, 显式 restore 和 Drop restore 不能互相冲突
- `disable_raw_mode()`、`LeaveAlternateScreen`、show cursor 等动作必须放在统一、可靠、可重复执行的清理路径中
- 不要把终端清理散落在多个 return 分支或 main 末尾的“理想路径”里
- 恢复顺序保持稳定: raw mode、颜色、cursor、alternate screen、flush
- raw mode 下不要依赖普通终端行为, 调试输出、换行和错误展示都要显式处理
- suspend / resume、panic、中途报错、主动退出都要走同一套 restore 语义

## 输入与 UTF-8

- 光标位置不要按字节拍脑袋递增, 必须按字符边界移动
- 插入、删除、左右移动都用 `char_indices` 或等价方式处理 UTF-8
- 中文、emoji、混合输入必须纳入测试
- 所有共享输入逻辑的模式都要统一使用同一套 cursor 语义

## 模式切换与共享状态

- 多模式共享字段时, 在进入和退出点都显式重置
- 不要假设上一个模式一定完成了清理
- 搜索、命令、编辑这类模式切换时, cursor、query、scroll、popup 状态要逐项确认
- 模式状态一旦共享过多, 优先拆成独立状态块

## 滚动语义

- 追加内容的视图优先使用“绝对位置 + auto scroll flag”, 不要用“距底部偏移”语义
- 用户手动滚动后, 新内容不应强行推走视角
- “跟到底部”和“保持当前视角”要分成两个明确状态
- 大列表滚动步长要有明确策略, 不要机械按 1 行移动

## 可测性与类型

- 效果枚举、事件枚举、关键状态类型优先补齐 `Debug`、`Clone`、`PartialEq`、`Eq`
- 让测试可以直接断言 `effects`、状态转移和滚动结果
- 终端恢复、UTF-8 输入、滚动语义、模式切换必须有专项测试
- 一开始就为 runtime / app / ui 分层, 测试成本会明显更低

## 结构升级时机

- 集中式事件处理适合很小的 demo, 但不适合长期增长的项目
- 出现多模式、多组件、多快捷键、多异步来源后, 要升级到 action / component 架构
- 鼠标命中、复杂交互或动态布局出现后, 优先缓存 layout 结果, 不要重复计算两套坐标语义
- 如果 TUI 还要临时切到外部程序, suspend / resume 和终端恢复路径必须单独验证

## 可选扩展

- 如果项目包含插件或脚本扩展, 宿主入口与插件导出入口必须对齐
- 如果扩展运行时存在预算限制, 失败路径必须可见, 不要允许静默失败
- 所有扩展调用失败都要有日志和诊断信息
