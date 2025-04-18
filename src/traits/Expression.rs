use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;
use crate::traits::Interpreter::InterpreterTrait;

pub trait Expression: Any {
    fn expression_print(&self) -> String;
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait) -> Result<LiteralValue, String>;
    fn as_any(&self) -> &dyn Any;
}
