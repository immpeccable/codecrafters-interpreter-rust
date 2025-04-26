use std::collections::HashMap;

use crate::{enums::LiteralValue::LiteralValue, traits::LoxCallableTrait::LoxCallableTrait};

use super::LoxInstance::LoxInstance;

#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxCallableTrait for LoxClass {
    fn arity(&mut self) -> usize {
        return 0;
    }
    fn call(
        &mut self,
        interpreter: &mut super::Interpreter::Interpreter,
        arguments: Vec<crate::enums::LiteralValue::LiteralValue>,
    ) -> LiteralValue {
        let instance = LoxInstance {
            klass: self.clone(),
            fields: HashMap::new(),
        };
        return LiteralValue::LoxInstance(instance);
    }
}
