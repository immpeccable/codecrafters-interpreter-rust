use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Interpreter::InterpreterTrait, LoxCallableTrait::LoxCallableTrait},
};

use super::{FunctionStatement::FunctionStatement, Interpreter::Interpreter};

pub struct LoxFunction {
    pub declaration: FunctionStatement,
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        LoxFunction {
            declaration: self.declaration.clone(),
        }
    }
}

impl LoxCallableTrait for LoxFunction {
    fn arity(&mut self) -> usize {
        return self.declaration.parameters.len();
    }
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> LiteralValue {
        let environment = &mut interpreter.environment;
        for (index, item) in self.declaration.parameters.iter().enumerate() {
            environment.define(
                self.declaration
                    .parameters
                    .get(index)
                    .unwrap()
                    .token_value
                    .clone(),
                arguments.get(index).unwrap().clone(),
            );
        }
        interpreter.interpret(&mut self.declaration.body);
        return LiteralValue::Nil;
    }
}
