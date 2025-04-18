use crate::traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement};

use super::Token::Token;

pub struct VariableStatement {
    pub initializer: Box<dyn Expression>,
    pub name: Token,
}

impl Statement for VariableStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        let _ = interpreter.evaluate(&mut self.initializer);
        interpreter.visit_variable_statement(self);
    }
}
