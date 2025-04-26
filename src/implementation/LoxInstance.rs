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
        return self.fields.get(&token.token_value.to_string()).cloned();
    }
    pub fn set(&mut self, token: Token, value: LiteralValue) {
        self.fields.insert(token.token_value.to_string(), value);
    }
}
