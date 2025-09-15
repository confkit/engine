//! 条件表达式求值引擎
//!
//! 提供条件表达式的运行时求值能力，支持环境变量解析、类型转换和逻辑运算

use anyhow::{anyhow, Result};
use std::collections::HashMap;

use super::parser::parse_condition;
use super::{ComparisonOperator, ConditionExpression, ConditionValue, LogicalOperator};

/// 条件求值引擎
#[derive(Debug)]
pub struct ConditionEvaluator {
    /// 环境变量映射
    environment: HashMap<String, String>,
    /// 最大递归深度
    max_recursion_depth: usize,
    /// 未定义变量的默认值
    default_for_undefined_var: ConditionValue,
}

/// 求值错误类型
#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("递归深度超限")]
    RecursionLimit,
    #[error("不支持的操作: {0}")]
    UnsupportedOperation(String),
}

impl ConditionEvaluator {
    /// 创建新的求值器实例
    pub fn new(environment: HashMap<String, String>) -> Self {
        Self {
            environment,
            max_recursion_depth: 100,
            default_for_undefined_var: ConditionValue::Null,
        }
    }

    /// 求值条件表达式，返回布尔结果
    pub fn evaluate(&self, expr: &ConditionExpression) -> Result<bool> {
        let value = self.evaluate_to_value(expr, 0)?;
        Ok(value.is_truthy())
    }

    /// 从字符串表达式求值
    pub fn evaluate_string(&self, expr_str: &str) -> Result<bool> {
        match parse_condition(expr_str) {
            Ok(ast) => self.evaluate(&ast),
            Err(parse_error) => self.handle_parse_error(expr_str, parse_error),
        }
    }

    /// 处理解析错误的降级策略  
    fn handle_parse_error(&self, expr_str: &str, error: super::parser::ParseError) -> Result<bool> {
        // 默认策略：无条件执行（保守策略）
        tracing::warn!("条件表达式解析失败，采用无条件执行策略: {} - {}", expr_str, error);
        Ok(true)
    }

    /// 求值表达式为条件值
    fn evaluate_to_value(
        &self,
        expr: &ConditionExpression,
        depth: usize,
    ) -> Result<ConditionValue> {
        // 防止递归栈溢出
        if depth > self.max_recursion_depth {
            return Err(anyhow!(EvaluationError::RecursionLimit));
        }

        match expr {
            ConditionExpression::Variable { name } => self.evaluate_variable(name),
            ConditionExpression::Literal { value } => Ok(value.clone()),
            ConditionExpression::Comparison { left, operator, right } => {
                self.evaluate_comparison(left, operator, right, depth + 1)
            }
            ConditionExpression::Logical { operator, left, right } => self.evaluate_logical(
                operator,
                left.as_ref().map(|v| &**v),
                right.as_ref().map(|v| &**v),
                depth + 1,
            ),
        }
    }

    /// 求值变量表达式
    fn evaluate_variable(&self, name: &str) -> Result<ConditionValue> {
        match self.get_variable_value(name) {
            Some(value) => {
                let condition_value = ConditionValue::from_string(&value);
                Ok(condition_value)
            }
            None => Ok(self.default_for_undefined_var.clone()),
        }
    }

    /// 从环境变量获取变量值
    fn get_variable_value(&self, name: &str) -> Option<String> {
        self.environment.get(name).cloned()
    }

    /// 求值比较表达式
    fn evaluate_comparison(
        &self,
        left: &ConditionExpression,
        operator: &ComparisonOperator,
        right: &ConditionExpression,
        depth: usize,
    ) -> Result<ConditionValue> {
        let left_val = self.evaluate_to_value(left, depth)?;
        let right_val = self.evaluate_to_value(right, depth)?;

        let result = self.compare_values(&left_val, &right_val, operator)?;

        Ok(ConditionValue::Boolean(result))
    }

    /// 求值逻辑表达式
    fn evaluate_logical(
        &self,
        operator: &LogicalOperator,
        left: Option<&ConditionExpression>,
        right: Option<&ConditionExpression>,
        depth: usize,
    ) -> Result<ConditionValue> {
        match operator {
            LogicalOperator::And => {
                if let (Some(left_expr), Some(right_expr)) = (left, right) {
                    tracing::trace!("└─ Evaluating logical AND");

                    let left_val = self.evaluate_to_value(left_expr, depth)?;
                    tracing::trace!(
                        "└─ Left operand: {:?} (truthy: {})",
                        left_val,
                        left_val.is_truthy()
                    );

                    if !left_val.is_truthy() {
                        // 短路求值: 左操作数为假，直接返回false
                        tracing::trace!("└─ Short-circuit: left is false, returning false");
                        return Ok(ConditionValue::Boolean(false));
                    }

                    let right_val = self.evaluate_to_value(right_expr, depth)?;
                    let result = right_val.is_truthy();
                    tracing::trace!("└─ Right operand: {:?} (truthy: {})", right_val, result);
                    tracing::trace!("└─ AND result: {}", result);

                    Ok(ConditionValue::Boolean(result))
                } else {
                    Err(anyhow!(EvaluationError::UnsupportedOperation(
                        "逻辑与运算需要两个操作数".to_string()
                    )))
                }
            }
            LogicalOperator::Or => {
                if let (Some(left_expr), Some(right_expr)) = (left, right) {
                    tracing::trace!("└─ Evaluating logical OR");

                    let left_val = self.evaluate_to_value(left_expr, depth)?;
                    tracing::trace!(
                        "└─ Left operand: {:?} (truthy: {})",
                        left_val,
                        left_val.is_truthy()
                    );

                    if left_val.is_truthy() {
                        // 短路求值: 左操作数为真，直接返回true
                        tracing::trace!("└─ Short-circuit: left is true, returning true");
                        return Ok(ConditionValue::Boolean(true));
                    }

                    let right_val = self.evaluate_to_value(right_expr, depth)?;
                    let result = right_val.is_truthy();
                    tracing::trace!("└─ Right operand: {:?} (truthy: {})", right_val, result);
                    tracing::trace!("└─ OR result: {}", result);

                    Ok(ConditionValue::Boolean(result))
                } else {
                    Err(anyhow!(EvaluationError::UnsupportedOperation(
                        "逻辑或运算需要两个操作数".to_string()
                    )))
                }
            }
            LogicalOperator::Not => {
                if let Some(operand) = right {
                    tracing::trace!("└─ Evaluating logical NOT");

                    let val = self.evaluate_to_value(operand, depth)?;
                    let result = !val.is_truthy();

                    tracing::trace!("└─ Operand: {:?} (truthy: {})", val, val.is_truthy());
                    tracing::trace!("└─ NOT result: {}", result);

                    Ok(ConditionValue::Boolean(result))
                } else {
                    Err(anyhow!(EvaluationError::UnsupportedOperation(
                        "逻辑非运算需要一个操作数".to_string()
                    )))
                }
            }
        }
    }

    /// 比较两个条件值
    fn compare_values(
        &self,
        left: &ConditionValue,
        right: &ConditionValue,
        operator: &ComparisonOperator,
    ) -> Result<bool> {
        match (left, right) {
            // 同类型比较
            (ConditionValue::String(l), ConditionValue::String(r)) => {
                Ok(self.compare_strings(l, r, operator))
            }
            (ConditionValue::Number(l), ConditionValue::Number(r)) => {
                Ok(self.compare_numbers(*l, *r, operator))
            }
            (ConditionValue::Boolean(l), ConditionValue::Boolean(r)) => {
                Ok(self.compare_booleans(*l, *r, operator))
            }

            // Null值特殊处理
            (ConditionValue::Null, ConditionValue::Null) => {
                Ok(matches!(operator, ComparisonOperator::Equal))
            }
            (ConditionValue::Null, _) | (_, ConditionValue::Null) => {
                Ok(matches!(operator, ComparisonOperator::NotEqual))
            }

            // 智能跨类型比较
            (l, r) => {
                // 尝试数字比较
                if let (Some(ln), Some(rn)) = (self.try_as_number(l), self.try_as_number(r)) {
                    Ok(self.compare_numbers(ln, rn, operator))
                } else {
                    // 回退到字符串比较
                    Ok(self.compare_strings(&l.to_string(), &r.to_string(), operator))
                }
            }
        }
    }

    /// 字符串比较
    fn compare_strings(&self, left: &str, right: &str, operator: &ComparisonOperator) -> bool {
        match operator {
            ComparisonOperator::Equal => left == right,
            ComparisonOperator::NotEqual => left != right,
            ComparisonOperator::GreaterThan => left > right,
            ComparisonOperator::LessThan => left < right,
            ComparisonOperator::GreaterThanOrEqual => left >= right,
            ComparisonOperator::LessThanOrEqual => left <= right,
        }
    }

    /// 数字比较
    fn compare_numbers(&self, left: f64, right: f64, operator: &ComparisonOperator) -> bool {
        match operator {
            ComparisonOperator::Equal => (left - right).abs() < f64::EPSILON,
            ComparisonOperator::NotEqual => (left - right).abs() >= f64::EPSILON,
            ComparisonOperator::GreaterThan => left > right,
            ComparisonOperator::LessThan => left < right,
            ComparisonOperator::GreaterThanOrEqual => left >= right,
            ComparisonOperator::LessThanOrEqual => left <= right,
        }
    }

    /// 布尔值比较
    fn compare_booleans(&self, left: bool, right: bool, operator: &ComparisonOperator) -> bool {
        match operator {
            ComparisonOperator::Equal => left == right,
            ComparisonOperator::NotEqual => left != right,
            // 布尔值的大小比较: false < true
            ComparisonOperator::GreaterThan => left && !right,
            ComparisonOperator::LessThan => !left && right,
            ComparisonOperator::GreaterThanOrEqual => left >= right,
            ComparisonOperator::LessThanOrEqual => left <= right,
        }
    }

    /// 尝试将条件值转换为数字
    fn try_as_number(&self, value: &ConditionValue) -> Option<f64> {
        match value {
            ConditionValue::Number(n) => Some(*n),
            ConditionValue::String(s) => s.parse().ok(),
            ConditionValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            ConditionValue::Null => Some(0.0),
        }
    }
}
