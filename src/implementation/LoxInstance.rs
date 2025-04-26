use std::collections::HashMap;

use crate::enums::LiteralValue::LiteralValue;

use super::{LoxClass::LoxClass, Token::Token};

#[derive(Clone)]
pub struct LoxInstance {
    pub klass: LoxClass,
    pub fields: HashMap<String, LiteralValue>,
}

impl LoxInstance {
    pub fn get(&self, token: Token) -> Option<LiteralValue> {
        if self.fields.contains_key(&token.token_value.to_string()) {
            return self.fields.get(&token.token_value.to_string()).cloned();
        } else {
            match self.klass.find_method(token) {
                Some(v) => Some(LiteralValue::Function(v)),
                None => None,
            }
        }
    }
    pub fn set(&mut self, token: Token, value: LiteralValue) {
        self.fields.insert(token.token_value.to_string(), value);
    }
}
