mod token;
use std::{fs, process};
use token::Token;
use token::TokenType;

fn main() {
    let source_code = fs::read_to_string("sc.txt").expect("Haven't loaded the source code");

    if source_code.len() == 0 {
        println!("blank program");
        process::exit(0);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut word = String::new();
    let mut current_line = 1;

    for c in source_code.chars() {
        match c {
            ',' => tokens.push(Token {
                token_type: TokenType::Comma,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '\n' => current_line += 1,
            _ => println!("Invalid character at line {}", current_line),
        }
    }
}
