use std::fmt;

use crate::implementation::Clock::Clock;

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralValue {
    Nil,
    Boolean(bool),
    Number(String),
    String(String),
    Function(Clock),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Boolean(val) => write!(f, "{}", val),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::String(s) => write!(f, "{}", s),
            LiteralValue::Function(_) => write!(f, "<native fn>"),
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
