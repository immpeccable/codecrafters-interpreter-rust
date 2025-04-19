use std::collections::HashMap;

use crate::enums::LiteralValue::LiteralValue;

use super::Token::Token;

#[derive(Default)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
    pub fn get(&mut self, name: String) -> Option<&LiteralValue> {
        match self.values.get(&name) {
            Some(v) => Some(v),
            None => match self.enclosing.as_mut() {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }
    pub fn assign(&mut self, token: Token, value: LiteralValue) -> Result<(), String> {
        if self.values.contains_key(&token.token_value) {
            self.values.insert(token.token_value, value);
            return Ok(());
        }
        match self.enclosing.as_mut() {
            Some(parent) => {
                parent.assign(token.clone(), value)?;
            }
            None => {}
        }
        return Err(String::from(format!(
            "Undefined variable {}.",
            token.token_value
        )));
    }
}
