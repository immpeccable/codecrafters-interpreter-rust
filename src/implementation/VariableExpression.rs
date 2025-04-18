use crate::enums::LiteralValue::LiteralValue;
use crate::implementation::Token::Token;
use crate::traits::Expression::Expression;
use crate::traits::Interpreter::InterpreterTrait;

pub struct VariableExpression {
    pub variable: Token,
}

impl Expression for VariableExpression {
    fn expression_print(&self) -> String {
        return String::from("Visit variable expression");
    }

    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) -> Result<LiteralValue, String> {
        return interpreter.visit_variable_expression(&self);
    }
}
