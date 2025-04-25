use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::Token::Token;

pub struct VariableStatement {
    pub initializer: Box<dyn Expression>,
    pub name: Token,
}

impl Clone for VariableStatement {
    fn clone(&self) -> Self {
        VariableStatement {
            initializer: self.initializer.clone_box(),
            name: self.name.clone(),
        }
    }
}

impl Statement for VariableStatement {
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        let _ = interpreter.evaluate(&mut self.initializer);
        return interpreter.visit_variable_statement(self);
    }

    fn resolve(&mut self, resolver: &mut super::Resolver::Resolver) {
        resolver.visit_variable_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
