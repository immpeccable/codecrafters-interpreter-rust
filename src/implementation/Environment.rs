use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::enums::LiteralValue::LiteralValue;

use super::Token::Token;

#[derive(Default, Clone)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
    /// Now returns an owned LiteralValue.
    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        if let Some(v) = self.values.get(name) {
            // clone out of the current scope
            Some(v.clone())
        } else if let Some(parent) = &self.enclosing {
            // recurse into parent
            parent.borrow().get(name)
        } else {
            None
        }
    }
    pub fn assign(&mut self, token: Token, value: LiteralValue) -> Result<(), String> {
        if self.values.contains_key(&token.token_value) {
            self.values.insert(token.token_value, value);
            return Ok(());
        }
        match self.enclosing.as_mut() {
            Some(parent) => {
                parent.borrow_mut().assign(token.clone(), value)?;
                return Ok(());
            }
            None => {}
        }
        return Err(String::from(format!(
            "Undefined variable {}.",
            token.token_value
        )));
    }
}
