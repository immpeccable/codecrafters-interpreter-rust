use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;
use std::any::Any;

pub struct AssignmentExpression {
    pub name: Token,
    pub value: Box<dyn Expression>,
}

impl Expression for AssignmentExpression {
    fn expression_print(&self) -> String {
        return String::from("Visit variable expression");
    }

    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<LiteralValue, String> {
        return interpreter.visit_assignment_expression(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
