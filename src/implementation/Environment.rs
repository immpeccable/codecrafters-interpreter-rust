use std::collections::HashMap;

use crate::enums::LiteralValue::LiteralValue;

use super::Token::Token;

#[derive(Default)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
    pub fn get(&mut self, name: String) -> Option<&LiteralValue> {
        return self.values.get(&name);
    }
    pub fn assign(&mut self, token: Token, value: LiteralValue) -> Result<(), String> {
        if self.values.contains_key(&token.token_value) {
            self.values.insert(token.token_value, value);
            return Ok(());
        }
        return Err(String::from(format!(
            "Undefined variable {}.",
            token.token_value
        )));
    }
}
