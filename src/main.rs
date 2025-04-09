use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let mut exit_code = 0;


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
                        fallback => {
                            exit_code = 65;
                            writeln!(io::stderr(), "[line 1] Error: Unexpected character: {}", fallback).unwrap()
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
