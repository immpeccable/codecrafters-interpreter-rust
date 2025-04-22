use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

use super::Token::Token;

pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Box<dyn Statement>>,
}

impl Clone for FunctionStatement {
    fn clone(&self) -> Self {
        FunctionStatement {
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            body: self.body.iter().map(|stmt| stmt.clone_box()).collect(),
        }
    }
}

impl Statement for FunctionStatement {
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_function_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
