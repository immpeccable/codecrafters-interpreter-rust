use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct PrintStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for PrintStatement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) {
        interpreter.visit_print_statement(self);
    }
}
