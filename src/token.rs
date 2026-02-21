#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let,
    Fn,
    If,
    Else,
    While,
    Return,

    // Identifiers + literals
    Ident(String),
    Int(i64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    Equal,

    // Comparison operators
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,

    // Type hints
    Colon,
    Arrow,
    IntType,

    Eof,
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
