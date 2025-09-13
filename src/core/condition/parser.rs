//! 条件表达式解析器
//!
//! 基于 nom 解析器组合子实现条件表达式的词法和语法解析

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use super::{ComparisonOperator, ConditionExpression, ConditionValue, LogicalOperator};

/// 解析错误类型
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("语法错误: {0}")]
    SyntaxError(String),
}

/// 条件表达式解析器主入口
pub fn parse_condition(input: &str) -> Result<ConditionExpression, ParseError> {
    match condition_expression(input.trim()) {
        Ok((remaining, expr)) => {
            if !remaining.trim().is_empty() {
                Err(ParseError::SyntaxError(format!(
                    "表达式解析完成后发现意外的内容: '{}'",
                    remaining.trim()
                )))
            } else {
                Ok(expr)
            }
        }
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(ParseError::SyntaxError(
            format!("语法错误: 无法解析表达式 '{}' (在位置附近: '{}')", input, e.input),
        )),
        Err(nom::Err::Incomplete(_)) => {
            Err(ParseError::SyntaxError("表达式不完整，可能缺少结束符号".to_string()))
        }
    }
}

/// 解析完整的条件表达式
fn condition_expression(input: &str) -> IResult<&str, ConditionExpression> {
    logical_or_expression(input)
}

/// 解析逻辑或表达式 (优先级最低)
fn logical_or_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, first) = logical_and_expression(input)?;

    let (input, rest) = nom::multi::many0(preceded(
        delimited(multispace0, tag("||"), multispace0),
        logical_and_expression,
    ))(input)?;

    if rest.is_empty() {
        Ok((input, first))
    } else {
        let mut expr = first;
        for right in rest {
            expr = ConditionExpression::Logical {
                operator: LogicalOperator::Or,
                left: Some(Box::new(expr)),
                right: Some(Box::new(right)),
            };
        }
        Ok((input, expr))
    }
}

/// 解析逻辑与表达式
fn logical_and_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, first) = comparison_expression(input)?;

    let (input, rest) = nom::multi::many0(preceded(
        delimited(multispace0, tag("&&"), multispace0),
        comparison_expression,
    ))(input)?;

    if rest.is_empty() {
        Ok((input, first))
    } else {
        let mut expr = first;
        for right in rest {
            expr = ConditionExpression::Logical {
                operator: LogicalOperator::And,
                left: Some(Box::new(expr)),
                right: Some(Box::new(right)),
            };
        }
        Ok((input, expr))
    }
}

/// 解析比较表达式 (优先级最高)
fn comparison_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, left) = primary_expression(input)?;
    let (input, _) = multispace0(input)?;

    // 尝试解析比较运算符
    if let Ok((input, op)) = comparison_operator(input) {
        let (input, _) = multispace0(input)?;
        let (input, right) = primary_expression(input)?;

        let comparison_op = match op.as_str() {
            "==" => ComparisonOperator::Equal,
            "!=" => ComparisonOperator::NotEqual,
            ">" => ComparisonOperator::GreaterThan,
            "<" => ComparisonOperator::LessThan,
            ">=" => ComparisonOperator::GreaterThanOrEqual,
            "<=" => ComparisonOperator::LessThanOrEqual,
            _ => {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Alt,
                )))
            }
        };

        Ok((
            input,
            ConditionExpression::Comparison {
                left: Box::new(left),
                operator: comparison_op,
                right: Box::new(right),
            },
        ))
    } else {
        Ok((input, left))
    }
}

/// 解析主表达式 (变量、字面量、括号表达式、逻辑非)
fn primary_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, _) = multispace0(input)?;

    alt((
        // 逻辑非运算符 (优先级最高)
        unary_not_expression,
        // 原子表达式
        atom_expression,
    ))(input)
}

/// 解析逻辑非表达式
fn unary_not_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, _) = char('!')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = atom_expression(input)?;

    Ok((
        input,
        ConditionExpression::Logical {
            operator: LogicalOperator::Not,
            left: None,
            right: Some(Box::new(expr)),
        },
    ))
}

/// 解析原子表达式 (变量、字面量、括号表达式)
fn atom_expression(input: &str) -> IResult<&str, ConditionExpression> {
    let (input, _) = multispace0(input)?;

    alt((
        // 括号表达式
        delimited(char('('), delimited(multispace0, condition_expression, multispace0), char(')')),
        // 变量
        map(variable, |var| ConditionExpression::Variable { name: var }),
        // 字面量
        map(literal_value, |val| ConditionExpression::Literal { value: val }),
    ))(input)
}

/// 解析变量 ${VAR}
fn variable(input: &str) -> IResult<&str, String> {
    delimited(
        tag("${"),
        map(take_while1(|c: char| c.is_alphanumeric() || c == '_'), |s: &str| s.to_string()),
        char('}'),
    )(input)
}

/// 解析字面量值
fn literal_value(input: &str) -> IResult<&str, ConditionValue> {
    alt((
        map(boolean_literal, ConditionValue::Boolean),
        map(number_literal, ConditionValue::Number),
        map(string_literal, ConditionValue::String),
    ))(input)
}

/// 解析布尔字面量
fn boolean_literal(input: &str) -> IResult<&str, bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}

/// 解析数字字面量
fn number_literal(input: &str) -> IResult<&str, f64> {
    map(recognize(tuple((opt(char('-')), digit1, opt(tuple((char('.'), digit1)))))), |s: &str| {
        s.parse().unwrap_or(0.0)
    })(input)
}

/// 解析字符串字面量
fn string_literal(input: &str) -> IResult<&str, String> {
    alt((
        // 双引号字符串
        delimited(char('"'), map(take_while(|c| c != '"'), |s: &str| s.to_string()), char('"')),
        // 单引号字符串
        delimited(char('\''), map(take_while(|c| c != '\''), |s: &str| s.to_string()), char('\'')),
    ))(input)
}

/// 解析比较运算符
fn comparison_operator(input: &str) -> IResult<&str, String> {
    alt((
        map(tag("=="), |s: &str| s.to_string()),
        map(tag("!="), |s: &str| s.to_string()),
        map(tag(">="), |s: &str| s.to_string()),
        map(tag("<="), |s: &str| s.to_string()),
        map(tag(">"), |s: &str| s.to_string()),
        map(tag("<"), |s: &str| s.to_string()),
    ))(input)
}
