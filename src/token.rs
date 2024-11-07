use std::collections::HashMap;
use std::sync::OnceLock;

pub static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

fn init_keywords() -> HashMap<&'static str, TokenType> {
    let mut map = HashMap::new();
    map.insert("OR", TokenType::OR);
    map.insert("AND", TokenType::AND);
    map.insert("NOT", TokenType::NOT);
    map.insert("func", TokenType::Function);
    map.insert("mut", TokenType::Mutable);
    map.insert("bool", TokenType::Bool);
    map.insert("fp", TokenType::FloatingPoint);
    map.insert("int", TokenType::Integer);
    map.insert("str", TokenType::String);
    map.insert("ret", TokenType::Return);
    map.insert("if", TokenType::If);
    map.insert("else", TokenType::Else);
    map.insert("while", TokenType::While);
    map.insert("break", TokenType::Break);
    map.insert("continue", TokenType::Continue);

    map
}

// this is so we can easily find where an error occurs
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: String,
}

// PartialEq so we can compare tokens
// e.g. if we expect a semicolon we can compare Token.token_type to check
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // single character
    Comma,
    Semicolon,
    DoubleQuote,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Assign,
    Minus,
    Plus,
    Multiply,
    Divide,
    LessThan,
    GreaterThan,

    // double character
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,

    // keywords
    // logic
    OR,
    AND,
    NOT,
    // functions
    Function,
    Return,
    // variables
    Mutable,
    Bool,
    FloatingPoint,
    Integer,
    String,
    // conditionals
    If,
    Else,
    // loops
    While,
    Break,
    Continue,

    // literals
    IntegerLiteral(i64),
    FloatingPointLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),

    // special
    Identifier(String),
    Comment,
    EOF,
}
