use std::any::Any;

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
}

impl Clone for PrintStatement {
    fn clone(&self) -> Self {
        PrintStatement {
            expression: self.expression.clone_box(),
        }
    }
}

impl Statement for PrintStatement {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_print_statement(self);
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_print_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
