use core::panic;

use std::{
    cell::RefCell,
    collections::HashMap,
    f32::consts::E,
    hash::Hash,
    io::{self, Write},
    process::exit,
    rc::Rc,
};

use crate::implementation::Environment::EnvExt;

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
    FunctionStatement::FunctionStatement, GetExpression::GetExpression, Grouping::Grouping,
    IfStatement::IfStatement, Literal::Literal, LoxClass::LoxClass, LoxFunction::LoxFunction,
    PrintStatement::PrintStatement, ReturnStatement::ReturnStatement, Token::Token,
    UnaryExpression::UnaryExpression, VariableExpression::VariableExpression,
    VariableStatement::VariableStatement, WhileStatement::WhileStatement,
};

pub type SharedEnv = Rc<RefCell<Environment>>;

#[derive(Default)]
pub struct Interpreter {
    pub environment: SharedEnv,
    pub locals: HashMap<u32, usize>,
}

impl InterpreterTrait for Interpreter {
    fn define_globals(&mut self) {
        self.environment
            .borrow_mut()
            .define(String::from("clock"), LiteralValue::Clock(Clock {}));
    }

    fn resolve(&mut self, expression: &mut dyn Expression, depth: usize) {
        self.locals.insert(expression.id(), depth);
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

    fn execute(
        &mut self,
        statement: &mut Box<dyn Statement>,
    ) -> Result<Option<LiteralValue>, String> {
        return statement.interpret(self);
    }

    fn visit_class_statement(
        &mut self,
        statement: &mut super::ClassStatement::ClassStatement,
    ) -> Result<Option<LiteralValue>, String> {
        self.environment
            .borrow_mut()
            .define(statement.name.token_value.clone(), LiteralValue::Nil);

        let mut methods: HashMap<String, LoxFunction> = HashMap::new();
        for method in &mut statement.methods {
            if let Some(method_fn) = method.as_any_mut().downcast_mut::<FunctionStatement>() {
                let fnc = LoxFunction {
                    declaration: method_fn.clone(),
                    closure: self.environment.clone(),
                };
                methods.insert(method_fn.name.token_value.clone(), fnc);
            } else {
                unreachable!("ClassStatement.methods must all be functions");
            }
        }
        let klass = LoxClass {
            name: statement.name.token_value.clone(),
            methods,
        };
        self.environment
            .borrow_mut()
            .assign(statement.name.clone(), LiteralValue::LoxClass(klass))?;
        return Ok(None);
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

    fn visit_this_expression(
        &mut self,
        expression: &mut super::ThisExpression::ThisExpression,
    ) -> Result<LiteralValue, String> {
        self.look_up_variable(&expression.value, expression)
    }

    fn look_up_variable(
        &mut self,
        token: &Token,
        expr: &dyn Expression,
    ) -> Result<LiteralValue, String> {
        // See if we resolved it to some local depth
        if let Some(&distance) = self.locals.get(&expr.id()) {
            self.environment
                .get_at(distance, &token.token_value)
                .ok_or_else(|| {
                    self.error(format!("Undefined variable {}.", token.token_value), token)
                })
        } else {
            self.environment
                .borrow_mut()
                .get(&token.token_value)
                .ok_or_else(|| {
                    self.error(format!("Undefined variable {}.", token.token_value), token)
                })
        }
    }

    /// Visitor for your VariableExpression node
    fn visit_variable_expression(
        &mut self,
        expression: &VariableExpression,
    ) -> Result<LiteralValue, String> {
        self.look_up_variable(&expression.variable, expression)
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

        if let Some(&distance) = self.locals.get(&expression.id) {
            self.environment
                .assign_at(distance, expression.name.clone(), value.clone())?
        } else {
            self.environment
                .borrow_mut()
                .assign(expression.name.clone(), value.clone())?;
        }

        return Ok(value.clone());
    }

    fn visit_get_expression(
        &mut self,
        expression: &mut GetExpression,
    ) -> Result<LiteralValue, String> {
        let object = self.evaluate(&mut expression.expression)?;
        match object {
            LiteralValue::Instance(li) => match li.borrow_mut().get(expression.name.clone()) {
                Some(v) => {
                    return Ok(v);
                }
                None => Err(self.error(
                    String::from(format!(
                        "Undefined property '{}'.",
                        expression.name.token_value
                    )),
                    &expression.name,
                )),
            },
            _ => Err(self.error(
                String::from("Only instances have properties."),
                &expression.name,
            )),
        }
    }

    fn visit_set_expression(
        &mut self,
        expression: &mut super::SetExpression::SetExpression,
    ) -> Result<LiteralValue, String> {
        let object = self.evaluate(&mut expression.expression)?;
        match object {
            LiteralValue::Instance(li) => {
                let value = self.evaluate(&mut expression.value)?;
                li.borrow_mut().set(expression.name.clone(), value.clone());
                return Ok(value);
            }
            _ => Err(self.error(
                String::from("Only instances have properties."),
                &expression.name,
            )),
        }
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
                let res = fnc.call(self, arguments);
                return Ok(res);
            }
            LiteralValue::LoxClass(mut cl) => {
                if arguments.len() != cl.arity() {
                    return Err(self.error(
                        format!(
                            "Expected {} arguments but got {}.",
                            cl.arity(),
                            arguments.len()
                        ),
                        &expression.paren,
                    ));
                }
                let res = cl.call(self, arguments);
                return Ok(res);
            }
            LiteralValue::Clock(mut fnc) => {
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

    fn visit_expression_statement(
        &mut self,
        statement: &mut ExpressionStatement,
    ) -> Result<Option<LiteralValue>, String> {
        self.evaluate(&mut statement.expression)?;
        return Ok(None);
    }
    fn visit_variable_statement(
        &mut self,
        statement: &mut VariableStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let value = &self.evaluate(&mut statement.initializer);
        self.environment
            .borrow_mut()
            .define(statement.name.token_value.clone(), value.clone().unwrap());
        return Ok(None);
    }
    fn visit_print_statement(
        &mut self,
        statement: &mut PrintStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let res = self.evaluate(&mut statement.expression).unwrap();
        match res {
            LiteralValue::Number(n) => {
                println!("{}", n.parse::<f64>().unwrap());
            }
            _ => println!("{}", res.to_string()),
        }
        return Ok(None);
    }

    fn visit_while_statement(
        &mut self,
        statement: &mut WhileStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let mut condition_evaluation = self.evaluate(&mut statement.condition)?;
        while self.is_truthy(&condition_evaluation) {
            match self.execute(&mut statement.body)? {
                Some(v) => return Ok(Some(v)),
                None => {}
            };
            condition_evaluation = self.evaluate(&mut statement.condition)?;
        }
        return Ok(None);
    }

    fn visit_function_statement(
        &mut self,
        statement: &mut FunctionStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let name = statement.name.token_value.clone();
        let fnc = LoxFunction {
            declaration: statement.clone(),
            closure: Rc::clone(&self.environment),
        };
        self.environment
            .borrow_mut()
            .define(name, LiteralValue::Function(fnc));
        return Ok(None);
    }

    fn visit_return_statement(
        &mut self,
        statement: &mut ReturnStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let value = self.evaluate(&mut statement.value)?;
        return Ok(Some(value));
    }

    fn visit_block_statement(
        &mut self,
        statement: &mut BlockStatement,
    ) -> Result<Option<LiteralValue>, String> {
        // 1) build a brand‑new frame whose parent is the old one
        let parent = Rc::clone(&self.environment);
        let child = Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
        }));

        // 2) swap it in, saving the old pointer
        let old = std::mem::replace(&mut self.environment, child);

        // 3) run statements
        let mut result = None;
        for stmt in &mut statement.statements {
            if let Some(v) = self.execute(stmt)? {
                result = Some(v);
                break;
            }
        }

        // 4) restore the old pointer
        self.environment = old;
        Ok(result)
    }

    fn execute_block(
        &mut self,
        statements: &mut Vec<Box<dyn Statement>>,
    ) -> Result<Option<LiteralValue>, String> {
        let mut result = None;
        for stmt in statements {
            let res = self.execute(stmt)?;
            if res.is_some() {
                result = res;
                break;
            }
        }
        Ok(result)
    }

    fn visit_if_statement(
        &mut self,
        statement: &mut IfStatement,
    ) -> Result<Option<LiteralValue>, String> {
        let condition = self.evaluate(&mut statement.condition)?;
        if self.is_truthy(&condition) {
            // *Propagate* whatever the then‐branch returns (Some or None)
            return self.execute(&mut statement.then_statement);
        } else if let Some(else_branch) = &mut statement.else_statement {
            // Likewise for an else branch
            return self.execute(else_branch);
        }
        // No return executed, fall through
        Ok(None)
    }

    fn interpret(
        &mut self,
        statements: &mut Vec<Box<dyn Statement>>,
    ) -> Result<Option<LiteralValue>, String> {
        let mut ret_val = None;
        for statement in statements {
            match self.execute(statement)? {
                Some(rv) => {
                    ret_val = Some(rv);
                    break;
                }
                None => {}
            }
        }
        return Ok(ret_val);
    }
}
