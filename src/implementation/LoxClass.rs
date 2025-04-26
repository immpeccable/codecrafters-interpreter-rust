use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{enums::LiteralValue::LiteralValue, traits::LoxCallableTrait::LoxCallableTrait};

use super::{LoxFunction::LoxFunction, LoxInstance::LoxInstance, Token::Token};

#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
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
        let instance_rc = Rc::new(RefCell::new(LoxInstance {
            klass: self.clone(),
            fields: HashMap::new(),
        }));
        return LiteralValue::Instance(instance_rc);
    }
}

impl LoxClass {
    pub fn find_method(&self, method_name: Token) -> Option<LoxFunction> {
        return self
            .methods
            .get(&method_name.token_value.to_string())
            .cloned();
    }
}
