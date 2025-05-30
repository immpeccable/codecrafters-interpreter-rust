use std::{
    collections::HashMap,
    io::{self, Write},
    process::exit,
};

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::{
    AssignmentExpression::AssignmentExpression, BinaryExpression::BinaryExpression,
    BlockStatement::BlockStatement, CallExpression::CallExpression, ClassStatement::ClassStatement,
    ExpressionStatement::ExpressionStatement, FunctionStatement::FunctionStatement,
    GetExpression::GetExpression, Grouping::Grouping, IfStatement::IfStatement, Literal::Literal,
    LogicalExpression::LogicalExpression, PrintStatement::PrintStatement,
    ReturnStatement::ReturnStatement, SetExpression::SetExpression,
    SuperExpression::SuperExpression, ThisExpression::ThisExpression, Token::Token,
    UnaryExpression::UnaryExpression, VariableExpression::VariableExpression,
    VariableStatement::VariableStatement, WhileStatement::WhileStatement,
};

#[derive(Clone, PartialEq)]
pub enum FunctionType {
    NONE,
    FUNCTION,
    METHOD,
    INITIALIZER,
}

#[derive(Clone, PartialEq)]
pub enum ClassType {
    NONE,
    CLASS,
    SUBCLASS,
}

pub struct Resolver {
    pub interpreter: Box<dyn InterpreterTrait>,
    pub scopes: Vec<HashMap<String, bool>>,
    pub current_function: FunctionType,
    pub current_class: ClassType,
}

impl Resolver {
    pub fn visit_block_statement(&mut self, statement: &mut BlockStatement) {
        self.begin_scope();
        self.resolve_statements(&mut statement.statements);
        self.end_scope();
    }

    pub fn visit_variable_statement(&mut self, statement: &mut VariableStatement) {
        self.declare(&statement.name);
        statement.initializer.resolve(self);
        self.define(&statement.name);
    }

    pub fn visit_variable_expression(&mut self, expression: &mut VariableExpression) {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(v) = scope.get(&expression.variable.token_value) {
                if *v == false && self.scopes.len() > 1 {
                    self.error(
                        String::from("Can't read local variable in its own initializer."),
                        &expression.variable,
                    );
                }
            }
        }

        let variable_token = expression.variable.clone();

        self.resolve_local(expression, variable_token);
    }

    pub fn visit_assignment_expression(&mut self, expression: &mut AssignmentExpression) {
        expression.value.resolve(self);
        let name = expression.name.clone();
        self.resolve_local(expression, name);
    }

    pub fn visit_get_expression(&mut self, expression: &mut GetExpression) {
        expression.expression.resolve(self);
    }

    pub fn visit_set_expression(&mut self, expression: &mut SetExpression) {
        expression.expression.resolve(self);
        expression.value.resolve(self);
    }

    pub fn visit_function_statement(&mut self, statement: &mut FunctionStatement) {
        self.declare(&statement.name);
        self.define(&statement.name);
        self.resolve_function(statement, FunctionType::FUNCTION);
    }

    pub fn visit_expression_statement(&mut self, statement: &mut ExpressionStatement) {
        statement.expression.resolve(self);
    }

    pub fn visit_if_statement(&mut self, statement: &mut IfStatement) {
        statement.condition.resolve(self);
        statement.then_statement.resolve(self);
        if let Some(else_st) = &mut statement.else_statement {
            else_st.resolve(self);
        }
    }

    pub fn visit_print_statement(&mut self, statement: &mut PrintStatement) {
        statement.expression.resolve(self);
    }

    pub fn visit_return_statement(&mut self, statement: &mut ReturnStatement) {
        if self.current_function == FunctionType::NONE {
            self.error(
                String::from("Can't return from top-level code."),
                &statement.keyword,
            )
        }
        if let Some(v) = &mut statement.value {
            if self.current_function == FunctionType::INITIALIZER {
                self.error(
                    String::from("Can't return a value from an initializer."),
                    &statement.keyword,
                );
            }
            v.resolve(self);
        }
    }

    pub fn visit_while_statement(&mut self, statement: &mut WhileStatement) {
        statement.body.resolve(self);
        statement.condition.resolve(self);
    }

    pub fn visit_binary_expression(&mut self, expression: &mut BinaryExpression) {
        expression.left.resolve(self);
        expression.right.resolve(self);
    }

    pub fn visit_call_expression(&mut self, expression: &mut CallExpression) {
        expression.callee.resolve(self);
        for expr in &mut expression.arguments {
            expr.resolve(self);
        }
    }

    pub fn visit_grouping_expression(&mut self, expression: &mut Grouping) {
        expression.expression.resolve(self);
    }

    pub fn visit_logical_expression(&mut self, expression: &mut LogicalExpression) {
        expression.left.resolve(self);
        expression.right.resolve(self);
    }

    pub fn visit_unary_expression(&mut self, expression: &mut UnaryExpression) {
        expression.expression.resolve(self);
    }

    pub fn visit_literal_expression(&mut self, _expression: &mut Literal) {}
    pub fn visit_super_expression(&mut self, expression: &mut SuperExpression) {
        if self.current_class == ClassType::NONE {
            self.error(
                String::from("Can't use 'super' outside of a class."),
                &expression.keyword,
            );
        } else if self.current_class != ClassType::SUBCLASS {
            self.error(
                String::from("Can't use 'super' in a class with no superclass."),
                &expression.keyword,
            );
        }
        let token = expression.keyword.clone();
        self.resolve_local(expression, token);
    }
    pub fn visit_class_statement(&mut self, statement: &mut ClassStatement) {
        let prev = self.current_class.clone();
        self.current_class = ClassType::CLASS;
        self.declare(&statement.name);
        self.define(&statement.name);

        if let Some(superclass) = &mut statement.super_class {
            if statement
                .name
                .token_value
                .eq(&superclass.variable.token_value)
            {
                self.error(
                    String::from("A class can't inherit from itself."),
                    &superclass.variable,
                );
            }
            self.visit_variable_expression(superclass);
        }
        if let Some(_) = &mut statement.super_class {
            self.begin_scope();
            self.current_class = ClassType::SUBCLASS;
            let last = self.scopes.last_mut().unwrap();
            last.insert("super".to_string(), true);
        }
        self.begin_scope();
        let last = self.scopes.last_mut().unwrap();
        last.insert(String::from("this"), true);
        for method in &mut statement.methods {
            if let Some(method_fn) = method.as_any_mut().downcast_mut::<FunctionStatement>() {
                let mut declaration = FunctionType::METHOD;
                if method_fn.name.token_value.eq("init") {
                    declaration = FunctionType::INITIALIZER;
                }

                self.resolve_function(method_fn, declaration);
            } else {
                unreachable!("ClassStatement.methods must all be functions");
            }
        }
        self.end_scope();
        if let Some(_) = &mut statement.super_class {
            self.end_scope();
        }
        self.current_class = prev;
    }

    pub fn visit_this_expression(&mut self, expression: &mut ThisExpression) {
        if self.current_class == ClassType::NONE {
            self.error(
                String::from("Can't use 'this' outside of a class."),
                &expression.value,
            );
        }
        let token = expression.value.clone();
        self.resolve_local(expression, token);
    }

    fn error(&self, message: String, token: &Token) {
        writeln!(io::stderr(), "{}", message).unwrap();
        writeln!(io::stderr(), "[line {}]", token.line).unwrap();
        exit(65);
    }

    fn resolve_function(&mut self, statement: &mut FunctionStatement, ft: FunctionType) {
        let enclosing_function_type = self.current_function.clone();
        self.current_function = ft;
        self.begin_scope();
        for prm in &statement.parameters {
            self.declare(prm);
            self.define(prm);
        }
        self.resolve_statements(&mut statement.body);
        self.end_scope();
        self.current_function = enclosing_function_type;
    }

    fn resolve_local(&mut self, expression: &mut dyn Expression, token: Token) {
        for (depth, scope) in self.scopes.iter_mut().rev().enumerate() {
            if scope.contains_key(&token.token_value) {
                return self.interpreter.resolve(expression, depth);
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        let scope_len = self.scopes.len();
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.token_value) && scope_len > 1 {
                self.error(
                    String::from("Already a variable with this name in this scope."),
                    name,
                );
            } else {
                scope.insert(name.token_value.clone(), false);
            }
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.token_value.clone(), true);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    fn end_scope(&mut self) {
        self.scopes.pop();
    }
    pub fn resolve_statements(&mut self, statements: &mut Vec<Box<dyn Statement>>) {
        for stm in statements {
            stm.resolve(self);
        }
    }
}
