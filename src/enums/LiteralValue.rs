use std::fmt;

use crate::implementation::{
    Clock::Clock, LoxClass::LoxClass, LoxFunction::LoxFunction, LoxInstance::LoxInstance,
};

#[derive(Clone, Default)]
pub enum LiteralValue {
    #[default]
    Nil,
    Boolean(bool),
    Number(String),
    String(String),
    Clock(Clock),
    Function(LoxFunction),
    LoxClass(LoxClass),
    LoxIntance(LoxInstance),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Boolean(val) => write!(f, "{}", val),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Clock(_) => write!(f, "<native fn>"),
            LiteralValue::Function(lf) => {
                write!(f, "<fn {}>", lf.declaration.name.token_value.to_string())
            }
            LiteralValue::LoxClass(cl) => {
                write!(f, "{}", cl.name)
            }
            LiteralValue::LoxIntance(ins) => {
                write!(f, "{} instance", ins.klass.name)
            }
            LiteralValue::Number(s) => {
                if let Ok(num) = s.parse::<f64>() {
                    // Using default formatting gives us the trimmed version.
                    let formatted = format!("{}", num);
                    // If the default formatted string doesn't contain a '.', it means it's whole,
                    // so we append ".0". Otherwise, we output the formatted string as is.
                    if formatted.contains('.') {
                        write!(f, "{}", formatted)
                    } else {
                        write!(f, "{}.0", formatted)
                    }
                } else {
                    // Fallback: if parsing fails, simply print the original string.
                    write!(f, "{}", s)
                }
            }
        }
    }
}
