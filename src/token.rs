/// Token kinds emitted by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let,
    Fn,
    Use,
    If,
    Else,
    While,
    Return,
    Struct,
    Enum,
    Match,

    // Identifiers + literals
    Ident(String),
    StringLit(String),
    Int(i64),
    Float(f64),
    True,
    False,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    Equal,
    Ampersand,

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
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    Dot,
    Colon,
    DoubleColon,
    Arrow,
    FatArrow,

    // Type keywords
    IntType,
    FloatType,
    BoolType,
    StringType,

    Eof,
}

impl TokenKind {
    /// Human-readable token name for diagnostics.
    pub fn name(&self) -> String {
        match self {
            TokenKind::Ident(_) => "identifier".to_string(),
            TokenKind::StringLit(_) => "string literal".to_string(),
            TokenKind::Int(_) => "integer literal".to_string(),
            TokenKind::Float(_) => "float literal".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

/// One token with source span information.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// Half-open byte span `[start, end)` in the original source.
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
