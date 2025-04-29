use crate::{
    enums::LiteralValue::LiteralValue,
    implementation::{Interpreter::Interpreter, LoxInstance::LoxInstance},
};

pub trait LoxCallableTrait {
    fn arity(&mut self) -> usize;
    fn call(&mut self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>)
        -> LiteralValue;
}
