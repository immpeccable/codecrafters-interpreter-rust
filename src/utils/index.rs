use crate::constants::constants::RESERVED_KEYWORDS_MAP;
use crate::implementation::Token::Token;

use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, Error, Result};

pub fn consume_until_next_line(chars: &mut Peekable<Chars>) {
    while let Some(char) = chars.next() {
        if char == '\n' {
            break;
        }
    }
}

pub fn consume_until_next_double_quote(chars: &mut Peekable<Chars>) -> Result<String, Error> {
    let mut literal = String::new();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            return Ok(literal);
        } else {
            literal.push(ch);
        }
    }
    Err(anyhow!("Unterminated string literal"))
}

pub fn get_number(chars: &mut Peekable<Chars>) -> String {
    let mut number = String::new();
    let mut dot_used = false;
    while let Some(ch) = chars.peek() {
        if ch.is_digit(10) {
            number.push(*ch);
            chars.next();
        } else if *ch == '.' && !dot_used {
            dot_used = true;
            number.push(*ch);
            chars.next();
        } else {
            break;
        }
    }
    return number;
}

pub fn get_identifier(chars: &mut Peekable<Chars>) -> String {
    let mut identifier = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_alphabetic() || *ch == '_' || ch.is_digit(10) {
            identifier.push(*ch);
            chars.next();
        } else {
            break;
        }
    }
    return identifier;
}

pub fn get_if_reserved_keyword(chars: &mut Peekable<Chars>, line: u32) -> Option<Token> {
    let mut word = String::new();
    let mut cloned_chars = chars.clone();
    let mut count = 0;

    while let Some(&ch) = cloned_chars.peek() {
        if !ch.is_alphabetic() {
            break;
        }
        if count < 6 {
            word.push(ch);
            cloned_chars.next();
            count += 1;
        } else {
            break;
        }
    }

    if let Some(token) = RESERVED_KEYWORDS_MAP.get(word.as_str()).cloned() {
        return Some(Token {
            token_type: token,
            token_value: word,
            line,
        });
    }
    return None;
}
