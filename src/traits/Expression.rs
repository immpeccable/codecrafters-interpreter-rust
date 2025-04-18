use crate::enums::LiteralValue::LiteralValue;
use crate::traits::Interpreter::InterpreterTrait;

pub trait Expression {
    fn expression_print(&self) -> String;
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) -> Result<LiteralValue, String>;
}
