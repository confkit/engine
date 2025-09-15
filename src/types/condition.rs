//! Author: xiaoYown  
//! Created: 2025-09-13
//! Description: Condition execution types

use serde::{Deserialize, Serialize};
use std::fmt;

/// 条件表达式枚举，表示不同类型的条件
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionExpression {
    /// 变量引用：${VAR_NAME}
    Variable { name: String },

    /// 字面量值：字符串、数字、布尔值
    Literal { value: ConditionValue },

    /// 比较表达式：left operator right
    Comparison {
        left: Box<ConditionExpression>,
        operator: ComparisonOperator,
        right: Box<ConditionExpression>,
    },

    /// 逻辑表达式：left operator right 或 operator operand
    Logical {
        operator: LogicalOperator,
        left: Option<Box<ConditionExpression>>,
        right: Option<Box<ConditionExpression>>,
    },
}

/// 条件值类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionValue {
    /// 字符串值
    String(String),
    /// 数字值
    Number(f64),
    /// 布尔值
    Boolean(bool),
    /// 空值
    Null,
}

/// 比较运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// 等于 ==
    Equal,
    /// 不等于 !=
    NotEqual,
    /// 大于 >
    GreaterThan,
    /// 小于 <
    LessThan,
    /// 大于等于 >=
    GreaterThanOrEqual,
    /// 小于等于 <=
    LessThanOrEqual,
}

/// 逻辑运算符
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalOperator {
    /// 逻辑与 &&
    And,
    /// 逻辑或 ||
    Or,
    /// 逻辑非 !
    Not,
}

/// 降级策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FallbackStrategy {
    /// 无条件执行（忽略条件）
    ExecuteUnconditionally,
    /// 跳过执行
    SkipExecution,
    /// 使用默认条件值
    UseDefaultValue(bool),
    /// 抛出错误
    ThrowError,
}

impl ConditionValue {
    /// 从字符串创建条件值，尝试解析为数字或布尔值
    pub fn from_string(s: &str) -> Self {
        // 尝试解析为布尔值
        match s.to_lowercase().as_str() {
            "true" => return ConditionValue::Boolean(true),
            "false" => return ConditionValue::Boolean(false),
            "null" | "" => return ConditionValue::Null,
            _ => {}
        }

        // 尝试解析为数字
        if let Ok(num) = s.parse::<f64>() {
            return ConditionValue::Number(num);
        }

        // 默认为字符串
        ConditionValue::String(s.to_string())
    }

    /// 检查值是否为真值（用于逻辑运算）
    pub fn is_truthy(&self) -> bool {
        match self {
            ConditionValue::Boolean(b) => *b,
            ConditionValue::Number(n) => *n != 0.0,
            ConditionValue::String(s) => !s.is_empty(),
            ConditionValue::Null => false,
        }
    }
}

impl std::fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComparisonOperator::Equal => write!(f, "=="),
            ComparisonOperator::NotEqual => write!(f, "!="),
            ComparisonOperator::GreaterThan => write!(f, ">"),
            ComparisonOperator::LessThan => write!(f, "<"),
            ComparisonOperator::GreaterThanOrEqual => write!(f, ">="),
            ComparisonOperator::LessThanOrEqual => write!(f, "<="),
        }
    }
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalOperator::And => write!(f, "&&"),
            LogicalOperator::Or => write!(f, "||"),
            LogicalOperator::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for ConditionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionValue::String(s) => write!(f, "{}", s),
            ConditionValue::Number(n) => write!(f, "{}", n),
            ConditionValue::Boolean(b) => write!(f, "{}", b),
            ConditionValue::Null => write!(f, ""),
        }
    }
}
