use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for LogicalExpression {
    fn expression_print(&self) -> String {
        return String::from("zz");
    }
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_logical_expression(self);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
