use core::panic;

use crate::{
    enums::{LiteralValue::LiteralValue, TokenType::TokenType},
    traits::{Expression::Expression, Interpreter::InterpreterTrait},
};

use super::{
    BinaryExpression::BinaryExpression, Grouping::Grouping, Literal::Literal,
    UnaryExpression::UnaryExpression,
};

pub struct Interpreter {}

impl InterpreterTrait for Interpreter {
    fn is_equal(&self, left: LiteralValue, right: LiteralValue) -> bool {
        return true;
    }

    fn is_truthy(&self, object: LiteralValue) -> bool {
        match object {
            LiteralValue::False => return false,
            LiteralValue::Nil => return false,
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
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(LiteralValue::Number((left_num - right_num).to_string()))
            }
            TokenType::PLUS => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(LiteralValue::Number((left_num + right_num).to_string()))
                } else {
                    Ok(LiteralValue::String(format!("{}{}", left_str, right_str)))
                }
            }
            TokenType::STAR => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(LiteralValue::Number((left_num * right_num).to_string()))
            }
            TokenType::SLASH => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(LiteralValue::Number((left_num / right_num).to_string()))
            }
            TokenType::GREATER => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(if left_num > right_num {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                })
            }
            TokenType::GREATER_EQUAL => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(if left_num >= right_num {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                })
            }
            TokenType::LESS => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(if left_num < right_num {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                })
            }
            TokenType::LESS_EQUAL => {
                let left_num = parse_f64(&left_str)?;
                let right_num = parse_f64(&right_str)?;
                Ok(if left_num <= right_num {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                })
            }
            TokenType::EQUAL_EQUAL => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(if left_num == right_num {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    })
                } else {
                    Ok(if left_str == right_str {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    })
                }
            }
            TokenType::BANG_EQUAL => {
                if let (Ok(left_num), Ok(right_num)) = (parse_f64(&left_str), parse_f64(&right_str))
                {
                    Ok(if left_num != right_num {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    })
                } else {
                    Ok(if left_str != right_str {
                        LiteralValue::True
                    } else {
                        LiteralValue::False
                    })
                }
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
                _ => panic!("Invalid operator for minus"),
            },
            TokenType::BANG => {
                let is_truthy = self.is_truthy(right);
                if is_truthy {
                    return Ok(LiteralValue::False);
                } else {
                    return Ok(LiteralValue::True);
                }
            }
            op => panic!("Unexpected unary operator: {:?}", op),
        }
    }
}
