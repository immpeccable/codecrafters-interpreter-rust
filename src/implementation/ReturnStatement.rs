use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::Token::Token;

pub struct ReturnStatement {
    pub keyword: Token,
    pub value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn clone_box(&self) -> Box<dyn Statement> {
        return Box::new(ReturnStatement {
            keyword: self.keyword.clone(),
            value: self.value.clone_box(),
        });
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
