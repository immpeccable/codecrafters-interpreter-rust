use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct BlockStatement {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Statement for BlockStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_block_statement(self);
    }
}
