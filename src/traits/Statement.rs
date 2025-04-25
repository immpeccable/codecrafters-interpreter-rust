use crate::{enums::LiteralValue::LiteralValue, implementation::Resolver::Resolver};

use super::Interpreter::InterpreterTrait;

pub trait Statement {
    fn interpret(
        &mut self,
        interpreter: &mut dyn InterpreterTrait,
    ) -> Result<Option<LiteralValue>, String>;
    fn clone_box(&self) -> Box<dyn Statement>;
    fn resolve(&mut self, resolver: &mut Resolver);
}
