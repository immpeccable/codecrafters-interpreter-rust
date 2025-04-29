use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    enums::LiteralValue::LiteralValue,
    traits::{Interpreter::InterpreterTrait, LoxCallableTrait::LoxCallableTrait},
};

use super::{
    BlockStatement::BlockStatement,
    Environment::Environment,
    FunctionStatement::FunctionStatement,
    Interpreter::{Interpreter, SharedEnv},
    LoxInstance::LoxInstance,
};

pub struct LoxFunction {
    pub declaration: FunctionStatement,
    pub closure: SharedEnv,
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        LoxFunction {
            declaration: self.declaration.clone(),
            closure: self.closure.clone(),
        }
    }
}

impl LoxFunction {
    pub fn bind(&mut self, instance: &mut LoxInstance) -> LoxFunction {
        let parent = Rc::clone(&self.closure);
        let child = Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(parent),
        }));
        child.borrow_mut().define(
            String::from("this"),
            LiteralValue::Instance(Rc::new(RefCell::new(instance.clone()))),
        );

        return LoxFunction {
            declaration: self.declaration.clone(),
            closure: child,
        };
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
        // 1) Build a new, empty frame whose parent is the closure.
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

        // 5) Restore the caller’s environment
        interpreter.environment = old_env;

        // 6) Return the function’s return‐value or Nil
        result.unwrap_or(LiteralValue::Nil)
    }
}
