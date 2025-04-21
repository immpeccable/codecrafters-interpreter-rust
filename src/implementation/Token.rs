use crate::TokenType;

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub line: u32,
}
