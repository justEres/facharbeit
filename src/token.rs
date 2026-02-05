#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    //Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    Return,

    //Identifiers + literals
    Ident(String),
    Int(i64),

    //Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    Equal,

    //Comparison Operators
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    //Delimeters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,

    //Type Hints
    Colon,
    Arrow, 
    IntType,

    EOF,
}

impl TokenKind {
    pub fn name(&self) -> String {
        match self {
            TokenKind::Ident(_) => "identifier".to_string(),
            TokenKind::Int(_) => "integer literal".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}


