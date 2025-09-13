use confkit_engine::types::condition::{
    ComparisonOperator, ConditionExpression, ConditionValue, LogicalOperator,
};

#[test]
fn test_condition_value_from_string() {
    // println!("=== 测试 ConditionValue::from_string: test_condition_value_from_string ===");

    // 测试布尔值解析
    let input = "true";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::Boolean(true));

    let input = "false";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::Boolean(false));

    // 测试数字解析
    let input = "123";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::Number(123.0));

    let input = "123.45";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::Number(123.45));

    // 测试字符串解析
    let input = "hello";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::String("hello".to_string()));

    let input = "";
    let result = ConditionValue::from_string(input);
    // println!("{}: {} -> {:?}", "from_string", input, result);
    assert_eq!(result, ConditionValue::Null);

    // println!("✅ ConditionValue::from_string() 测试通过\n");
}

#[test]
fn test_condition_value_is_truthy() {
    // println!("=== 测试 ConditionValue::is_truthy: test_condition_value_is_truthy ===");

    // 布尔值 truthy 测试
    let value = ConditionValue::Boolean(true);
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(result);

    let value = ConditionValue::Boolean(false);
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(!result);

    // 数字 truthy 测试
    let value = ConditionValue::Number(1.0);
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(result);

    let value = ConditionValue::Number(-1.0);
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(result);

    let value = ConditionValue::Number(0.0);
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(!result);

    // 字符串 truthy 测试
    let value = ConditionValue::String("hello".to_string());
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(result);

    let value = ConditionValue::String("".to_string());
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(!result);

    // Null truthy 测试
    let value = ConditionValue::Null;
    let result = value.is_truthy();
    // println!("{}: {:?} -> {}", "is_truthy", value, result);
    assert!(!result);

    // println!("✅ ConditionValue::is_truthy() 测试通过\n");
}

#[test]
fn test_condition_value_to_string() {
    // println!("=== 测试 ConditionValue::to_string: test_condition_value_to_string ===");

    // 不同类型转字符串测试
    let value = ConditionValue::Boolean(true);
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "true");

    let value = ConditionValue::Boolean(false);
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "false");

    let value = ConditionValue::Number(123.0);
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "123");

    let value = ConditionValue::Number(123.45);
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "123.45");

    let value = ConditionValue::String("hello".to_string());
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "hello");

    let value = ConditionValue::Null;
    let result = value.to_string();
    // println!("{}: {:?} -> {}", "to_string", value, result);
    assert_eq!(result, "");

    // println!("✅ ConditionValue::to_string() 测试通过\n");
}

#[test]
fn test_condition_expression_construction() {
    // println!("=== 测试 ConditionExpression 构造: test_condition_expression_construction ===");

    // 测试变量表达式构造
    let var_expr = ConditionExpression::Variable { name: "TEST_VAR".to_string() };
    // println!("构造变量表达式: {:?}", var_expr);
    match var_expr {
        ConditionExpression::Variable { name } => {
            // println!("✓ 变量名: {}", name);
            assert_eq!(name, "TEST_VAR");
        }
        _ => panic!("Expected Variable expression"),
    }

    // 测试字面量表达式构造
    let literal_expr = ConditionExpression::Literal { value: ConditionValue::Boolean(true) };
    // println!("构造字面量表达式: {:?}", literal_expr);
    match literal_expr {
        ConditionExpression::Literal { value } => {
            // println!("✓ 字面量值: {:?}", value);
            match value {
                ConditionValue::Boolean(b) => assert!(b),
                _ => panic!("Expected Boolean value"),
            }
        }
        _ => panic!("Expected Literal expression"),
    }

    // 测试比较表达式构造
    let comp_expr = ConditionExpression::Comparison {
        left: Box::new(ConditionExpression::Variable { name: "VAR1".to_string() }),
        operator: ComparisonOperator::Equal,
        right: Box::new(ConditionExpression::Literal {
            value: ConditionValue::String("test".to_string()),
        }),
    };
    // println!("构造比較表达式: VAR1 == \"test\"");

    match comp_expr {
        ConditionExpression::Comparison { operator, left, right } => {
            // println!("✓ 运算符: {:?}", operator);
            // println!("✓ 左操作数: {:?}", left);
            // println!("✓ 右操作数: {:?}", right);
            assert_eq!(operator, ComparisonOperator::Equal);
        }
        _ => panic!("Expected Comparison expression"),
    }

    // println!("✅ ConditionExpression 构造测试通过\n");
}

#[test]
fn test_comparison_operators() {
    // println!("=== 测试比較运算符: test_comparison_operators ===");

    let operators = vec![
        ("==", ComparisonOperator::Equal),
        ("!=", ComparisonOperator::NotEqual),
        (">", ComparisonOperator::GreaterThan),
        ("<", ComparisonOperator::LessThan),
        (">=", ComparisonOperator::GreaterThanOrEqual),
        ("<=", ComparisonOperator::LessThanOrEqual),
    ];

    // println!("支持的比较运算符:");
    for (symbol, operator) in &operators {
        // println!("  {} -> {:?}", symbol, operator);
    }

    // 确保所有运算符都可以正常实例化
    assert_eq!(operators.len(), 6);
    // println!("✅ 比较运算符测试通过 (共 {} 个)\n", operators.len());
}

#[test]
fn test_logical_operators() {
    // println!("=== 测试逻辑运算符: test_logical_operators ===");

    let operators = vec![
        ("&&", LogicalOperator::And),
        ("||", LogicalOperator::Or),
        ("!", LogicalOperator::Not),
    ];

    // println!("支持的逻辑运算符:");
    for (symbol, operator) in &operators {
        // println!("  {} -> {:?}", symbol, operator);
    }

    // 确保所有逻辑运算符都可以正常实例化
    assert_eq!(operators.len(), 3);
    // println!("✅ 逻辑运算符测试通过 (共 {} 个)\n", operators.len());
}

#[test]
fn test_complex_condition_expression() {
    // println!("=== 测试复杂条件表达式构造: test_complex_condition_expression ===");
    // println!("目标表达式: (VAR1 == \"test\" && VAR2 > 10) || !VAR3");

    // 构造复杂的条件表达式: (VAR1 == "test" && VAR2 > 10) || !VAR3
    let complex_expr = ConditionExpression::Logical {
        operator: LogicalOperator::Or,
        left: Some(Box::new(ConditionExpression::Logical {
            operator: LogicalOperator::And,
            left: Some(Box::new(ConditionExpression::Comparison {
                left: Box::new(ConditionExpression::Variable { name: "VAR1".to_string() }),
                operator: ComparisonOperator::Equal,
                right: Box::new(ConditionExpression::Literal {
                    value: ConditionValue::String("test".to_string()),
                }),
            })),
            right: Some(Box::new(ConditionExpression::Comparison {
                left: Box::new(ConditionExpression::Variable { name: "VAR2".to_string() }),
                operator: ComparisonOperator::GreaterThan,
                right: Box::new(ConditionExpression::Literal {
                    value: ConditionValue::Number(10.0),
                }),
            })),
        })),
        right: Some(Box::new(ConditionExpression::Logical {
            operator: LogicalOperator::Not,
            left: None,
            right: Some(Box::new(ConditionExpression::Variable { name: "VAR3".to_string() })),
        })),
    };

    // println!("构造的表达式结构:");
    // println!("  顶层: {:?}", LogicalOperator::Or);
    // println!("  左侧: (VAR1 == \"test\" && VAR2 > 10)");
    // println!("  右侧: !VAR3");

    // 验证表达式结构正确构建
    match complex_expr {
        ConditionExpression::Logical { operator, left, right } => {
            // println!("✓ 顶层运算符: {:?}", operator);
            // println!("✓ 左操作数: {:?}", left.is_some());
            // println!("✓ 右操作数: {:?}", right.is_some());
            assert_eq!(operator, LogicalOperator::Or);
        }
        _ => panic!("Expected top-level logical expression"),
    }

    // println!("✅ 复杂条件表达式构造测试通过\n");
}
