use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Interpreter::InterpreterTrait, LoxCallableTrait::LoxCallableTrait},
};

use super::{
    BlockStatement::BlockStatement,
    Environment::{EnvExt, Environment},
    FunctionStatement::FunctionStatement,
    Interpreter::{Interpreter, SharedEnv},
    LoxInstance::LoxInstance,
};

pub struct LoxFunction {
    pub declaration: FunctionStatement,
    pub closure: SharedEnv,
    pub is_initializer: bool,
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: self.closure.clone(),
            is_initializer: self.is_initializer,
        }
    }
}

impl LoxFunction {
    pub fn bind(&self, instance: Rc<RefCell<LoxInstance>>) -> LoxFunction {
        // 1) Create a new child environment whose parent is our closure
        let env = {
            let mut map = HashMap::new();
            // Define "this" in that new scope
            map.insert(
                "this".to_string(),
                LiteralValue::Instance(Rc::clone(&instance)),
            );
            Rc::new(RefCell::new(Environment {
                values: map,
                enclosing: Some(Rc::clone(&self.closure)),
            }))
        };

        // 2) Return a fresh LoxFunction with the bound environment
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: env,
            is_initializer: self.is_initializer,
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
        let parent = Rc::clone(&self.closure);
        let child = Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
        }));

        // 2) Swap it into the interpreter, saving the old
        let old_env = std::mem::replace(&mut interpreter.environment, child);

        // 3) Bind parameters into _this_ frame
        {
            let mut env = interpreter.environment.borrow_mut();
            for (param, arg) in self.declaration.parameters.iter().zip(arguments) {
                env.define(param.token_value.clone(), arg);
            }
        }

        // 4) Execute the function body in that frame
        let mut body = self
            .declaration
            .body
            .iter()
            .map(|s| s.clone_box())
            .collect::<Vec<_>>();
        let result = interpreter
            .execute_block(&mut body)
            .expect("Function body must execute");

        interpreter.environment = old_env;

        if self.is_initializer {
            return self.closure.get_at(0, "this").unwrap();
        }

        // 6) Return the function’s return‐value or Nil
        result.unwrap_or(LiteralValue::Nil)
    }
}
