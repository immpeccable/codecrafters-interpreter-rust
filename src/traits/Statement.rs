use super::Interpreter::InterpreterTrait;

pub trait Statement {
    fn interpret(&mut self, interpreter: &mut dyn InterpreterTrait);
    fn clone_box(&self) -> Box<dyn Statement>;
}
