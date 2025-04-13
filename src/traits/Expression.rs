use std::any::Any;

use crate::enums::LiteralValue::LiteralValue;

pub trait Expression {
    fn expression_print(&self) -> String;
    fn interpret(&self) -> Result<LiteralValue, String>;
}
