use confkit_engine::core::condition::evaluator::ConditionEvaluator;
use std::collections::HashMap;

#[tokio::test]
async fn test_condition_evaluator_basic_comparison() {
    let mut env_vars = HashMap::new();
    env_vars.insert("ENVIRONMENT".to_string(), "production".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试字符串相等比较
    let result = evaluator.evaluate_string("${ENVIRONMENT} == \"production\"").unwrap();
    assert!(result);

    let result = evaluator.evaluate_string("${ENVIRONMENT} == \"development\"").unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_condition_evaluator_numeric_comparison() {
    let mut env_vars = HashMap::new();
    env_vars.insert("BUILD_NUMBER".to_string(), "123".to_string());
    env_vars.insert("MIN_VERSION".to_string(), "100".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试数字比较
    let result = evaluator.evaluate_string("${BUILD_NUMBER} > ${MIN_VERSION}").unwrap();
    assert!(result);

    let result = evaluator.evaluate_string("${BUILD_NUMBER} < 50").unwrap();
    assert!(!result);

    let result = evaluator.evaluate_string("${BUILD_NUMBER} >= 123").unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_condition_evaluator_boolean_operations() {
    let mut env_vars = HashMap::new();
    env_vars.insert("ENABLE_DEPLOY".to_string(), "true".to_string());
    env_vars.insert("SKIP_TESTS".to_string(), "false".to_string());
    env_vars.insert("ENVIRONMENT".to_string(), "production".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试布尔值解析
    let result = evaluator.evaluate_string("${ENABLE_DEPLOY} == true").unwrap();
    assert!(result);

    let result = evaluator.evaluate_string("!${SKIP_TESTS}").unwrap();
    assert!(result);

    // 测试逻辑运算符
    let result = evaluator.evaluate_string("${ENABLE_DEPLOY} && !${SKIP_TESTS}").unwrap();
    assert!(result);

    let result =
        evaluator.evaluate_string("${ENVIRONMENT} == \"production\" && ${ENABLE_DEPLOY}").unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_condition_evaluator_complex_expressions() {
    let mut env_vars = HashMap::new();
    env_vars.insert("ENVIRONMENT".to_string(), "staging".to_string());
    env_vars.insert("BUILD_NUMBER".to_string(), "150".to_string());
    env_vars.insert("FEATURE_FLAG".to_string(), "true".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试复合条件表达式
    let result = evaluator.evaluate_string(
        "(${ENVIRONMENT} == \"production\" || ${ENVIRONMENT} == \"staging\") && ${BUILD_NUMBER} > 100"
    ).unwrap();
    assert!(result);

    let result = evaluator
        .evaluate_string("${ENVIRONMENT} == \"production\" && ${BUILD_NUMBER} > 200")
        .unwrap();
    assert!(!result);

    // 测试带括号的复杂表达式
    let result = evaluator.evaluate_string(
        "(${ENVIRONMENT} == \"staging\" && ${FEATURE_FLAG}) || (${ENVIRONMENT} == \"production\" && ${BUILD_NUMBER} > 100)"
    ).unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_condition_evaluator_undefined_variables() {
    let env_vars = HashMap::new(); // 空环境变量
    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试未定义变量的处理
    let result = evaluator.evaluate_string("${UNDEFINED_VAR} == \"test\"").unwrap();
    assert!(!result); // null != "test"

    let result = evaluator.evaluate_string("!${UNDEFINED_VAR}").unwrap();
    assert!(result); // !null = true
}

#[tokio::test]
async fn test_condition_evaluator_error_handling() {
    let evaluator = ConditionEvaluator::new(HashMap::new());

    // 测试语法错误的表达式 (应该返回 true，降级策略)
    let result = evaluator.evaluate_string("${VAR ==").unwrap(); // 不完整的表达式
    assert!(result); // 降级策略返回 true

    let result = evaluator.evaluate_string("${VAR} ===== \"test\"").unwrap(); // 无效运算符
    assert!(result); // 降级策略返回 true

    let result = evaluator.evaluate_string("${VAR} && && ${VAR2}").unwrap(); // 语法错误
    assert!(result); // 降级策略返回 true
}

#[tokio::test]
async fn test_condition_evaluator_type_conversions() {
    let mut env_vars = HashMap::new();
    env_vars.insert("STRING_NUM".to_string(), "42".to_string());
    env_vars.insert("STRING_BOOL".to_string(), "true".to_string());
    env_vars.insert("EMPTY_STRING".to_string(), "".to_string());
    env_vars.insert("ZERO".to_string(), "0".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试自动类型转换
    let result = evaluator.evaluate_string("${STRING_NUM} > 40").unwrap();
    assert!(result);

    let result = evaluator.evaluate_string("${STRING_BOOL} == true").unwrap();
    assert!(result);

    // 测试 truthy 规则
    let result = evaluator.evaluate_string("!${EMPTY_STRING}").unwrap();
    assert!(result); // 空字符串为 false，所以 !false = true

    let result = evaluator.evaluate_string("!${ZERO}").unwrap();
    assert!(result); // 0 为 false，所以 !false = true
}

#[tokio::test]
async fn test_condition_with_step_execution() {
    let mut env_vars = HashMap::new();
    env_vars.insert("DEPLOY_ENV".to_string(), "production".to_string());

    let evaluator = ConditionEvaluator::new(env_vars);

    // 测试条件求值
    let should_execute = evaluator.evaluate_string("${DEPLOY_ENV} == \"production\"").unwrap();
    assert!(should_execute);

    // 测试条件不满足的情况
    let should_execute = evaluator.evaluate_string("${DEPLOY_ENV} == \"development\"").unwrap();
    assert!(!should_execute);
}
