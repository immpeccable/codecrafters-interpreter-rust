use crate::enums::LiteralValue::LiteralValue;

pub trait Statement {
    fn statement_print(&self) -> String;
    fn interpret(&self);
}
