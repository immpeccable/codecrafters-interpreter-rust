use crate::traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement};

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for WhileStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_while_statement(self);
    }
}
