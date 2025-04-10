use std::convert;
use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

use anyhow::{anyhow, Result, Error};
use std::result::Result::Ok;


fn convert(val: f32, precision: usize) -> String {
    format!("{:.prec$}", val, prec = precision)
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

fn populate_number(chars: &mut Peekable<Chars>, number: &mut String) {
    let mut dot_used = false;
    while let Some(ch) = chars.peek() {

        if ch.is_digit(10) {
            number.push(*ch);
            chars.next();
        } else if *ch == '.' && !dot_used {
            dot_used = true;
            number.push(*ch);
            chars.next();
        }
        else {
            break;
        }
    }
}

fn populate_identifier(chars: &mut Peekable<Chars>, identifier: &mut String) {
    while let Some(ch) = chars.peek() {
        if ch.is_alphabetic() || *ch == '_' || ch.is_digit(10) {
            identifier.push(*ch);
            chars.next();
        }
        else {
            break;
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
    let mut exit_code = 0;
    let mut line = 1;

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });


            if !file_contents.is_empty() {
                let mut chars = file_contents.chars().peekable();
                while let Some(char) = chars.next() {
                    match char {
                        '(' => {
                            println!("LEFT_PAREN {} null", char)
                        },
                        ')' => {
                            println!("RIGHT_PAREN {} null", char)
                        },
                        '{' => {
                            println!("LEFT_BRACE {} null", char)
                        },
                        '}' => {
                            println!("RIGHT_BRACE {} null", char)
                        },
                        '*' => {
                            println!("STAR {} null", char)
                        },
                        '.' => {
                            println!("DOT {} null", char)
                        },
                        ',' => {
                            println!("COMMA {} null", char)
                        },
                        '+' => {
                            println!("PLUS {} null", char)
                        },
                        '-' => {
                            println!("MINUS {} null", char)
                        }
                        ';' => {
                            println!("SEMICOLON {} null", char)
                        },
                        '=' => {
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '=' {
                                    chars.next();
                                    println!("EQUAL_EQUAL {} null", "==");
                                } else {
                                    println!("EQUAL {} null", "=");
                                }
                            } else {
                                println!("EQUAL {} null", "=");
                            }
                        },
                        '!' => {
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '=' {
                                    chars.next();
                                    println!("BANG_EQUAL != null");
                                } else {
                                    println!("BANG ! null");
                                }
                            } else {
                                println!("BANG ! null");
                            }
                        },
                        '<' => {
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '=' {
                                    chars.next();
                                    println!("LESS_EQUAL <= null");
                                } else {
                                    println!("LESS < null");
                                }
                            } else {
                                println!("LESS < null");
                            }
                        },
                        '>' => {
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '=' {
                                    chars.next();
                                    println!("GREATER_EQUAL >= null");
                                } else {
                                    println!("GREATER > null");
                                }
                            } else {
                                println!("GREATER > null");
                            }
                        },
                        '/' => {
                            if let Some(&next_ch) = chars.peek() {
                                if next_ch == '/' {
                                    consume_until_next_line(&mut chars);
                                    line += 1
                                } else {
                                    println!("SLASH / null");
                                }
                            } else {
                                println!("SLASH / null");
                            }
                        },
                        '"' => {
                            match consume_until_next_double_quote(&mut chars) {
                                Ok(literal) => println!("STRING \"{}\" {literal}", literal),
                                Err(_) => {
                                    writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line).unwrap();
                                    exit_code = 65;
                                },
                            }
                        }
                        '\n' => {
                            line += 1;
                            continue;
                        }
                        '\t' | ' '  => continue,
                        fallback => {
                            if fallback.is_digit(10) {
                                let mut number = String::new();
                                number.push(fallback);
                                populate_number(&mut chars, &mut number);
                                let float_value = number.parse::<f32>().unwrap();

                                let precision;
                                match number.find('.') {
                                    Some(index) => {precision = number.len() - index - 1}
                                    None => {precision = 0}
                                }
                                if precision == 0 || float_value.fract() == 0.0 {
                                    println!("NUMBER {} {:.1}", number, float_value);
                                } else {
                                    println!("NUMBER {} {}", number, number);
                                }
                            }
                            else if fallback.is_alphabetic() || fallback == '_' {
                                let mut identifier = String::new();
                                identifier.push(fallback);
                                populate_identifier(&mut chars, &mut identifier);
                                println!("IDENTIFIER {} null", identifier);
                            }
                            else {
                                exit_code = 65;
                                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line, fallback).unwrap()
                            }
                        }
                    }
                }

            } 
            print!("EOF  null");
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    exit(exit_code);
}
