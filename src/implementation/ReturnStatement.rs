use std::any::Any;

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::Token::Token;

pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Option<Box<dyn Expression>>,
}

impl Statement for ReturnStatement {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn clone_box(&self) -> Box<dyn Statement> {
        match &self.value {
            Some(v) => Box::new(ReturnStatement {
                keyword: self.keyword.clone(),
                value: Some(v.clone_box()),
            }),
            None => Box::new(ReturnStatement {
                keyword: self.keyword.clone(),
                value: None,
            }),
        }
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_return_statement(self);
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_return_statement(self);
    }
}
