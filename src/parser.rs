
use crate::ast::*;
use crate::token::*;

//recursive descent

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex_file;

    #[test]
    fn parse_paren_comparison() {
        let src = "fn main(x, y) -> Int { if (x < y) { return 1; } else { return 0; } }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");

        // one function
        assert_eq!(program.functions.len(), 1);
        let f = &program.functions[0];
        assert_eq!(f.body.len(), 1);

        match &f.body[0] {
            Stmt::If { cond, then_block, else_block } => {
                // cond should be Binary with Lt
                match cond {
                    Expr::Binary { op, left, right } => {
                        match op {
                            BinOp::Lt => (),
                            _ => panic!("expected Lt op"),
                        }

                        match (&**left, &**right) {
                            (Expr::Local(l), Expr::Local(r)) => {
                                assert_eq!(l, "x");
                                assert_eq!(r, "y");
                            }
                            _ => panic!("expected locals in comparison"),
                        }
                    }
                    _ => panic!("expected binary cond"),
                }

                assert_eq!(then_block.len(), 1);
                assert_eq!(else_block.len(), 1);
            }
            _ => panic!("expected if statement"),
        }
    }

    #[test]
    fn parse_mixed_precedence() {
        let src = "fn main(x, y) -> Int { let z = x + 1 < y; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = &program.functions[0];

        match &f.body[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "z");
                match value {
                    Expr::Binary { op, left, right } => {
                        match op {
                            BinOp::Lt => (),
                            _ => panic!("expected Lt op"),
                        }
                        match &**left {
                            Expr::Binary { op: lop, left: lleft, right: _lright } => {
                                match lop {
                                    BinOp::Add => (),
                                    _ => panic!("expected Add as left op"),
                                }
                                // left operand of add should be local x
                                match &**lleft {
                                    Expr::Local(n) => assert_eq!(n, "x"),
                                    _ => panic!("expected local x"),
                                }
                            }
                            _ => panic!("expected binary left side"),
                        }
                        match &**right {
                            Expr::Local(n) => assert_eq!(n, "y"),
                            _ => panic!("expected local y"),
                        }
                    }
                    _ => panic!("expected binary value"),
                }
            }
            _ => panic!("expected let statement"),
        }
    }

    #[test]
    fn parse_parenthesized_complex() {
        let src = "fn main(a,b,c,d) -> Int { let r = (a + b) <= (c - d); }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = &program.functions[0];

        match &f.body[0] {
            Stmt::Let { name, value } => {
                assert_eq!(name, "r");
                match value {
                    Expr::Binary { op, left, right } => {
                        match op {
                            BinOp::Le => (),
                            _ => panic!("expected Le op"),
                        }
                    }
                    _ => panic!("expected binary value"),
                }
            }
            _ => panic!("expected let statement"),
        }
    }
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

    fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
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
        // optional return type
        let return_type = if self.peek().kind == TokenKind::Arrow {
            self.bump();
            // currently only Int type supported
            self.expect(TokenKind::IntType)?;
            Some(crate::ast::Type::Int)
        } else {
            None
        };
        let body = self.parse_block()?;

        Ok(FunctionDecl { name, params, body, return_type })
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

        // Allow `return;` with no expression. If present, parse an expression.
        if self.peek().kind == TokenKind::Semicolon {
            self.bump();
            Ok(Stmt::Return(None))
        } else {
            let expr = self.parse_expr()?;
            self.expect(TokenKind::Semicolon)?;
            Ok(Stmt::Return(Some(expr)))
        }
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

        self.parse_binary_expr(0)
    }

    // Precedence-climbing / Pratt-style binary expression parser
    // min_prec: minimum precedence to accept in this call
    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        // 1. parse left-hand side primary
        let mut lhs = self.parse_primary()?;

        loop {
            // 2. check if next token is a binary operator and get its precedence
            let (op, prec) = match self.token_to_binop_prec(&self.peek().kind) {
                Some(x) => x,
                None => break,
            };

            if prec < min_prec {
                break;
            }

            // 3. consume operator
            self.bump();

            // 4. parse RHS with higher precedence (left-associative operators use prec+1)
            let next_min = prec + 1;
            let rhs = self.parse_binary_expr(next_min)?;

            // 5. build binary node and continue
            lhs = Expr::Binary {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    // Map a token kind to (BinOp, precedence). Higher number = higher precedence.
    // Precedence levels (high -> low): * / = 4, + - = 3, < > <= >= = 2, == != = 1
    fn token_to_binop_prec(&self, kind: &TokenKind) -> Option<(BinOp, u8)> {
        match kind {
            TokenKind::Star => Some((BinOp::Mul, 4)),
            TokenKind::Slash => Some((BinOp::Div, 4)),
            TokenKind::Plus => Some((BinOp::Add, 3)),
            TokenKind::Minus => Some((BinOp::Sub, 3)),
            TokenKind::Less => Some((BinOp::Lt, 2)),
            TokenKind::LessEqual => Some((BinOp::Le, 2)),
            TokenKind::Greater => Some((BinOp::Gt, 2)),
            TokenKind::GreaterEqual => Some((BinOp::Ge, 2)),
            TokenKind::EqualEqual => Some((BinOp::Eq, 1)),
            TokenKind::NotEqual => Some((BinOp::NotEq, 1)),
            _ => None,
        }
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

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    ExpectedExpression { span: Span },
}
