use core::panic;

use std::{
    collections::HashMap,
    io::{self, Write},
    process::exit,
};

use crate::{
    enums::{LiteralValue::LiteralValue, TokenType::TokenType},
    traits::{
        Expression::Expression, Interpreter::InterpreterTrait, LoxCallableTrait::LoxCallableTrait,
        Statement::Statement,
    },
};

use super::{
    AssignmentExpression::AssignmentExpression, BinaryExpression::BinaryExpression,
    BlockStatement::BlockStatement, CallExpression::CallExpression, Clock::Clock,
    Environment::Environment, ExpressionStatement::ExpressionStatement,
    FunctionStatement::FunctionStatement, Grouping::Grouping, IfStatement::IfStatement,
    Literal::Literal, LoxFunction::LoxFunction, PrintStatement::PrintStatement, Token::Token,
    UnaryExpression::UnaryExpression, VariableExpression::VariableExpression,
    VariableStatement::VariableStatement, WhileStatement::WhileStatement,
};

#[derive(Default)]
pub struct Interpreter {
    pub environment: Environment,
}

impl InterpreterTrait for Interpreter {
    fn define_globals(&mut self) {
        self.environment
            .define(String::from("clock"), LiteralValue::Clock(Clock {}));
    }

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
            _ => false,
        }
    }

    fn is_truthy(&self, object: &LiteralValue) -> bool {
        match object {
            LiteralValue::Nil => return false,
            LiteralValue::Boolean(val) => return *val,
            _ => return true,
        }
    }

    fn evaluate(&mut self, expression: &mut Box<dyn Expression>) -> Result<LiteralValue, String> {
        return expression.interpret(self);
    }

    fn execute(&mut self, statement: &mut Box<dyn Statement>) {
        statement.interpret(self);
    }

    fn visit_binary_expression(
        &mut self,
        expression: &mut BinaryExpression,
    ) -> Result<LiteralValue, String> {
        let left_val = self.evaluate(&mut expression.left)?;
        let right_val = self.evaluate(&mut expression.right)?;
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

    fn visit_grouping(&mut self, expression: &mut Grouping) -> Result<LiteralValue, String> {
        return self.evaluate(&mut expression.expression);
    }
    fn visit_literal(&self, expression: &Literal) -> Result<LiteralValue, String> {
        return Ok(expression.value.clone());
    }

    fn visit_unary_expression(
        &mut self,
        expression: &mut UnaryExpression,
    ) -> Result<LiteralValue, String> {
        let right = self.evaluate(&mut expression.expression).unwrap();
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
    fn visit_variable_expression(
        &mut self,
        expression: &VariableExpression,
    ) -> Result<LiteralValue, String> {
        match self
            .environment
            .get(expression.variable.token_value.clone())
        {
            Some(res) => Ok(res.clone()),
            None => Err(self.error(
                String::from(format!(
                    "Undefined variable {}.",
                    &expression.variable.token_value
                )),
                &expression.variable,
            )),
        }
    }

    fn visit_logical_expression(
        &mut self,
        expression: &mut super::LogicalExpression::LogicalExpression,
    ) -> Result<LiteralValue, String> {
        let left = self.evaluate(&mut expression.left)?;
        if expression.operator.token_type == TokenType::OR {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }
        return self.evaluate(&mut expression.right);
    }

    fn visit_assignment_expression(
        &mut self,
        expression: &mut AssignmentExpression,
    ) -> Result<LiteralValue, String> {
        let value = self.evaluate(&mut expression.value)?;
        self.environment
            .assign(expression.name.clone(), value.clone())?;
        return Ok(value.clone());
    }

    fn visit_call_expression(
        &mut self,
        expression: &mut CallExpression,
    ) -> Result<LiteralValue, String> {
        let callee = self.evaluate(&mut expression.callee)?;

        let mut arguments = Vec::new();

        for mut arg in &mut expression.arguments {
            arguments.push(self.evaluate(&mut arg)?);
        }

        match callee {
            LiteralValue::Function(mut fnc) => {
                if arguments.len() != fnc.arity() {
                    return Err(self.error(
                        format!(
                            "Expected {} arguments but got {}.",
                            fnc.arity(),
                            arguments.len()
                        ),
                        &expression.paren,
                    ));
                }
                return Ok(fnc.call(self, arguments));
            }
            _ => {
                return Err(self.error(
                    String::from("Can only call functions and classes."),
                    &expression.paren,
                ))
            }
        }
    }

    fn visit_expression_statement(&mut self, statement: &mut ExpressionStatement) {
        let _ = self.evaluate(&mut statement.expression);
    }
    fn visit_variable_statement(&mut self, statement: &mut VariableStatement) {
        let value = &self.evaluate(&mut statement.initializer);
        self.environment
            .define(statement.name.token_value.clone(), value.clone().unwrap());
    }
    fn visit_print_statement(&mut self, statement: &mut PrintStatement) {
        let res = self.evaluate(&mut statement.expression).unwrap();
        match res {
            LiteralValue::Number(n) => {
                println!("{}", n.parse::<f64>().unwrap());
            }
            _ => println!("{}", res.to_string()),
        }
    }

    fn visit_while_statement(&mut self, statement: &mut WhileStatement) -> Result<(), String> {
        let mut condition_evaluation = self.evaluate(&mut statement.condition)?;
        while self.is_truthy(&condition_evaluation) {
            self.execute(&mut statement.body);
            condition_evaluation = self.evaluate(&mut statement.condition)?;
        }
        return Ok(());
    }

    fn visit_function_statement(
        &mut self,
        statement: &mut FunctionStatement,
    ) -> Result<(), String> {
        let name = statement.name.token_value.clone();
        let fnc = LoxFunction {
            declaration: statement.clone(),
        };
        self.environment.define(name, LiteralValue::Function(fnc));
        return Ok(());
    }

    fn visit_block_statement(&mut self, statement: &mut BlockStatement) {
        // swap out the old env, leaving an empty one in its place
        let old = std::mem::replace(
            &mut self.environment,
            Environment {
                values: HashMap::new(),
                enclosing: None,
            },
        );

        // now create the child, owning the old as its parent
        self.environment.enclosing = Some(Box::new(old));

        // run the block in that new scope
        self.interpret(&mut statement.statements);

        // when we're done, pull the parent back out and restore it
        if let Some(parent_box) = self.environment.enclosing.take() {
            self.environment = *parent_box;
        }
    }

    fn visit_if_statement(&mut self, statement: &mut IfStatement) -> Result<(), String> {
        let condition_evaluate_result = self.evaluate(&mut statement.condition)?;
        if self.is_truthy(&condition_evaluate_result) {
            self.execute(&mut statement.then_statement);
        } else {
            match &mut statement.else_statement {
                Some(else_st) => self.execute(else_st),
                None => {}
            };
        }
        return Ok(());
    }

    fn interpret(&mut self, statements: &mut Vec<Box<dyn Statement>>) {
        for mut statement in statements {
            self.execute(&mut statement);
        }
    }
}
