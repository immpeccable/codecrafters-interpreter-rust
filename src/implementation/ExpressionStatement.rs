use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Expression::Expression, Interpreter::InterpreterTrait, Statement::Statement},
};

pub struct ExpressionStatement {
    pub expression: Box<dyn Expression>,
}

impl Clone for ExpressionStatement {
    fn clone(&self) -> Self {
        ExpressionStatement {
            expression: self.expression.clone_box(),
        }
    }
}

impl Statement for ExpressionStatement {
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String> {
        return interpreter.visit_expression_statement(self);
    }

    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}
