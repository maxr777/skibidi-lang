// this is so we can easily find where an error occurs
#[derive(Debug)]
struct Token {
    token_type: TokenType,
    line: usize,
    lexeme: String,
}

// PartialEq so we can compare tokens
// e.g. if we expect a semicolon we can compare Token.token_type to check
#[derive(Debug, PartialEq)]
enum TokenType {
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
    Equals,
    Minus,
    Plus,
    Multiply,
    Divide,

    // double character
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    Greater,
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
    Identifier(String),
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
    Comment,
    EOF,
}
