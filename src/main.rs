use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use std::result::Result::Ok;

mod constants;
mod enums;
mod implementation;
mod traits;
mod utils;

use enums::LiteralValue::LiteralValue;
use implementation::Interpreter::Interpreter;
use traits::Interpreter::InterpreterTrait;
use utils::index::{
    consume_until_next_double_quote, consume_until_next_line, get_identifier,
    get_if_reserved_keyword, get_number,
};

use enums::TokenType::TokenType;

use implementation::Parser::Parser;
use implementation::Token::Token;

struct TokenizeResult {
    tokens: Vec<Token>,
    exit_code: i32,
}

fn tokenize(file_contents: String) -> TokenizeResult {
    let mut chars = file_contents.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    let mut exit_code = 0;
    let mut line = 1;

    while let Some(_) = chars.peek() {
        match chars.peek().unwrap() {
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::LEFT_PAREN,
                    token_value: "(".to_string(),
                    line,
                });
                chars.next();
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::RIGHT_PAREN,
                    token_value: ")".to_string(),
                    line,
                });
                chars.next();
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::LEFT_BRACE,
                    token_value: "{".to_string(),
                    line,
                });
                chars.next();
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::RIGHT_BRACE,
                    token_value: "}".to_string(),
                    line,
                });
                chars.next();
            }
            '*' => {
                tokens.push(Token {
                    token_type: TokenType::STAR,
                    token_value: "*".to_string(),
                    line,
                });
                chars.next();
            }
            '.' => {
                tokens.push(Token {
                    token_type: TokenType::DOT,
                    token_value: ".".to_string(),
                    line,
                });
                chars.next();
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::COMMA,
                    token_value: ",".to_string(),
                    line,
                });
                chars.next();
            }
            '+' => {
                tokens.push(Token {
                    token_type: TokenType::PLUS,
                    token_value: "+".to_string(),
                    line,
                });
                chars.next();
            }
            '-' => {
                tokens.push(Token {
                    token_type: TokenType::MINUS,
                    token_value: "-".to_string(),
                    line,
                });
                chars.next();
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::SEMICOLON,
                    token_value: ";".to_string(),
                    line,
                });
                chars.next();
            }
            '=' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::EQUAL_EQUAL,
                        token_value: "==".to_string(),
                        line,
                    }
                } else {
                    Token {
                        token_type: TokenType::EQUAL,
                        token_value: "=".to_string(),
                        line,
                    }
                };
                tokens.push(token);
            }
            '!' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::BANG_EQUAL,
                        token_value: "!=".to_string(),
                        line,
                    }
                } else {
                    Token {
                        token_type: TokenType::BANG,
                        token_value: "!".to_string(),
                        line,
                    }
                };
                tokens.push(token);
            }
            '<' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::LESS_EQUAL,
                        token_value: "<=".to_string(),
                        line,
                    }
                } else {
                    Token {
                        token_type: TokenType::LESS,
                        token_value: "<".to_string(),
                        line,
                    }
                };
                tokens.push(token);
            }
            '>' => {
                chars.next();
                let token = if chars.peek() == Some(&'=') {
                    chars.next();
                    Token {
                        token_type: TokenType::GREATER_EQUAL,
                        token_value: ">=".to_string(),
                        line,
                    }
                } else {
                    Token {
                        token_type: TokenType::GREATER,
                        token_value: ">".to_string(),
                        line,
                    }
                };
                tokens.push(token);
            }
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    chars.next(); // Consume second '/'
                    consume_until_next_line(&mut chars);
                    line += 1;
                } else {
                    tokens.push(Token {
                        token_type: TokenType::SLASH,
                        token_value: "/".to_string(),
                        line,
                    });
                }
            }
            '"' => {
                chars.next(); // Consume the opening quote.
                match consume_until_next_double_quote(&mut chars) {
                    Ok(literal) => {
                        tokens.push(Token {
                            token_type: TokenType::STRING,
                            token_value: literal,
                            line,
                        });
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
                    let number_str = get_number(&mut chars);
                    tokens.push(Token {
                        token_type: TokenType::NUMBER,
                        token_value: number_str,
                        line,
                    });
                } else if fallback.is_alphabetic() || *fallback == '_' {
                    if let Some(result) = get_if_reserved_keyword(&mut chars, line) {
                        for _ in 0..result.token_value.len() {
                            chars.next();
                        }
                        tokens.push(result);
                    } else {
                        let identifier = get_identifier(&mut chars);
                        tokens.push(Token {
                            token_type: TokenType::IDENTIFIER,
                            token_value: identifier,
                            line,
                        });
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
    tokens.push(Token {
        token_type: TokenType::EOF,
        token_value: "EOF".to_string(),
        line,
    });
    TokenizeResult { tokens, exit_code }
}

fn token_printer(tokens: &[Token]) {
    for token in tokens {
        match token.token_type {
            TokenType::LEFT_PAREN => {
                println!("LEFT_PAREN {} null", token.token_value)
            }
            TokenType::RIGHT_PAREN => {
                println!("RIGHT_PAREN {} null", token.token_value)
            }
            TokenType::LEFT_BRACE => {
                println!("LEFT_BRACE {} null", token.token_value)
            }
            TokenType::RIGHT_BRACE => {
                println!("RIGHT_BRACE {} null", token.token_value)
            }
            TokenType::STAR => {
                println!("STAR {} null", token.token_value)
            }
            TokenType::DOT => {
                println!("DOT {} null", token.token_value)
            }
            TokenType::COMMA => {
                println!("COMMA {} null", token.token_value)
            }
            TokenType::PLUS => {
                println!("PLUS {} null", token.token_value)
            }
            TokenType::MINUS => {
                println!("MINUS {} null", token.token_value)
            }
            TokenType::SEMICOLON => {
                println!("SEMICOLON {} null", token.token_value)
            }
            TokenType::EQUAL => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::EQUAL_EQUAL => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::BANG => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::BANG_EQUAL => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::LESS => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::LESS_EQUAL => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::GREATER => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::GREATER_EQUAL => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::SLASH => {
                println!("{:?} {} null", token.token_type, token.token_value)
            }
            TokenType::STRING => {
                // Replicate the original print for strings.
                println!("STRING \"{}\" {}", token.token_value, token.token_value)
            }
            TokenType::NUMBER => {
                // For numbers, replicate the formatted output.
                if let Ok(num) = token.token_value.parse::<f32>() {
                    let precision = token
                        .token_value
                        .find('.')
                        .map(|i| token.token_value.len() - i - 1)
                        .unwrap_or(0);
                    if precision == 0 || num.fract() == 0.0 {
                        println!("NUMBER {} {:.1}", token.token_value, num);
                    } else {
                        println!("NUMBER {} {}", token.token_value, token.token_value);
                    }
                } else {
                    println!("NUMBER {} {}", token.token_value, token.token_value);
                }
            }
            TokenType::IDENTIFIER => {
                println!("IDENTIFIER {} null", token.token_value)
            }
            TokenType::AND
            | TokenType::CLASS
            | TokenType::ELSE
            | TokenType::FALSE
            | TokenType::FUN
            | TokenType::FOR
            | TokenType::IF
            | TokenType::NIL
            | TokenType::OR
            | TokenType::PRINT
            | TokenType::RETURN
            | TokenType::SUPER
            | TokenType::THIS
            | TokenType::TRUE
            | TokenType::VAR
            | TokenType::WHILE => {
                // Reserved keywords: print uppercase of token value, then token_value, then "null".
                println!(
                    "{} {} null",
                    token.token_value.to_uppercase(),
                    token.token_value
                )
            }
            TokenType::EOF => {
                println!("EOF  null")
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    match command.as_str() {
        "tokenize" => {
            let result: TokenizeResult = tokenize(file_contents);
            token_printer(&result.tokens);
            exit(result.exit_code);
        }
        "parse" => {
            let result = tokenize(file_contents);
            let mut parser = Parser {
                tokens: result.tokens,
                current: 0,
            };
            let parser_result = parser.expression();
            match parser_result {
                Ok(expr) => {
                    let string_representation = expr.expression_print();
                    println!("{}", string_representation);
                }
                Err(err) => exit(65),
            }
        }
        "evaluate" => {
            let result = tokenize(file_contents);
            let mut parser = Parser {
                tokens: result.tokens,
                current: 0,
            };
            let parser_result = parser.expression();
            match parser_result {
                Ok(expr) => {
                    let literal_value = expr.interpret().unwrap();
                    match literal_value {
                        LiteralValue::Number(n) => {
                            println!("{}", n.parse::<f64>().unwrap());
                        }
                        _ => println!("{}", literal_value.to_string()),
                    }
                }
                Err(_) => exit(65),
            }
        }
        "run" => {
            let result = tokenize(file_contents);
            let mut parser = Parser {
                tokens: result.tokens,
                current: 0,
            };
            let parser_res = parser.parse();
            let intp = Interpreter {};
            match parser_res {
                Ok(statements) => intp.interpret(statements),
                Err(_) => exit(65),
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        }
    }
}
