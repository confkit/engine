# Condition Module

条件表达式模块，为 ConfKit 提供基于环境变量的条件步骤执行能力。

## 功能概述

该模块实现了一个轻量级的条件表达式系统，支持：
- 环境变量解析
- 基础数据类型（字符串、数字、布尔值）
- 比较运算符（==、!=、>、<、>=、<=）
- 逻辑运算符（&&、||、!）
- 错误处理和降级策略

## 模块结构

```
src/core/condition/
├── mod.rs           # 模块导出和类型重导出
├── evaluator.rs     # 条件表达式求值引擎
├── parser.rs        # 表达式解析器
└── README.md        # 模块文档（本文件）
```

## 核心组件

### 1. 类型定义

**位置**: `src/types/condition.rs`

核心类型包括：
- `ConditionExpression` - 条件表达式 AST
- `ConditionValue` - 条件值类型
- `ComparisonOperator` - 比较运算符
- `LogicalOperator` - 逻辑运算符
- `FallbackStrategy` - 错误降级策略

### 2. 表达式解析器

**文件**: `parser.rs`  
**功能**: 将字符串条件表达式解析为 AST

支持的语法：
```
${VAR_NAME}           # 环境变量引用
"string"              # 字符串字面量
123, 123.45          # 数字字面量
true, false          # 布尔字面量
==, !=, >, <, >=, <= # 比较运算符
&&, ||, !            # 逻辑运算符
()                   # 分组括号
```

### 3. 条件求值器

**文件**: `evaluator.rs`  
**功能**: 执行条件表达式求值

核心特性：
- 环境变量自动解析
- 类型自动转换
- 递归深度限制（默认100层）
- 错误降级策略（默认无条件执行）

## 模块调用方法

### 1. 在步骤执行器中使用

**位置**: `src/core/executor/step_executor.rs`

```rust
use crate::core::condition::evaluator::ConditionEvaluator;

// 在步骤执行前进行条件检查
if let Some(condition_str) = &step.condition {
    let evaluator = ConditionEvaluator::new(context);
    let should_execute = evaluator.evaluate_string(condition_str)
        .unwrap_or_else(|e| {
            tracing::warn!("条件表达式求值失败，默认执行: {}", e);
            true
        });
    
    if !should_execute {
        return Ok(()); // 跳过当前步骤
    }
}
```

### 2. 独立使用条件求值器

```rust
use crate::core::condition::evaluator::ConditionEvaluator;
use crate::core::executor::context::ExecutionContext;

// 创建求值器（需要执行上下文）
let evaluator = ConditionEvaluator::new(&execution_context);

// 求值条件表达式
match evaluator.evaluate_string("${ENVIRONMENT} == 'production'") {
    Ok(result) => println!("条件结果: {}", result),
    Err(e) => eprintln!("求值错误: {}", e),
}
```

### 3. 在项目配置中使用

**步骤配置示例**:

```yaml
steps:
  - name: "生产环境部署"
    condition: "${ENVIRONMENT} == 'production' && ${ENABLE_DEPLOY} == true"
    commands:
      - "deploy.sh"
      
  - name: "开发环境测试"
    condition: "${ENVIRONMENT} != 'production'"
    commands:
      - "npm test"
```

## 条件表达式语法

### 变量引用
```
${VAR_NAME}           # 引用环境变量
${BUILD_NUMBER}       # 数字变量
${ENABLE_DEBUG}       # 布尔变量
```

### 比较运算
```
${ENV} == "prod"      # 字符串相等
${COUNT} > 10         # 数字比较
${FLAG} != false      # 布尔比较
```

### 逻辑运算
```
${A} && ${B}          # 逻辑与
${A} || ${B}          # 逻辑或
!${FLAG}              # 逻辑非
```

### 复合表达式
```
(${ENV} == "prod" || ${ENV} == "staging") && !${SKIP_DEPLOY}
```

## 类型转换规则

### 自动类型推断
- `"true"` / `"false"` → 布尔值
- 纯数字字符串 → 数字
- 其他 → 字符串

### Truthy 规则
- 布尔值：`true` 为真，`false` 为假
- 数字：非零为真，零为假
- 字符串：非空为真，空字符串为假
- Null：总是为假

## 错误处理

### 解析错误
当条件表达式语法错误时，默认采用**无条件执行**策略（返回 `true`），确保系统稳定性。

### 求值错误
- 未定义变量：使用默认值 `Null`
- 递归超限：抛出错误
- 类型转换失败：使用字符串表示

## 性能特性

- **轻量级设计**：无缓存系统，简单直接
- **递归限制**：防止无限递归导致栈溢出
- **内存友好**：最小化内存分配

## 扩展指南

### 添加新运算符

1. 在 `src/types/condition.rs` 中添加运算符枚举值
2. 在 `parser.rs` 中添加解析规则
3. 在 `evaluator.rs` 中添加求值逻辑

### 添加新数据类型

1. 扩展 `ConditionValue` 枚举
2. 更新 `from_string()` 转换逻辑
3. 更新 `is_truthy()` 规则

### 自定义降级策略

修改 `evaluator.rs` 中的 `handle_parse_error()` 方法。

## 注意事项

1. **环境变量依赖**: 条件求值依赖 `ExecutionContext` 中的环境变量
2. **线程安全**: 求值器是无状态的，可以安全地在多线程中使用
3. **错误恢复**: 默认采用保守策略，优先保证系统可用性
4. **性能考虑**: 复杂表达式会增加求值时间，建议保持表达式简洁

## 示例场景

### 环境判断
```yaml
condition: "${ENVIRONMENT} == 'production'"
```

### 功能开关
```yaml
condition: "${FEATURE_X_ENABLED} == true"
```

### 多条件组合
```yaml
condition: "${ENVIRONMENT} == 'prod' && ${BUILD_NUMBER} > 100"
```

### 条件跳过
```yaml
condition: "!${SKIP_TESTS}"
```