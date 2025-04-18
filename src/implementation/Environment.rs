use std::collections::HashMap;

use crate::enums::LiteralValue::LiteralValue;

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
}
