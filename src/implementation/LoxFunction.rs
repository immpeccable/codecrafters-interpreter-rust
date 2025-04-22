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
        self.declaration.parameters.len()
    }

    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> LiteralValue {
        // 1) Take ownership of the old env, leaving an empty one in its place.
        //    This avoids ever having to clone the old env.
        let old_env = std::mem::replace(&mut interpreter.environment, Environment::default());

        // 2) Set up the function's own local scope, chaining to the old one.
        interpreter.environment.enclosing = Some(Box::new(old_env));

        // 3) Bind parameters into this fresh, now‐active scope.
        for (param, arg) in self.declaration.parameters.iter().zip(arguments) {
            interpreter
                .environment
                .define(param.token_value.clone(), arg);
        }

        // 4) Build and execute the body as a BlockStatement.
        //    The visit_block_statement method will *not* push another scope—
        //    it will simply run in the current one, so we get exactly one scope push here.
        let sts = self.declaration.body.iter().map(|s| s.clone_box());
        let result = interpreter
            .execute_block(&mut sts.collect())
            .expect("function body must execute");

        // 5) Pop back to the old environment.
        //    `enclosing.take()` gives us the Box<old_env> we put there,
        //    and we move it back into `interpreter.environment`.
        let parent = interpreter.environment.enclosing.take().unwrap();
        interpreter.environment = *parent;

        // 6) Return the function’s return value (or Nil by default).
        result.unwrap_or(LiteralValue::Nil)
    }
}
