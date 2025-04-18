use crate::{
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_expression_statement(self);
    }
}
