use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::Interpreter::Interpreter;

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn statement_print(&self) -> String {
        String::from("value")
    }
    fn interpret(&self) {
        let interpreter = Interpreter {};
        interpreter.visit_expression_statement(&self);
    }
}
