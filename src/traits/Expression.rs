use crate::{enums::LiteralValue::LiteralValue, implementation::Resolver::Resolver};
use std::any::Any;

use crate::traits::Interpreter::InterpreterTrait;

pub trait Expression: Any {
    fn expression_print(&self) -> String;
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait)
        -> Result<LiteralValue, String>;
    fn resolve(&mut self, resolver: &mut Resolver);
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn Expression>;
}
