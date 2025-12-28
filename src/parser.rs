use std::mem::MaybeUninit;

use crate::ast::*;
use crate::token::*;

//recursive descent

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap()
    }

    fn bump(&mut self) -> Token {
        let tok = self.peek().clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let tok = self.bump();
        if tok.kind == kind {
            Ok(tok)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: kind.name(),
                found: tok,
            })
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut functions = Vec::new();

        while self.peek().kind != TokenKind::EOF {
            let func = self.parse_function()?;
            functions.push(func);
        }
        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> Result<Function, ParseError> {
        // Placeholder implementation
        self.expect(TokenKind::Fn)?;

        let name = match self.bump().kind.clone() {
            TokenKind::Ident(s) => s,
            tok => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: Token {
                        kind: tok,
                        span: self.peek().span.clone(),
                    },
                });
            }
        };

        self.expect(TokenKind::LParen)?;

        //TODO: Type hints, since only integer ignore for now
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                match self.bump().kind.clone() {
                    TokenKind::Ident(s) => params.push(s),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "parameter name".to_string(),
                            found: self.peek().clone(),
                        });
                    }
                }

                if self.peek().kind == TokenKind::Comma {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Arrow)?;
        self.expect(TokenKind::IntType)?;

        let body = self.parse_block()?;

        Ok(Function { name, params, body })
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();

        while self.peek().kind != TokenKind::RBrace {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        self.expect(TokenKind::RBrace)?;
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().kind {
            TokenKind::Let => self.parse_let(),
            TokenKind::Return => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Let)?;

        let name = match self.bump().kind.clone() {
            TokenKind::Ident(s) => s,
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: self.peek().clone(),
                });
            }
        };

        self.expect(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Return)?;

        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::If)?;
        let cond = self.parse_expr()?;
        let then_block = self.parse_block()?;

        let else_block = if self.peek().kind == TokenKind::Else {
            self.bump();
            self.parse_block()?
        } else {
            Vec::new()
        };

        Ok(Stmt::If {
            cond,
            then_block,
            else_block,
        })
    }

    fn parse_while(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::While)?;
        let cond = self.parse_expr()?;
        let body = self.parse_block()?;

        Ok(Stmt::While { cond, body })
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;

        while self.peek().kind == TokenKind::EqualEqual {
            self.bump();
            let rhs = self.parse_term()?;
            expr = Expr::Binary {
                op: BinOp::Eq,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;

        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = match self.bump().kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => unreachable!(),
            };

            let rhs = self.parse_factor()?;

            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        while matches!(self.peek().kind, TokenKind::Star | TokenKind::Slash) {
            let op = match self.bump().kind {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => unreachable!(),
            };

            let rhs = self.parse_primary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let tok = self.bump().clone();

        match tok.kind {
            TokenKind::Int(value) => Ok(Expr::Int(value)),

            TokenKind::Ident(name) => {
                if self.peek().kind == TokenKind::LParen {
                    self.bump(); // '('
                    let mut args = Vec::new();

                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expr()?);

                            if self.peek().kind == TokenKind::Comma {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Local(name))
                }
            }

            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            _ => Err(ParseError::ExpectedExpression { span: tok.span }),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    ExpectedExpression { span: Span },
}
