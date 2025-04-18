use super::Interpreter::InterpreterTrait;

pub trait Statement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait);
}
