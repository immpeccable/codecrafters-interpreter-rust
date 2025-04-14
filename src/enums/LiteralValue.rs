use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralValue {
    Nil,
    Boolean(bool),
    Number(String), // Number stored as a string.
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::Boolean(val) => write!(f, "{}", val),
            LiteralValue::Nil => write!(f, "nil"),
            LiteralValue::String(s) => write!(f, "{}", s),
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
