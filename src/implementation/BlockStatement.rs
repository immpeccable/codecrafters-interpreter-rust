use std::any::Any;

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Clone for BlockStatement {
    fn clone(&self) -> Self {
        BlockStatement {
            statements: self.statements.iter().map(|s| s.clone_box()).collect(),
        }
    }
}

impl Statement for BlockStatement {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_block_statement(self);
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_block_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
