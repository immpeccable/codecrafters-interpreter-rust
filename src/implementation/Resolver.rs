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
    Grouping::Grouping, IfStatement::IfStatement, Literal::Literal,
    LogicalExpression::LogicalExpression, PrintStatement::PrintStatement,
    ReturnStatement::ReturnStatement, Token::Token, UnaryExpression::UnaryExpression,
    VariableExpression::VariableExpression, VariableStatement::VariableStatement,
    WhileStatement::WhileStatement,
};

#[derive(Clone, PartialEq)]
pub enum FunctionType {
    NONE,
    FUNCTION,
}

pub struct Resolver {
    pub interpreter: Box<dyn InterpreterTrait>,
    pub scopes: Vec<HashMap<String, bool>>,
    pub current_function: FunctionType,
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
        statement.value.resolve(self);
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
    pub fn visit_class_statement(&mut self, statement: &mut ClassStatement) {
        self.declare(&statement.name);
        self.define(&statement.name);
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
