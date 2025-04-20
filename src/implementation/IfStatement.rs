use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub then_statement: Box<dyn Statement>,
    pub else_statement: Option<Box<dyn Statement>>
}

impl Statement for IfStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_if_statement(self);
    }
}
