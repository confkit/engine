# Conditional Step Execution

ConfKit supports conditional execution of build steps based on environment variables and runtime conditions. Add a `condition` field to any step to control when it should execute.

## Basic Syntax

```yaml
steps:
  - name: "Production Build"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "npm run build:prod"

  - name: "Development Build"
    condition: "${ENVIRONMENT} == 'development'"
    commands:
      - "npm run build:dev"
```

When a step has a `condition` field, the expression is evaluated before execution. If the result is `true`, the step runs; otherwise it is skipped.

---

## Supported Operators

### Comparison Operators

| Operator | Description |
|----------|-------------|
| `==` | Equal to |
| `!=` | Not equal to |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater than or equal to |
| `<=` | Less than or equal to |

### Logical Operators

| Operator | Description |
|----------|-------------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

---

## Advanced Examples

### Multiple Conditions with Logical Operators

```yaml
- name: "Deploy to Staging"
  condition: "${ENVIRONMENT} == 'staging' && ${GIT_BRANCH} == 'main'"
  commands:
    - "deploy.sh staging"
```

### Numeric Comparisons

```yaml
- name: "Performance Test"
  condition: "${BUILD_NUMBER} > 100"
  commands:
    - "npm run test:performance"
```

### Complex Nested Conditions

```yaml
- name: "Quality Gate"
  condition: "(${ENVIRONMENT} == 'production' || ${ENVIRONMENT} == 'staging') && !${SKIP_TESTS}"
  commands:
    - "npm run test:quality"
```

### Boolean Variables

```yaml
- name: "Debug Mode"
  condition: "${ENABLE_DEBUG} == true"
  commands:
    - "echo 'Debug mode enabled'"
```

---

## Expression Syntax

- Variables use `${VAR_NAME}` syntax
- String values are wrapped in single quotes: `'value'`
- Numeric values are compared directly: `${NUM} > 100`
- Boolean values can be compared: `${FLAG} == true`
- Parentheses `()` can group sub-expressions
- The `!` operator negates a variable or expression

---

## Fallback Behavior

When a condition expression cannot be parsed or evaluated:

- **Default**: Step is **skipped** (safe fallback)
- **Configurable**: Can be configured to execute unconditionally or use custom fallback logic

## Performance

- Expressions are parsed once and cached for reuse
- Environment variable values are cached during task execution
- Simple expressions evaluate in < 10ms, complex expressions in < 50ms
