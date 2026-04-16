# Conditional Step Execution

Add a `condition` field to any step to control execution. The expression is evaluated before the step runs; `true` executes, otherwise skipped.

## Syntax

Variables use `${VAR_NAME}`, strings in single quotes, numbers compared directly.

```yaml
steps:
  - name: "Production Build"
    condition: "${ENVIRONMENT} == 'production'"
    commands:
      - "npm run build:prod"
```

## Operators

### Comparison

| Operator | Description |
|----------|-------------|
| `==` | Equal |
| `!=` | Not equal |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater or equal |
| `<=` | Less or equal |

### Logical

| Operator | Description |
|----------|-------------|
| `&&` | AND |
| `\|\|` | OR |
| `!` | NOT |

## Examples

### Multiple conditions

```yaml
condition: "${ENVIRONMENT} == 'staging' && ${GIT_BRANCH} == 'main'"
```

### Numeric

```yaml
condition: "${BUILD_NUMBER} > 100"
```

### Nested

```yaml
condition: "(${ENVIRONMENT} == 'production' || ${ENVIRONMENT} == 'staging') && !${SKIP_TESTS}"
```

### Boolean

```yaml
condition: "${ENABLE_DEBUG} == true"
```

## Fallback

When expression cannot be parsed or evaluated: step is **skipped** (safe default).

## Performance

- Parsed once, cached for reuse
- Simple < 10ms, complex < 50ms
