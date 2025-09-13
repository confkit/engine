use confkit_engine::core::condition::evaluator::ConditionEvaluator;
use confkit_engine::core::condition::parser::parse_condition;
use confkit_engine::types::condition::{
    ComparisonOperator, ConditionExpression, ConditionValue, LogicalOperator,
};
use std::collections::HashMap;

mod common;
mod test_utils;

test_attr! {
    #[pretty_test("条件表达式解析")]
    fn test_condition_expression_parsing() {
        // 测试变量解析
        let input = "${VAR}";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Variable { name } => assert_eq!(name, "VAR"),
                    _ => panic!("Expected Variable expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }

        // 测试布尔字面量解析
        let input = "true";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Literal { value } => {
                        assert_eq!(value, ConditionValue::Boolean(true));
                    }
                    _ => panic!("Expected Literal expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }

        // 测试数字字面量解析
        let input = "123";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Literal { value } => {
                        assert_eq!(value, ConditionValue::Number(123.0));
                    }
                    _ => panic!("Expected Literal expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }

        // 测试字符串字面量解析
        let input = "\"hello\"";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Literal { value } => {
                        assert_eq!(value, ConditionValue::String("hello".to_string()));
                    }
                    _ => panic!("Expected Literal expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }

        // 测试比较表达式解析
        let input = "${VAR} == \"test\"";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Comparison { operator, .. } => {
                        assert_eq!(operator, ComparisonOperator::Equal);
                    }
                    _ => panic!("Expected Comparison expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }

        // 测试逻辑表达式解析
        let input = "${VAR1} && ${VAR2}";
        match parse_condition(input) {
            Ok(expr) => {
                match expr {
                    ConditionExpression::Logical { operator, .. } => {
                        assert_eq!(operator, LogicalOperator::And);
                    }
                    _ => panic!("Expected Logical expression"),
                }
            }
            Err(e) => panic!("Parse failed: {:?}", e),
        }
    }
}

test_attr! {
    #[pretty_test("简单条件求值")]
    fn test_simple_condition_evaluation() {
        // 测试简单的真值
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1", true),
            ("0", false),
            ("\"hello\"", true),
            ("\"\"", false),
        ];

        for (input, expected) in tests {
            match parse_condition(input) {
                Ok(expr) => {
                    match expr {
                        ConditionExpression::Literal { value } => {
                            let result = value.is_truthy();
                            println!("eval: {} -> {}", input, result);
                            assert_eq!(result, expected);
                        }
                        _ => {
                            // For non-literal expressions, we can't easily evaluate without context
                            // println!("skip: {} (需要上下文)", input);
                        }
                    }
                }
                Err(e) => panic!("Parse failed for '{}': {:?}", input, e),
            }
        }
    }
}

async_test_attr! {
    #[pretty_async_test("完整条件求值")]
    async fn test_complete_condition_evaluation() {
        // 创建测试环境变量
        let mut env_vars = std::collections::HashMap::new();
        env_vars.insert("ENVIRONMENT".to_string(), "production".to_string());
        env_vars.insert("BUILD_NUMBER".to_string(), "150".to_string());
        env_vars.insert("ENABLE_DEPLOY".to_string(), "true".to_string());
        println!("- ENVIRONMENT: production");
        println!("- BUILD_NUMBER: 150");
        println!("- ENABLE_DEPLOY: true");

        // 创建条件求值器
        let evaluator = ConditionEvaluator::new(env_vars);

        // 测试完整的条件表达式求值
        let test_cases = vec![
            ("${ENVIRONMENT} == \"production\"", true),
            ("${ENVIRONMENT} == \"development\"", false),
            ("${ENVIRONMENT} == \"production\" && ${BUILD_NUMBER} > 200", false),
            ("${BUILD_NUMBER} > 100", true),
            ("${BUILD_NUMBER} < 100", false),
            ("${ENABLE_DEPLOY} == true", true),
            ("!${ENABLE_DEPLOY}", false),
            ("${ENABLE_DEPLOY} && ${BUILD_NUMBER} > 100", true),
            ("${UNDEFINED_VAR} == \"test\"", false),
            ("${UNDEFINED_VAR}", false),        // null 值为 falsy
            ("!${UNDEFINED_VAR}", true),        // !null = true
            ("${UNDEFINED_VAR} == null", true), // null == null
        ];

        for (input, expected) in test_cases {
            match evaluator.evaluate_string(input) {
                Ok(result) => {
                    println!("eval: {} -> {}", input, result);
                    assert_eq!(result, expected, "Failed for expression: {}", input);
                }
                Err(e) => panic!("Evaluation failed for '{}': {:?}", input, e),
            }
        }
    }
}

test_attr! {
    #[pretty_test("条件求值器错误处理")]
    fn test_condition_evaluator_error_handling() {
        let env_vars = HashMap::new();
        let evaluator = ConditionEvaluator::new(env_vars);

        // 测试语法错误的条件（应该返回 true，降级策略）
        let invalid_condition = "${VAR ===== invalid";
        let result = evaluator.evaluate_string(invalid_condition).unwrap();
        println!("eval: {} -> {} (降级策略)", invalid_condition, result);
        assert!(result, "无效条件应该返回 true (降级策略)");

        // 测试未定义变量的条件
        let undefined_condition = "${UNDEFINED_VAR} == \"test\"";
        let result = evaluator.evaluate_string(undefined_condition).unwrap();
        println!("eval: {} -> {}", undefined_condition, result);
        assert!(!result, "未定义变量条件应该返回 false");
    }
}

test_attr! {
    #[pretty_test("条件求值器实际场景")]
    fn test_condition_evaluator_practical_scenarios() {
        // 模拟 CI/CD 环境变量
        let mut env_vars = HashMap::new();
        env_vars.insert("CI_ENVIRONMENT".to_string(), "staging".to_string());
        env_vars.insert("CI_BRANCH".to_string(), "develop".to_string());
        env_vars.insert("CI_BUILD_NUMBER".to_string(), "245".to_string());
        env_vars.insert("DEPLOY_ENABLED".to_string(), "true".to_string());

        let evaluator = ConditionEvaluator::new(env_vars);

        let scenarios = vec![
            // 只在 staging 环境运行的步骤
            ("${CI_ENVIRONMENT} == \"staging\"", true),
            // 只在 main 分支运行的步骤
            ("${CI_BRANCH} == \"main\"", false),
            // 构建号大于 200 的步骤
            ("${CI_BUILD_NUMBER} > 200", true),
            // 部署步骤：staging 环境 + 部署启用
            ("${CI_ENVIRONMENT} == \"staging\" && ${DEPLOY_ENABLED} == true", true),
            // 生产部署：只在 main 分支 + 生产环境
            ("${CI_BRANCH} == \"main\" && ${CI_ENVIRONMENT} == \"production\"", false),
            // 测试步骤：非生产环境
            ("${CI_ENVIRONMENT} != \"production\"", true),
        ];

        for (condition, expected) in scenarios {
            let result = evaluator.evaluate_string(condition).unwrap();
            println!("eval: {} -> {}", condition, result);
            assert_eq!(result, expected, "条件 '{}' 应该返回 {}", condition, expected);
        }
    }
}
