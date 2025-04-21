use crate::traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement};

pub struct ForStatement {
    pub initializer: Option<Box<dyn Statement>>,
    pub condition: Option<Box<dyn Expression>>,
    pub increment: Option<Box<dyn Expression>>,
    pub body: Box<dyn Statement>,
}

impl Statement for ForStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_for_statement(self);
    }
}
