use crate::{enums::LiteralValue::LiteralValue, implementation::Interpreter::Interpreter};

pub trait LoxCallableTrait {
    fn arity(&mut self) -> usize;
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>)
        -> LiteralValue;
}
