use std::any::Any;

use crate::{enums::LiteralValue::LiteralValue, traits::Statement::Statement};

use super::{Token::Token, VariableExpression::VariableExpression};

pub struct ClassStatement {
    pub name: Token,
    pub methods: Vec<Box<dyn Statement>>,
    pub super_class: Option<VariableExpression>,
}

impl Statement for ClassStatement {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(ClassStatement {
            name: self.name.clone(),
            methods: self.methods.iter().map(|stmt| stmt.clone_box()).collect(),
            super_class: self.super_class.clone(),
        })
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn crate::traits::Interpreter::InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_class_statement(self);
    }
    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_class_statement(self);
    }
}
