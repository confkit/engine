// 演示如何使用接近属性宏语法的测试宏
mod test_utils;

// 方式1: 接近真正属性宏的语法！
test_attr! {
    #[pretty_test("条件表达式解析")]
    fn test_condition_parsing() {
        println!("  解析变量表达式...");
        assert_eq!(2 + 2, 4);
        println!("  解析完成");
    }
}

test_attr! {
    #[pretty_test("简单条件求值")]
    fn test_simple_evaluation() {
        assert_eq!(3 * 3, 9);
        println!("  求值测试完成");
    }
}

test_attr! {
    #[pretty_test("错误处理测试")]
    fn test_error_handling() {
        println!("  测试错误处理逻辑...");
        assert_eq!(10 / 2, 5);
        println!("  错误处理测试完成");
    }
}

// 方式2: 异步版本的属性宏语法
async_test_attr! {
    #[pretty_async_test("异步条件求值")]
    async fn test_async_evaluation() {
        println!("  执行异步测试逻辑...");

        // 模拟异步操作
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        assert_eq!(5 + 5, 10);
        println!("  异步测试完成");
    }
}

async_test_attr! {
    #[pretty_async_test("异步错误处理")]
    async fn test_async_error_handling() {
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        assert_eq!(4 * 4, 16);
        println!("  异步错误处理完成");
    }
}

// 旧版本的调用方式也保留支持
test_with_name!("传统风格测试", test_legacy_style, {
    assert_eq!(7 * 8, 56);
    println!("  传统风格测试完成");
});

// 故意失败的测试示例（注释掉以免影响 CI）
/*
test_attr! {
    #[pretty_test("失败示例")]
    fn test_failure_example() {
        println!("  这个测试会失败...");
        assert_eq!(1, 2, "1 应该等于 2");
    }
}
*/
