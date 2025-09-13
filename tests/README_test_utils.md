# 测试工具使用说明

## 概述

`test_utils.rs` 模块提供了接近属性宏语法的自定义测试宏，用于统一测试的起始、成功、失败打印格式。

## 🌟 推荐语法 - 接近属性宏

### 1. `test_attr!` - 类属性宏语法（同步）

**最接近 `#[test]` 的优雅语法！**

```rust
test_attr! {
    #[pretty_test("条件表达式解析")]
    fn test_condition_parsing() {
        // 测试代码
        assert_eq!(2 + 2, 4);
        println!("解析完成");
    }
}
```

### 2. `async_test_attr!` - 类属性宏语法（异步）

**异步版本的属性宏语法**

```rust
async_test_attr! {
    #[pretty_async_test("异步条件求值")]
    async fn test_async_evaluation() {
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        assert_eq!(5 + 5, 10);
    }
}
```

## 📋 传统语法（备选方案）

### 3. `test_with_name!` - 传统宏调用

```rust
test_with_name!("测试描述", test_function_name, {
    // 测试代码
    assert_eq!(3 * 3, 9);
});
```

### 4. `async_test_with_name!` - 异步传统宏调用

```rust
async_test_with_name!("异步测试描述", test_async_function_name, {
    // 异步测试代码
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    assert_eq!(4 * 4, 16);
});
```

## 输出示例

所有宏都会产生统一的输出格式：

```
🧪 开始测试: 条件表达式解析
解析完成
✅ 测试通过: 条件表达式解析
```

失败时：
```
🧪 开始测试: 失败示例
❌ 测试失败: 失败示例
   错误信息: assertion failed: `(left == right)`
```

## 使用方法

1. 在测试文件中添加模块引用：
```rust
mod test_utils;
```

2. 使用推荐的类属性宏语法：
```rust
test_attr! {
    #[pretty_test("我的测试")]
    fn my_test() {
        // 测试代码
    }
}

async_test_attr! {
    #[pretty_async_test("我的异步测试")]
    async fn my_async_test() {
        // 异步测试代码
    }
}
```

## 🎯 语法对比

| 方式 | 语法 | 优雅度 | 推荐度 |
|-----|------|-------|--------|
| **类属性宏** | `test_attr! { #[pretty_test("描述")] fn test() {} }` | ⭐⭐⭐⭐⭐ | ✅ **强烈推荐** |
| **传统宏** | `test_with_name!("描述", test, { })` | ⭐⭐⭐ | ✅ 可选 |

## 特性

- ✅ **最接近属性宏**：`#[pretty_test("描述")]` 语法
- ✅ **统一输出格式**：🧪 开始 → ✅ 成功 / ❌ 失败
- ✅ **支持中文描述**：完美显示中文测试说明
- ✅ **错误信息详细**：自动捕获和显示 panic 信息
- ✅ **同步异步支持**：覆盖所有测试场景
- ✅ **IDE 友好**：语法高亮和补全支持

## 注意事项

1. **必须使用 `cargo test -- --nocapture`** 才能看到完整的彩色输出
2. 测试描述使用字符串字面量，支持中英文
3. 函数名遵循 Rust 命名规范
4. 类属性宏语法需要包装在对应的宏块中

## 示例文件

参见 `tests/test_macros_example.rs` 查看完整使用示例。