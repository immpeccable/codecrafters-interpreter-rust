use std::time::{SystemTime, UNIX_EPOCH};

use crate::{enums::LiteralValue::LiteralValue, traits::LoxCallableTrait::LoxCallableTrait};

use super::Interpreter::Interpreter;

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Clock {}

impl LoxCallableTrait for Clock {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> LiteralValue {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        return LiteralValue::Number(since_the_epoch.as_secs().to_string());
    }

    fn arity(&mut self) -> usize {
        return 0;
    }
}
