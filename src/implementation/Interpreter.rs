use core::panic;
use std::{
    io::{self, Write},
    process::exit,
};

use crate::{
    enums::{LiteralValue::LiteralValue, TokenType::TokenType},
    traits::{Expression::Expression, Interpreter::InterpreterTrait},
};

use super::{
    BinaryExpression::BinaryExpression, Grouping::Grouping, Literal::Literal, Token::Token,
    UnaryExpression::UnaryExpression,
};

pub struct Interpreter {}

impl InterpreterTrait for Interpreter {
    fn error(&self, message: String, token: &Token) -> String {
        writeln!(io::stderr(), "{}", message).unwrap();
        writeln!(io::stderr(), "[line {}]", token.line).unwrap();
        exit(70);
    }

    fn is_equal(&self, left: &LiteralValue, right: &LiteralValue) -> bool {
        match left {
            LiteralValue::Boolean(left_val) => match right {
                LiteralValue::Boolean(right_val) => return left_val == right_val,
                _ => return false,
            },
            LiteralValue::Nil => match right {
                LiteralValue::Nil => return true,
                _ => return false,
            },
            LiteralValue::Number(left_number) => match right {
                LiteralValue::Number(right_number) => return left_number == right_number,
                _ => return false,
            },
            LiteralValue::String(left_str) => match right {
                LiteralValue::String(right_str) => return left_str == right_str,
                _ => return false,
            },
        }
    }

    fn is_truthy(&self, object: &LiteralValue) -> bool {
        match object {
            LiteralValue::Nil => return false,
            LiteralValue::Boolean(val) => return *val,
            _ => return true,
        }
    }

    fn evaluate(&self, expression: &Box<dyn Expression>) -> Result<LiteralValue, String> {
        return expression.interpret();
    }
    fn visit_binary_expression(
        &self,
        expression: &BinaryExpression,
    ) -> Result<LiteralValue, String> {
        let left_val = self.evaluate(&expression.left)?;
        let right_val = self.evaluate(&expression.right)?;
        let left_str = left_val.to_string();
        let right_str = right_val.to_string();

        let parse_f64 = |s: &str| s.parse::<f64>().map_err(|e| e.to_string());

        match expression.operator.token_type {
            TokenType::MINUS => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Number((left_num - right_num).to_string()))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::PLUS => match (left_val, right_val) {
                (LiteralValue::String(left_str), LiteralValue::String(right_str)) => {
                    Ok(LiteralValue::String(format!("{}{}", left_str, right_str)))
                }
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    let left_f = parse_f64(&left_num).map_err(|_| {
                        self.error(
                            String::from("Operands must be numbers"),
                            &expression.operator,
                        )
                    })?;
                    let right_f = parse_f64(&right_num).map_err(|_| {
                        self.error(
                            String::from("Operands must be numbers"),
                            &expression.operator,
                        )
                    })?;
                    Ok(LiteralValue::Number((left_f + right_f).to_string()))
                }
                _ => Err(self.error(
                    "Operands must be two numbers or two strings".to_string(),
                    &expression.operator,
                )),
            },

            TokenType::STAR => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Number((left_num * right_num).to_string()))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::SLASH => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Number((left_num / right_num).to_string()))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::GREATER => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Boolean(left_num > right_num))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::GREATER_EQUAL => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Boolean(left_num >= right_num))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::LESS => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Boolean(left_num < right_num))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::LESS_EQUAL => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Boolean(left_num <= right_num))
                } else {
                    Err(self.error(
                        String::from("Operands must be numbers."),
                        &expression.operator,
                    ))
                }
            }
            TokenType::EQUAL_EQUAL => {
                Ok(LiteralValue::Boolean(self.is_equal(&left_val, &right_val)))
            }
            TokenType::BANG_EQUAL => {
                Ok(LiteralValue::Boolean(!self.is_equal(&left_val, &right_val)))
            }
            _ => Err("Expression operator not right".to_string()),
        }
    }

    fn visit_grouping(&self, expression: &Grouping) -> Result<LiteralValue, String> {
        return self.evaluate(&expression.expression);
    }
    fn visit_literal(&self, expression: &Literal) -> Result<LiteralValue, String> {
        return Ok(expression.value.clone());
    }

    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<LiteralValue, String> {
        let right = self.evaluate(&expression.expression).unwrap();
        match expression.operator.token_type {
            TokenType::MINUS => match right {
                LiteralValue::Number(number) => {
                    let right_number = number.parse::<f64>().unwrap();
                    return Ok(LiteralValue::Number((-1.0 * right_number).to_string()));
                }
                _ => Err(self.error(
                    String::from("Operand must be a number."),
                    &expression.operator,
                )),
            },
            TokenType::BANG => {
                return Ok(LiteralValue::Boolean(!self.is_truthy(&right)));
            }
            op => panic!("Unexpected unary operator: {:?}", op),
        }
    }
}
