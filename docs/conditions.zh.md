# 条件步骤执行

ConfKit 支持基于环境变量和运行时条件的条件步骤执行。在任何步骤中添加 `condition` 字段来控制其执行条件。

## 基础语法

```yaml
steps:
  - name: "生产环境构建"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "npm run build:prod"

  - name: "开发环境构建"
    condition: "${ENVIRONMENT} == 'development'"
    commands:
      - "npm run build:dev"
```

当步骤包含 `condition` 字段时，执行前会先求值表达式。结果为 `true` 则执行步骤，否则跳过。

---

## 支持的运算符

### 比较运算符

| 运算符 | 说明 |
|--------|------|
| `==` | 等于 |
| `!=` | 不等于 |
| `>` | 大于 |
| `<` | 小于 |
| `>=` | 大于等于 |
| `<=` | 小于等于 |

### 逻辑运算符

| 运算符 | 说明 |
|--------|------|
| `&&` | 逻辑与 |
| `\|\|` | 逻辑或 |
| `!` | 逻辑非 |

---

## 高级示例

### 多条件逻辑组合

```yaml
- name: "部署到测试环境"
  condition: "${ENVIRONMENT} == 'staging' && ${GIT_BRANCH} == 'main'"
  commands:
    - "deploy.sh staging"
```

### 数值比较

```yaml
- name: "性能测试"
  condition: "${BUILD_NUMBER} > 100"
  commands:
    - "npm run test:performance"
```

### 复杂嵌套条件

```yaml
- name: "质量门禁"
  condition: "(${ENVIRONMENT} == 'production' || ${ENVIRONMENT} == 'staging') && !${SKIP_TESTS}"
  commands:
    - "npm run test:quality"
```

### 布尔变量

```yaml
- name: "调试模式"
  condition: "${ENABLE_DEBUG} == true"
  commands:
    - "echo '调试模式已启用'"
```

---

## 表达式语法

- 变量使用 `${变量名}` 语法
- 字符串值使用单引号包裹：`'value'`
- 数值直接比较：`${NUM} > 100`
- 布尔值可直接比较：`${FLAG} == true`
- 括号 `()` 可用于分组子表达式
- `!` 运算符用于取反变量或表达式

---

## 降级策略

当条件表达式无法解析或求值时：

- **默认行为**：步骤**被跳过**（安全降级）
- **可配置**：可配置为无条件执行或使用自定义降级逻辑

## 性能优化

- 表达式解析一次后缓存供重复使用
- 任务执行期间缓存环境变量值
- 简单表达式求值 < 10ms，复杂表达式 < 50ms
