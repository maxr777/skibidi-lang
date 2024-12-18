mod token;
use std::{fs, process};
use token::{Token, TokenType, KEYWORDS};

fn main() {
    let source_code = fs::read_to_string("sc.txt").expect("Haven't loaded the source code");

    if source_code.len() == 0 {
        println!("blank program");
        process::exit(0);
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut current_line = 1;
    let keywords_map = KEYWORDS.get().unwrap();

    let mut chars = source_code.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ',' => tokens.push(Token {
                token_type: TokenType::Comma,
                line: current_line,
                lexeme: c.to_string(),
            }),
            ';' => tokens.push(Token {
                token_type: TokenType::Semicolon,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '"' => {
                chars.next();
                let mut word = String::new();

                while let Some(next_char) = chars.peek() {
                    if next_char == &'"' {
                        chars.next();
                        break;
                    } else {
                        word.push(next_char.clone());
                        chars.next();
                    }
                }
                tokens.push(Token {
                    token_type: TokenType::StringLiteral(word.clone()),
                    line: current_line,
                    lexeme: word.clone(),
                });
            }
            '(' => tokens.push(Token {
                token_type: TokenType::LParen,
                line: current_line,
                lexeme: c.to_string(),
            }),
            ')' => tokens.push(Token {
                token_type: TokenType::RParen,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '{' => tokens.push(Token {
                token_type: TokenType::LBrace,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '}' => tokens.push(Token {
                token_type: TokenType::RBrace,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '[' => tokens.push(Token {
                token_type: TokenType::LBracket,
                line: current_line,
                lexeme: c.to_string(),
            }),
            ']' => tokens.push(Token {
                token_type: TokenType::RBracket,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '=' => {
                if chars.peek() == Some(&'=') {
                    tokens.push(Token {
                        token_type: TokenType::Equal,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Assign,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                }
            }
            '!' => {
                if chars.peek() == Some(&'=') {
                    tokens.push(Token {
                        token_type: TokenType::NotEqual,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                } else {
                    panic!(
                        "error at line {}: ! is not immediately followed by = (use \"NOT\" for logical not)",
                        current_line
                    );
                }
            }
            '-' => tokens.push(Token {
                token_type: TokenType::Minus,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '+' => tokens.push(Token {
                token_type: TokenType::Plus,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '*' => tokens.push(Token {
                token_type: TokenType::Multiply,
                line: current_line,
                lexeme: c.to_string(),
            }),
            '/' => {
                if chars.peek() == Some(&'/') {
                    chars.next();
                    let mut comment_content = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '\n' {
                            tokens.push(Token {
                                token_type: TokenType::Comment(comment_content.clone()),
                                line: current_line,
                                lexeme: comment_content.clone(),
                            });
                            break;
                        }
                        comment_content.push(ch);
                        chars.next();
                    }
                } else {
                    tokens.push(Token {
                        token_type: TokenType::Divide,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    tokens.push(Token {
                        token_type: TokenType::LessEqual,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                } else {
                    tokens.push(Token {
                        token_type: TokenType::LessThan,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    tokens.push(Token {
                        token_type: TokenType::GreaterEqual,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                } else {
                    tokens.push(Token {
                        token_type: TokenType::GreaterThan,
                        line: current_line,
                        lexeme: c.to_string(),
                    })
                }
            }
            '\n' => current_line += 1,
            _ => {
                if c.is_alphanumeric() {
                    let mut word = String::new();
                    word.push(c);

                    if let Some(&next_char) = chars.peek() {
                        if !next_char.is_alphanumeric() {
                            if keywords_map.contains_key(&word.as_str()) {
                                tokens.push(Token {
                                    token_type: keywords_map.get(&word.as_str()).cloned().unwrap(),
                                    line: current_line,
                                    lexeme: word.clone(),
                                });
                                word.clear();
                            } else {
                                tokens.push(Token {
                                    token_type: TokenType::Identifier(word.clone()),
                                    line: current_line,
                                    lexeme: word.clone(),
                                });
                                word.clear()
                            }
                        }
                    }
                } else {
                    println!("Invalid character at line {}", current_line)
                }
            }
        }
    }

    tokens.push(Token {
        token_type: TokenType::EOF,
        line: current_line,
        lexeme: "EOF".to_string(),
    });
}
