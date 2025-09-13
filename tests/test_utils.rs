// Test utilities for custom test macros

/// 真正接近属性宏语法的实现
///
/// 用法：
/// ```rust
/// test_attr! {
///     #[pretty_test("条件表达式解析")]
///     fn test_condition_expression_parsing() {
///         // 测试代码
///     }
/// }
/// ```
#[macro_export]
macro_rules! test_attr {
    (#[pretty_test($test_desc:expr)] fn $test_name:ident() $test_body:block) => {
        #[test]
        fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            let result = std::panic::catch_unwind(|| $test_body);

            match result {
                Ok(_) => println!("✅ 测试通过: {}\n", $test_desc),
                Err(e) => {
                    println!("❌ 测试失败: {}", $test_desc);
                    if let Some(msg) = e.downcast_ref::<String>() {
                        println!("   错误信息: {}\n", msg);
                    } else if let Some(msg) = e.downcast_ref::<&str>() {
                        println!("   错误信息: {}\n", msg);
                    }
                    std::panic::resume_unwind(e);
                }
            }
        }
    };
}

/// 异步版本的属性宏语法
///
/// 用法：
/// ```rust
/// async_test_attr! {
///     #[pretty_async_test("异步条件求值")]
///     async fn test_async_condition_evaluation() {
///         // 异步测试代码
///     }
/// }
/// ```
#[macro_export]
macro_rules! async_test_attr {
    (#[pretty_async_test($test_desc:expr)] async fn $test_name:ident() $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            // 执行异步测试体
            $test_body

            println!("✅ 测试通过: {}\n", $test_desc);
        }
    };
}

/// 创建一个类似属性宏的优雅调用方式
///
/// 用法：
/// ```rust
/// test_with_name!("测试描述", test_function_name, {
///     // 测试代码
/// });
/// ```
#[macro_export]
macro_rules! test_with_name {
    ($test_desc:expr, $test_name:ident, $test_body:block) => {
        #[test]
        fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            let result = std::panic::catch_unwind(|| $test_body);

            match result {
                Ok(_) => println!("✅ 测试通过: {}\n", $test_desc),
                Err(e) => {
                    println!("❌ 测试失败: {}", $test_desc);
                    if let Some(msg) = e.downcast_ref::<String>() {
                        println!("   错误信息: {}\n", msg);
                    } else if let Some(msg) = e.downcast_ref::<&str>() {
                        println!("   错误信息: {}\n", msg);
                    }
                    std::panic::resume_unwind(e);
                }
            }
        }
    };
}

/// 异步版本的测试宏
///
/// 用法：
/// ```rust
/// async_test_with_name!("异步测试描述", test_async_function_name, {
///     // 异步测试代码
/// });
/// ```
#[macro_export]
macro_rules! async_test_with_name {
    ($test_desc:expr, $test_name:ident, $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            // 执行异步测试体
            $test_body

            println!("✅ 测试通过: {}\n", $test_desc);
        }
    };
}

/// 更简洁的测试宏，自动从函数名推导描述
///
/// 用法：
/// ```rust  
/// pretty_test! {
///     fn test_condition_parsing() -> "条件解析" {
///         // 测试代码
///     }
/// }
/// ```
#[macro_export]
macro_rules! pretty_test {
    (fn $test_name:ident() -> $test_desc:literal $test_body:block) => {
        #[test]
        fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            let result = std::panic::catch_unwind(|| $test_body);

            match result {
                Ok(_) => println!("✅ 测试通过: {}\n", $test_desc),
                Err(e) => {
                    println!("❌ 测试失败: {}", $test_desc);
                    if let Some(msg) = e.downcast_ref::<String>() {
                        println!("   错误信息: {}\n", msg);
                    } else if let Some(msg) = e.downcast_ref::<&str>() {
                        println!("   错误信息: {}\n", msg);
                    }
                    std::panic::resume_unwind(e);
                }
            }
        }
    };
}

/// 异步版本的简洁测试宏
///
/// 用法：
/// ```rust
/// pretty_async_test! {
///     async fn test_async_condition() -> "异步条件测试" {
///         // 异步测试代码
///     }
/// }
/// ```
#[macro_export]
macro_rules! pretty_async_test {
    (async fn $test_name:ident() -> $test_desc:literal $test_body:block) => {
        #[tokio::test]
        async fn $test_name() {
            println!("\n🧪 开始测试: {}", $test_desc);

            // 执行异步测试体
            $test_body

            println!("✅ 测试通过: {}\n", $test_desc);
        }
    };
}
