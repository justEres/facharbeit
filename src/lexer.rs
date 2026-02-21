use std::str::Chars;
use std::fmt::{Display, Formatter};

use crate::diagnostics::render_snippet;
use crate::token::{Span, Token, TokenKind};

/// Stateful lexer for one source string.
pub struct Lexer<'a> {
    chars: Chars<'a>,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn span_from(&self, start: usize) -> Span {
        Span {
            start,
            end: self.pos,
        }
    }

    /// Produces the next token from the input stream.
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let start = self.pos;

        let ch = match self.bump() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::Eof,
                    span: Span { start, end: start },
                });
            }
        };

        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ';' => TokenKind::Semicolon,
            '+' => TokenKind::Plus,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '>' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '-' => {
                if let Some('>') = self.peek() {
                    self.bump();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => TokenKind::Star,
            '/' => {
                if let Some('/') = self.peek() {
                    // Single-line comment
                    while let Some(c) = self.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.bump();
                    }
                    return self.next_token();
                } else {
                    TokenKind::Slash
                }
            }
            '%' => TokenKind::Percentage,
            '=' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                if let Some('=') = self.peek() {
                    self.bump();
                    TokenKind::NotEqual
                } else {
                    return Err(LexError::UnexpectedChar {
                        ch: '!',
                        span: self.span_from(start),
                    });
                }
            }
            c if c.is_ascii_digit() => self.lex_number(c)?,
            c if Lexer::is_ident_start(c) => self.lex_ident(c),

            _ => {
                return Err(LexError::UnexpectedChar {
                    ch,
                    span: self.span_from(start),
                });
            }
        };

        Ok(Token {
            kind,
            span: self.span_from(start),
        })
    }

    /// Lexes an integer literal starting with the already consumed first digit.
    pub fn lex_number(&mut self, first_digit: char) -> Result<TokenKind, LexError> {
        let start = self.pos - first_digit.len_utf8();
        let mut number_str = first_digit.to_string();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.bump();
            } else {
                break;
            }
        }

        match number_str.parse::<i64>() {
            Ok(value) => Ok(TokenKind::Int(value)),
            Err(_) => Err(LexError::InvalidNumber {
                span: self.span_from(start),
            }),
        }
    }

    /// Returns true if `c` can start an identifier.
    pub fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    /// Lexes identifiers and keywords.
    pub fn lex_ident(&mut self, first_char: char) -> TokenKind {
        let mut ident_str = first_char.to_string();

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                ident_str.push(c);
                self.bump();
            } else {
                break;
            }
        }

        match ident_str.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "Int" => TokenKind::IntType,
            _ => TokenKind::Ident(ident_str),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    /// Creates a lexer over `src`.
    pub fn new(src: &'a str) -> Self {
        Lexer {
            chars: src.chars(),
            pos: 0,
        }
    }
}

/// Lexes a full source file into tokens and appends an `Eof` token.
pub fn lex_file(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token()?;
        if token.kind == TokenKind::Eof {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum LexError {
    /// Input contained a character that cannot start any valid token.
    UnexpectedChar { ch: char, span: Span },
    /// Integer literal could not be parsed into the target numeric type.
    InvalidNumber { span: Span },
}

impl Display for LexError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedChar { ch, span } => write!(
                f,
                "unexpected character '{}' at byte range {}..{}",
                ch, span.start, span.end
            ),
            LexError::InvalidNumber { span } => {
                write!(f, "invalid number literal at byte range {}..{}", span.start, span.end)
            }
        }
    }
}

/// Prints a human-readable lexer error.
pub fn report_lex_error(src: &str, error: LexError) {
    match error {
        LexError::UnexpectedChar { ch, span } => {
            let snippet = render_snippet(src, &span);
            eprintln!(
                "LexError at line {}, column {}: unexpected character '{}'\n{}\n{}",
                snippet.line, snippet.column, ch, snippet.source_line, snippet.marker_line
            );
        }
        LexError::InvalidNumber { span } => {
            let snippet = render_snippet(src, &span);
            eprintln!(
                "LexError at line {}, column {}: invalid number literal\n{}\n{}",
                snippet.line, snippet.column, snippet.source_line, snippet.marker_line
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_simple_tokens() {
        let src = "( ) { } ; + - * / % , : -> <= < >= > == != ->";
        // include arrow twice intentionally
        let tokens = lex_file(src).expect("lexing failed");
        // ensure Eof is last
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
        // Check that some known tokens appear in order (spot check)
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::LParen));
        assert!(kinds.contains(&&TokenKind::LessEqual));
        assert!(kinds.contains(&&TokenKind::GreaterEqual));
        assert!(kinds.contains(&&TokenKind::EqualEqual));
        assert!(kinds.contains(&&TokenKind::NotEqual));
    }

    #[test]
    fn lex_skips_single_line_comments() {
        let src = "fn main() { // this is ignored\n return; }";
        let tokens = lex_file(src).expect("lexing failed");
        assert_eq!(tokens[0].kind, TokenKind::Fn);
        assert_eq!(tokens.last().expect("missing eof").kind, TokenKind::Eof);
    }

    #[test]
    fn lex_reports_unexpected_char() {
        let err = lex_file("@").expect_err("expected lex error");
        match err {
            LexError::UnexpectedChar { ch, .. } => assert_eq!(ch, '@'),
            _ => panic!("expected UnexpectedChar"),
        }
    }
}
