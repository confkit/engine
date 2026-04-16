# Config Layering Reference

## 字段归类

| 字段类型 | 推荐位置 | 说明 |
| --- | --- | --- |
| API Key / Token / Secret | env only | 不落盘, 不进普通配置文件 |
| `base_url` / proxy / timeout | config + env | 既可能长期使用, 也可能临时覆盖 |
| model / profile / default params | config + cli | 默认走长期配置, 单次执行允许覆盖 |
| theme / layout / verbosity | config only | 用户稳定偏好, 不应依赖 env |
| debug / trace / 临时开关 | env or cli | 偏运行时控制, 不建议写入持久配置 |

## 优先级

```text
CLI > ENV > CONFIG > DEFAULT
```

- CLI 负责本次执行的显式意图
- ENV 负责环境差异、敏感信息和临时切换
- CONFIG 负责稳定策略和用户偏好
- DEFAULT 负责兜底

## 设计检查

- 是否明确区分敏感字段与非敏感字段
- 是否为长期配置和临时覆盖提供了清晰边界
- 是否所有层最终都汇总到同一个配置对象
- 是否文档化了字段来源、优先级和默认值
- 是否避免为同一语义重复设计多个 key

## 反模式

- 在 config file 中保存 API Key、Token、密码
- 业务代码里零散读取 env, 导致配置来源不可追踪
- 同一字段在 config、env、cli 中使用不同语义或不同默认值
- 把 UI 偏好、稳定策略全部塞进 env
- 为临时调试字段新增持久化配置项
