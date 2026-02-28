---
paths:
  - "**/*.tsx"
  - "**/*.jsx"
globs:
  - "**/package.json"
---

# Fukict 框架开发规范

## 适用条件

当项目 package.json 包含 `@fukict/*` 依赖时激活此规则.

## 核心原则

- 使用 `this.update()` 手动触发更新
- 复杂场景优先使用 `fukict:detach` 脱围模式
- 直接 DOM 操作优于虚拟 DOM diff

## 按需查阅

根据开发场景查阅对应文档:

| 场景          | 文档路径                                    |
| ------------- | ------------------------------------------- |
| 项目搭建      | `rules/frameworks/fukict/01-setup.md`       |
| 组件开发      | `rules/frameworks/fukict/02-components.md`  |
| 性能优化      | `rules/frameworks/fukict/03-performance.md` |
| 列表渲染      | `rules/frameworks/fukict/04-lists.md`       |
| 组件状态      | `rules/frameworks/fukict/05-state.md`       |
| 路由配置      | `rules/frameworks/fukict/06-routing.md`     |
| 构建部署      | `rules/frameworks/fukict/07-build.md`       |
| Flux 状态管理 | `rules/frameworks/fukict/08-flux.md`        |
| 国际化        | `rules/frameworks/fukict/09-i18n.md`        |
