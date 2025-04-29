use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::enums::LiteralValue::LiteralValue;

use super::{LoxClass::LoxClass, Token::Token};

#[derive(Clone)]
pub struct LoxInstance {
    pub klass: LoxClass,
    pub fields: HashMap<String, LiteralValue>,
}

/// A small helper so we can write `rc_inst.get(token)`
pub trait LoxInstanceExt {
    fn get(&self, token: Token) -> Option<LiteralValue>;
    fn set(&self, token: Token, value: LiteralValue);
}

impl LoxInstanceExt for Rc<RefCell<LoxInstance>> {
    fn get(&self, token: Token) -> Option<LiteralValue> {
        let inst = self.borrow();
        if let Some(val) = inst.fields.get(&token.token_value) {
            Some(val.clone())
        } else if let Some(mut method) = inst.klass.find_method(token.token_value.clone()) {
            let bound = method.bind(Rc::clone(self));
            Some(LiteralValue::Function(bound))
        } else {
            None
        }
    }

    fn set(&self, token: Token, value: LiteralValue) {
        let mut inst = self.borrow_mut();
        inst.fields.insert(token.token_value, value);
    }
}
