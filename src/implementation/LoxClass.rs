use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    enums::{LiteralValue::LiteralValue, TokenType::TokenType},
    traits::LoxCallableTrait::LoxCallableTrait,
};

use super::{LoxFunction::LoxFunction, LoxInstance::LoxInstance, Token::Token};

#[derive(Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
    pub superclass: Option<Rc<RefCell<LoxClass>>>,
}

impl LoxCallableTrait for LoxClass {
    fn arity(&mut self) -> usize {
        if let Some(mut initializer) = self.find_method(String::from("init")) {
            return initializer.arity();
        }
        return 0;
    }
    fn call(
        &mut self,
        interpreter: &mut super::Interpreter::Interpreter,
        arguments: Vec<crate::enums::LiteralValue::LiteralValue>,
    ) -> LiteralValue {
        let lox_instance = Rc::new(RefCell::new(LoxInstance {
            klass: self.clone(),
            fields: HashMap::new(),
        }));
        if let Some(i) = self.find_method(String::from("init")) {
            let mut binded = i.bind(Rc::clone(&lox_instance));
            binded.call(interpreter, arguments);
        }

        return LiteralValue::Instance(lox_instance);
    }
}

impl LoxClass {
    pub fn find_method(&self, method_name: String) -> Option<LoxFunction> {
        if let Some(mt) = self.methods.get(&method_name) {
            return Some(mt.clone());
        }
        if let Some(superclass) = &self.superclass {
            return superclass.borrow_mut().find_method(method_name);
        }
        return None;
    }
}
