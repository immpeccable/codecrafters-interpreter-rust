use std::any::Any;

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_statement: Box<dyn Statement>,
    pub else_statement: Option<Box<dyn Statement>>,
}

impl Clone for IfStatement {
    fn clone(&self) -> Self {
        IfStatement {
            condition: self.condition.clone_box(),
            then_statement: self.then_statement.clone_box(),
            else_statement: self.else_statement.as_ref().map(|stmt| stmt.clone_box()),
        }
    }
}

impl Statement for IfStatement {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_if_statement(self);
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_if_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
