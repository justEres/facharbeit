use std::str::Chars;

use crate::token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    src: &'a str,
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

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        let start = self.pos;

        let ch = match self.bump() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::EOF,
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

    pub fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

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

    pub fn new(src: &'a str) -> Self {
        Lexer {
            src,
            chars: src.chars(),
            pos: 0,
        }
    }
}

pub fn lex_file(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token()?;
        if token.kind == TokenKind::EOF {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }

    Ok(tokens)
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedChar { ch: char, span: Span },
    InvalidNumber { span: Span },
    UnterminatedComment { span: Span },
}

pub fn report_lex_error(src: &str, error: LexError) {
    match error {
        LexError::UnexpectedChar { ch, span } => {
            eprintln!(
                "LexError: Unexpected character '{}' at {}:{}",
                ch, span.start, span.end
            );
        }
        LexError::InvalidNumber { span } => {
            eprintln!("LexError: Invalid number at {}:{}", span.start, span.end);
        }
        LexError::UnterminatedComment { span } => {
            eprintln!(
                "LexError: Unterminated comment starting at {}:{}",
                span.start, span.end
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
        // ensure EOF is last
        assert_eq!(tokens.last().unwrap().kind, TokenKind::EOF);
        // Check that some known tokens appear in order (spot check)
        let kinds: Vec<_> = tokens.iter().map(|t| &t.kind).collect();
        assert!(kinds.contains(&&TokenKind::LParen));
        assert!(kinds.contains(&&TokenKind::LessEqual));
        assert!(kinds.contains(&&TokenKind::GreaterEqual));
        assert!(kinds.contains(&&TokenKind::EqualEqual));
        assert!(kinds.contains(&&TokenKind::NotEqual));
    }
}
