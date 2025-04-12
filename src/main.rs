use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

use anyhow::{anyhow, Error, Result};
use std::result::Result::Ok;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    EQUAL_EQUAL,   // "=="
    BANG_EQUAL,    // "!="
    LESS,          // "<"
    LESS_EQUAL,    // "<="
    GREATER,       // ">"
    GREATER_EQUAL, // ">="
    PLUS,          // "+"
    MINUS,         // "-"
    STAR,          // "*"
    SLASH,         // "/"
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(LiteralValue),

    Grouping(Box<Expr>),

    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    token_value: String,
}

pub struct AST {
    
}

struct TokenizeResult {
    tokens: Vec<Token>,
    exit_code: i32,
}

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

fn parse_operator(op: &str) -> Option<Operator> {
    match op {
        "==" => Some(Operator::EQUAL_EQUAL),
        "!=" => Some(Operator::BANG_EQUAL),
        "<" => Some(Operator::LESS),
        "<=" => Some(Operator::LESS_EQUAL),
        ">" => Some(Operator::GREATER),
        ">=" => Some(Operator::GREATER_EQUAL),
        "+" => Some(Operator::PLUS),
        "-" => Some(Operator::MINUS),
        "*" => Some(Operator::STAR),
        "/" => Some(Operator::SLASH),
        _ => None,
    }
}

fn consume_until_next_line(chars: &mut Peekable<Chars>) {
    while let Some(char) = chars.next() {
        if char == '\n' {
            break;
        }
    }
}

fn consume_until_next_double_quote(chars: &mut Peekable<Chars>) -> Result<String, Error> {
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

fn get_number(chars: &mut Peekable<Chars>) -> String {
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

fn get_identifier(chars: &mut Peekable<Chars>) -> String {
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

fn get_if_reserved_keyword(chars: &mut Peekable<Chars>) -> Option<Token> {
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
        });
    }
    return None;
}

fn tokenize(file_contents: String) -> TokenizeResult {
    let mut chars = file_contents.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut exit_code = 0;
    let mut line = 1;
    while let Some(char) = chars.peek() {
        match char {
            '(' => {
                println!("LEFT_PAREN {} null", char);
                tokens.push(Token {
                    token_type: TokenType::LEFT_PAREN,
                    token_value: "(".to_string(),
                });
                chars.next();
            }
            ')' => {
                println!("RIGHT_PAREN {} null", char);
                tokens.push(Token {
                    token_type: TokenType::RIGHT_PAREN,
                    token_value: ")".to_string(),
                });
                chars.next();
            }
            '{' => {
                println!("LEFT_BRACE {} null", char);
                tokens.push(Token {
                    token_type: TokenType::LEFT_BRACE,
                    token_value: "{".to_string(),
                });
                chars.next();
            }
            '}' => {
                println!("RIGHT_BRACE {} null", char);
                tokens.push(Token {
                    token_type: TokenType::RIGHT_BRACE,
                    token_value: "}".to_string(),
                });
                chars.next();
            }
            '*' => {
                println!("STAR {} null", char);
                tokens.push(Token {
                    token_type: TokenType::STAR,
                    token_value: "*".to_string(),
                });
                chars.next();
            }
            '.' => {
                println!("DOT {} null", char);
                tokens.push(Token {
                    token_type: TokenType::DOT,
                    token_value: ".".to_string(),
                });
                chars.next();
            }
            ',' => {
                println!("COMMA {} null", char);
                tokens.push(Token {
                    token_type: TokenType::COMMA,
                    token_value: ",".to_string(),
                });
                chars.next();
            }
            '+' => {
                println!("PLUS {} null", char);
                tokens.push(Token {
                    token_type: TokenType::PLUS,
                    token_value: "+".to_string(),
                });
                chars.next();
            }
            '-' => {
                println!("MINUS {} null", char);
                tokens.push(Token {
                    token_type: TokenType::MINUS,
                    token_value: "-".to_string(),
                });
                chars.next();
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::SEMICOLON,
                    token_value: ";".to_string(),
                });
                chars.next();
            }
            '=' => {
                // Consume '='
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    // Consume the second '=' and create the dual token.
                    chars.next();
                    Token {
                        token_type: TokenType::EQUAL_EQUAL,
                        token_value: "==".to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::EQUAL,
                        token_value: "=".to_string(),
                    }
                };
                tokens.push(token);
                // Print using the token we just pushed.
                println!(
                    "{:?} {} null",
                    tokens.last().unwrap().token_type,
                    tokens.last().unwrap().token_value
                );
            }

            '!' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::BANG_EQUAL,
                        token_value: "!=".to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::BANG,
                        token_value: "!".to_string(),
                    }
                };
                tokens.push(token);
                println!(
                    "{:?} {} null",
                    tokens.last().unwrap().token_type,
                    tokens.last().unwrap().token_value
                );
            }

            '<' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::LESS_EQUAL,
                        token_value: "<=".to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::LESS,
                        token_value: "<".to_string(),
                    }
                };
                tokens.push(token);
                println!(
                    "{:?} {} null",
                    tokens.last().unwrap().token_type,
                    tokens.last().unwrap().token_value
                );
            }

            '>' => {
                // Consume '>'
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::GREATER_EQUAL,
                        token_value: ">=".to_string(),
                    }
                } else {
                    Token {
                        token_type: TokenType::GREATER,
                        token_value: ">".to_string(),
                    }
                };
                tokens.push(token);
                println!(
                    "{:?} {} null",
                    tokens.last().unwrap().token_type,
                    tokens.last().unwrap().token_value
                );
            }
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    chars.next(); // Consume the second '/'
                    consume_until_next_line(&mut chars);
                    line += 1;
                } else {
                    let token = Token {
                        token_type: TokenType::SLASH,
                        token_value: "/".to_string(),
                    };
                    tokens.push(token);
                    println!(
                        "{:?} {} null",
                        tokens.last().unwrap().token_type,
                        tokens.last().unwrap().token_value
                    );
                }
            }
            '"' => {
                chars.next();
                match consume_until_next_double_quote(&mut chars) {
                    Ok(literal) => {
                        let token = Token {
                            token_type: TokenType::STRING,
                            token_value: literal.clone(),
                        };
                        tokens.push(token);
                        println!("STRING \"{}\" {literal}", literal)
                    }
                    Err(_) => {
                        writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line)
                            .unwrap();
                        exit_code = 65;
                    }
                }
            }
            '\n' => {
                chars.next();
                line += 1;
            }
            '\t' | ' ' => {
                chars.next();
            }
            fallback => {
                if fallback.is_digit(10) {
                    let number = get_number(&mut chars);
                    let float_value = number.parse::<f32>().unwrap();

                    let precision;
                    match number.find('.') {
                        Some(index) => precision = number.len() - index - 1,
                        None => precision = 0,
                    }
                    if precision == 0 || float_value.fract() == 0.0 {
                        println!("NUMBER {} {:.1}", number, float_value);
                    } else {
                        println!("NUMBER {} {}", number, number);
                    }
                    let token = Token {
                        token_type: TokenType::NUMBER,
                        token_value: number,
                    };
                    tokens.push(token);
                } else if fallback.is_alphabetic() || *fallback == '_' {
                    if let Some(result) = get_if_reserved_keyword(&mut chars) {
                        println!(
                            "{} {} null",
                            result.token_value.to_uppercase(),
                            result.token_value
                        );
                        for _ in 0..result.token_value.len() {
                            chars.next();
                        }
                        tokens.push(result);
                    } else {
                        let identifier = get_identifier(&mut chars);
                        println!("IDENTIFIER {} null", identifier);
                        let token = Token {
                            token_type: TokenType::IDENTIFIER,
                            token_value: identifier,
                        };
                        tokens.push(token);
                    }
                } else {
                    exit_code = 65;
                    writeln!(
                        io::stderr(),
                        "[line {}] Error: Unexpected character: {}",
                        line,
                        fallback
                    )
                    .unwrap();
                    chars.next();
                }
            }
        }
    }
    print!("EOF  null");
    return TokenizeResult { tokens, exit_code };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            let result: TokenizeResult = tokenize(file_contents);
            exit(result.exit_code);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
