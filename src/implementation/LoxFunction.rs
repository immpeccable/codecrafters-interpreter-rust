use std::collections::HashMap;

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Interpreter::InterpreterTrait, LoxCallableTrait::LoxCallableTrait},
};

use super::{
    BlockStatement::BlockStatement, Environment::Environment, FunctionStatement::FunctionStatement,
    Interpreter::Interpreter,
};

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
        let mut environment = Environment {
            enclosing: Some(Box::new(interpreter.environment.clone())),
            values: HashMap::new(),
        };
        for (index, _) in self.declaration.parameters.iter().enumerate() {
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
        let mut sts = Vec::new();
        for st in &self.declaration.body {
            sts.push(st.clone_box());
        }
        match interpreter.execute_block(&mut sts, environment).unwrap() {
            Some(v) => return v,
            None => return LiteralValue::Nil,
        }
    }
}
