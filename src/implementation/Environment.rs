use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::enums::LiteralValue::LiteralValue;

use super::Token::Token;

#[derive(Default, Clone)]
pub struct Environment {
    pub values: HashMap<String, LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

pub trait EnvExt {
    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>>;
    fn get_at(&self, distance: usize, name: &str) -> Option<LiteralValue>;
    fn assign_at(&self, distance: usize, token: Token, value: LiteralValue) -> Result<(), String>;
}

impl EnvExt for Rc<RefCell<Environment>> {
    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let mut env = Rc::clone(self);
        for _ in 0..distance {
            let parent = {
                let guard = env.borrow();
                guard
                    .enclosing
                    .as_ref()
                    .expect("no enclosing environment")
                    .clone()
            };
            env = parent;
        }
        env
    }

    fn assign_at(&self, distance: usize, token: Token, value: LiteralValue) -> Result<(), String> {
        let _res = self
            .ancestor(distance)
            .borrow_mut()
            .values
            .insert(token.token_value, value);

        return Ok(());
    }

    fn get_at(&self, distance: usize, name: &str) -> Option<LiteralValue> {
        return self
            .ancestor(distance)
            .borrow_mut()
            .values
            .get(name)
            .cloned();
    }
}

impl Environment {
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }
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
