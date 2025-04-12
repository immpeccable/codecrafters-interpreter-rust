use crate::enums::TokenType::TokenType;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static RESERVED_KEYWORDS: &[&str] = &[
    "and", "class", "else", "false", "fun", "for", "if", "nil", "or", "print", "return", "super",
    "this", "true", "var", "while",
];

pub static RESERVED_KEYWORDS_MAP: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::AND);
    m.insert("class", TokenType::CLASS);
    m.insert("else", TokenType::ELSE);
    m.insert("false", TokenType::FALSE);
    m.insert("fun", TokenType::FUN);
    m.insert("for", TokenType::FOR);
    m.insert("if", TokenType::IF);
    m.insert("nil", TokenType::NIL);
    m.insert("or", TokenType::OR);
    m.insert("print", TokenType::PRINT);
    m.insert("return", TokenType::RETURN);
    m.insert("super", TokenType::SUPER);
    m.insert("this", TokenType::THIS);
    m.insert("true", TokenType::TRUE);
    m.insert("var", TokenType::VAR);
    m.insert("while", TokenType::WHILE);
    m
});
