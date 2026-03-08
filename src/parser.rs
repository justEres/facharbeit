use crate::ast::*;
use crate::diagnostics::render_snippet;
use crate::token::*;
use std::fmt::{Display, Formatter};

/// Recursive-descent parser over lexer tokens.
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
    fn parse_use_decl() {
        let src = "use \"./math.eres\";";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        match &program.items[0] {
            TopLevelDecl::Use(path) => assert_eq!(path, "./math.eres"),
            _ => panic!("expected use declaration"),
        }
    }

    #[test]
    fn parse_struct_decl() {
        let src = "struct Point { x: Int, y: Int }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        assert_eq!(program.items.len(), 1);
        match &program.items[0] {
            TopLevelDecl::Struct(def) => {
                assert_eq!(def.name, "Point");
                assert_eq!(def.fields.len(), 2);
            }
            _ => panic!("expected struct"),
        }
    }

    #[test]
    fn parse_enum_decl() {
        let src = "enum Status { Ok, Err(Int), Data { msg: Int } }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        match &program.items[0] {
            TopLevelDecl::Enum(def) => {
                assert_eq!(def.name, "Status");
                assert_eq!(def.variants.len(), 3);
            }
            _ => panic!("expected enum"),
        }
    }

    #[test]
    fn parse_paren_comparison() {
        let src = "fn main(x: Int, y: Int) -> Int { if (x < y) { return 1; } else { return 0; } }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");

        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");
        assert_eq!(f.body.len(), 1);

        match &f.body[0] {
            Stmt::If {
                cond,
                then_block,
                else_block,
            } => {
                match cond {
                    Expr::Binary {
                        op: BinOp::Lt, ..
                    } => {}
                    _ => panic!("expected Lt op"),
                }

                assert_eq!(then_block.len(), 1);
                assert_eq!(else_block.len(), 1);
            }
            _ => panic!("expected if statement"),
        }
    }

    #[test]
    fn parse_mixed_precedence() {
        let src = "fn main(x: Int, y: Int) -> Int { let z = x + 1 < y; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        match &f.body[0] {
            Stmt::Let { name, value, .. } => {
                assert_eq!(name, "z");
                match value {
                    Expr::Binary { op, left, right } => {
                        match op {
                            BinOp::Lt => (),
                            _ => panic!("expected Lt op"),
                        }
                        match (&**left, &**right) {
                            (
                                Expr::Binary {
                                    op: lop,
                                    left: lleft,
                                    right: _,
                                },
                                Expr::Local(_name),
                            ) => {
                                match lop {
                                    BinOp::Add => (),
                                    _ => panic!("expected Add"),
                                }
                                match &**lleft {
                                    Expr::Local(n) => assert_eq!(n, "x"),
                                    _ => panic!("expected local x"),
                                }
                            }
                            _ => panic!("unexpected shape"),
                        }
                    }
                    _ => panic!("expected binary value"),
                }
            }
            _ => panic!("expected let statement"),
        }
    }

    #[test]
    fn parse_return_without_expression() {
        let src = "fn main() -> Int { return; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");
        assert_eq!(f.body.len(), 1);
        match &f.body[0] {
            Stmt::Return(None) => {}
            _ => panic!("expected bare return"),
        }
    }

    #[test]
    fn parse_function_type_annotation() {
        let src =
            "fn map(f: fn(Int) -> Int, x: Int) -> Int { return f(x); }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        assert_eq!(f.params[0].ty, Type::Function(vec![Type::Int], Box::new(Type::Int)));
    }

    #[test]
    fn parse_list_and_tuple_types() {
        let src =
            "fn f(x: List<Int>, y: (Int, Float)) -> List<List<Float>> { return [1.0, 2.0]; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        assert_eq!(f.params[0].ty, Type::List(Box::new(Type::Int)));
        assert_eq!(f.params[1].ty, Type::Tuple(vec![Type::Int, Type::Float]));
        assert_eq!(
            f.return_type,
            Type::List(Box::new(Type::List(Box::new(Type::Float))))
        );
    }

    #[test]
    fn parse_list_and_tuple_literals() {
        let src = "fn f() -> Int { let xs = [1, 2, 3]; let t = (1, true); let s = \"hi\"; return 1; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        match &f.body[0] {
            Stmt::Let { value: Expr::ListLiteral(items), .. } => {
                assert_eq!(items.len(), 3);
            }
            _ => panic!("expected list literal"),
        }
        match &f.body[1] {
            Stmt::Let { value: Expr::TupleLiteral(items), .. } => {
                assert_eq!(items.len(), 2);
            }
            _ => panic!("expected tuple literal"),
        }
        match &f.body[2] {
            Stmt::Let { value: Expr::String(value), .. } => assert_eq!(value, "hi"),
            _ => panic!("expected string literal"),
        }
    }

    #[test]
    fn parse_match_expression() {
        let src =
            "fn f(x: Int) -> Int { let y: Int = match x { A => 1 }; return y; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");
        match &f.body[0] {
            Stmt::Let { value: Expr::Match { .. }, .. } => {}
            _ => panic!("expected match expression"),
        }
    }

    #[test]
    fn parse_list_access_and_dot_index() {
        let src = "fn f(xs: List<Int>, x: (Int, Float)) -> Int { return xs[0] + x.1; }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        match &f.body[0] {
            Stmt::Return(Some(expr)) => match expr {
                Expr::Binary {
                    left,
                    right,
                    ..
                } => {
                    match &**left {
                        Expr::Index { .. } => {}
                        _ => panic!("expected list index on left"),
                    }
                    match &**right {
                        Expr::Index { .. } => {}
                        _ => panic!("expected tuple-style dot index on right"),
                    }
                }
                _ => panic!("expected binary return expression"),
            },
            _ => panic!("expected return statement"),
        }
    }

    #[test]
    fn parse_list_method_call() {
        let src = "fn f(xs: List<Int>) -> Int { return xs.len(); }";
        let tokens = lex_file(src).expect("lex");
        let mut p = Parser::new(&tokens);
        let program = p.parse_program().expect("parse");
        let f = program
            .items
            .iter()
            .find_map(|i| match i {
                TopLevelDecl::Function(func) => Some(func),
                _ => None,
            })
            .expect("function missing");

        match &f.body[0] {
            Stmt::Return(Some(Expr::MethodCall { name, .. })) => {
                assert_eq!(name, "len");
            }
            _ => panic!("expected method call"),
        }
    }
}

impl<'a> Parser<'a> {
    /// Creates a parser over the token stream.
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

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        let tok = self.bump();
        match tok.kind {
            TokenKind::Ident(s) => Ok(s),
            _ => Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: tok,
            }),
        }
    }

    /// Parses a complete program until `Eof`.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();

        while self.peek().kind != TokenKind::Eof {
            let item = match self.peek().kind {
                TokenKind::Use => self.parse_use()?,
                TokenKind::Struct => self.parse_struct()?,
                TokenKind::Enum => self.parse_enum()?,
                TokenKind::Fn => TopLevelDecl::Function(self.parse_function()?),
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "use|fn|struct|enum".to_string(),
                        found: self.peek().clone(),
                    })
                }
            };
            items.push(item);
        }

        Ok(Program { items })
    }

    fn parse_use(&mut self) -> Result<TopLevelDecl, ParseError> {
        self.expect(TokenKind::Use)?;
        let path = match self.bump().kind {
            TokenKind::StringLit(path) => path,
            other => {
                return Err(ParseError::UnexpectedToken {
                    expected: "string literal".to_string(),
                    found: Token {
                        kind: other,
                        span: self.tokens[self.pos - 1].span.clone(),
                    },
                });
            }
        };
        self.expect(TokenKind::Semicolon)?;
        Ok(TopLevelDecl::Use(path))
    }

    fn parse_struct(&mut self) -> Result<TopLevelDecl, ParseError> {
        self.expect(TokenKind::Struct)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LBrace)?;

        let mut fields = Vec::new();
        if self.peek().kind != TokenKind::RBrace {
            loop {
                let fname = self.expect_ident()?;
                self.expect(TokenKind::Colon)?;
                let fty = self.parse_type()?;
                fields.push((fname, fty));

                if self.peek().kind == TokenKind::Comma {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(TopLevelDecl::Struct(StructDecl { name, fields }))
    }

    fn parse_enum(&mut self) -> Result<TopLevelDecl, ParseError> {
        self.expect(TokenKind::Enum)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LBrace)?;

        let mut variants = Vec::new();
        if self.peek().kind != TokenKind::RBrace {
            loop {
                let variant_name = self.expect_ident()?;
                let variant = if self.peek().kind == TokenKind::LParen {
                    self.bump();
                    let inner = self.parse_type()?;
                    self.expect(TokenKind::RParen)?;
                    EnumVariant::Tuple(variant_name, inner)
                } else if self.peek().kind == TokenKind::LBrace {
                    self.bump();
                    let mut fields = Vec::new();
                    if self.peek().kind != TokenKind::RBrace {
                        loop {
                            let field_name = self.expect_ident()?;
                            self.expect(TokenKind::Colon)?;
                            let field_ty = self.parse_type()?;
                            fields.push((field_name, field_ty));

                            if self.peek().kind == TokenKind::Comma {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RBrace)?;
                    EnumVariant::Struct(variant_name, fields)
                } else {
                    EnumVariant::Unit(variant_name)
                };

                variants.push(variant);

                if self.peek().kind == TokenKind::Comma {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(TopLevelDecl::Enum(EnumDecl { name, variants }))
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
        self.expect(TokenKind::Fn)?;

        let name = self.expect_ident()?;

        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let pname = self.expect_ident()?;
                self.expect(TokenKind::Colon)?;
                let pty = self.parse_type()?;
                params.push(Param { name: pname, ty: pty });

                if self.peek().kind == TokenKind::Comma {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Arrow)?;
        let return_type = self.parse_type()?;
        let body = self.parse_block()?;

        Ok(FunctionDecl {
            name,
            params,
            body,
            return_type,
        })
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

        let name = self.expect_ident()?;
        let ty = if self.peek().kind == TokenKind::Colon {
            self.bump();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;

        Ok(Stmt::Let { name, ty, value })
    }

    fn parse_return(&mut self) -> Result<Stmt, ParseError> {
        self.expect(TokenKind::Return)?;

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

    // Precedence-climbing parser.
    fn parse_binary_expr(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_unary()?;

        while let Some((op, prec)) = self.token_to_binop_prec(&self.peek().kind) {
            if prec < min_prec {
                break;
            }

            self.bump();

            let next_min = prec + 1;
            let rhs = self.parse_binary_expr(next_min)?;

            lhs = Expr::Binary {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek().kind {
            TokenKind::Ampersand => {
                self.bump();
                Ok(Expr::Ref(Box::new(self.parse_unary()?)))
            }
            TokenKind::Star => {
                self.bump();
                Ok(Expr::Deref(Box::new(self.parse_unary()?)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;

        while let TokenKind::LBracket | TokenKind::Dot = &self.peek().kind {
            match self.peek().kind {
                TokenKind::LBracket => {
                    self.bump();
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::Index {
                        base: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                TokenKind::Dot => {
                    self.bump();
                    let token = self.bump().clone();
                    match token.kind {
                        TokenKind::Int(index) => {
                            expr = Expr::Index {
                                base: Box::new(expr),
                                index: Box::new(Expr::Int(index)),
                            };
                        }
                        TokenKind::Ident(name) => {
                            if self.peek().kind != TokenKind::LParen {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "method call".to_string(),
                                    found: self.peek().clone(),
                                });
                            }
                            self.bump();
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
                            expr = Expr::MethodCall {
                                receiver: Box::new(expr),
                                name,
                                args,
                            };
                        }
                        _ => {
                            return Err(ParseError::UnexpectedToken {
                                expected: "integer index or method name".to_string(),
                                found: token,
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(expr)
    }

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
            TokenKind::Float(value) => Ok(Expr::Float(value)),
            TokenKind::True => Ok(Expr::Bool(true)),
            TokenKind::False => Ok(Expr::Bool(false)),
            TokenKind::StringLit(value) => Ok(Expr::String(value)),

            TokenKind::Ident(name) => {
                if self.peek().kind == TokenKind::DoubleColon {
                    self.bump();
                    let variant = self.expect_ident()?;

                    if self.peek().kind == TokenKind::LParen {
                        self.bump();
                        let mut payload = Vec::new();
                        if self.peek().kind != TokenKind::RParen {
                            loop {
                                payload.push(self.parse_expr()?);
                                if self.peek().kind == TokenKind::Comma {
                                    self.bump();
                                } else {
                                    break;
                                }
                            }
                        }
                        self.expect(TokenKind::RParen)?;
                        Ok(Expr::EnumInit {
                            enum_name: name,
                            variant,
                            payload,
                        })
                    } else {
                        Ok(Expr::EnumInit {
                            enum_name: name,
                            variant,
                            payload: Vec::new(),
                        })
                    }
                } else if self.peek().kind == TokenKind::LParen {
                    self.bump();
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
                } else if self.peek().kind == TokenKind::LBrace && self.looks_like_struct_init() {
                    self.bump();
                    let mut fields = Vec::new();
                    if self.peek().kind != TokenKind::RBrace {
                        loop {
                            let field_name = self.expect_ident()?;
                            self.expect(TokenKind::Colon)?;
                            let field_value = self.parse_expr()?;
                            fields.push((field_name, field_value));

                            if self.peek().kind == TokenKind::Comma {
                                self.bump();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RBrace)?;
                    Ok(Expr::StructInit { name, fields })
                } else {
                    Ok(Expr::Local(name))
                }
            }

            TokenKind::LParen => {
                let first = self.parse_expr()?;
                if self.peek().kind == TokenKind::Comma {
                    let mut elements = vec![first];
                    while self.peek().kind == TokenKind::Comma {
                        self.bump();
                        elements.push(self.parse_expr()?);
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::TupleLiteral(elements))
                } else {
                    self.expect(TokenKind::RParen)?;
                    Ok(first)
                }
            }

            TokenKind::LBracket => {
                let mut elements = Vec::new();
                if self.peek().kind != TokenKind::RBracket {
                    loop {
                        elements.push(self.parse_expr()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(TokenKind::RBracket)?;
                Ok(Expr::ListLiteral(elements))
            }

            TokenKind::Match => {
                let subject = self.parse_expr()?;
                self.expect(TokenKind::LBrace)?;

                let mut arms = Vec::new();
                while self.peek().kind != TokenKind::RBrace {
                    let pattern = self.parse_pattern()?;
                    self.expect(TokenKind::FatArrow)?;
                    let body = self.parse_expr()?;
                    arms.push(MatchArm { pattern, body });

                    if self.peek().kind == TokenKind::Comma {
                        self.bump();
                    } else {
                        break;
                    }
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Expr::Match {
                    subject: Box::new(subject),
                    arms,
                })
            }

            _ => Err(ParseError::ExpectedExpression { span: tok.span }),
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let variant = self.expect_ident()?;

        let variant_pattern = if self.peek().kind == TokenKind::LParen {
            self.bump();
            let mut vars = Vec::new();
            if self.peek().kind != TokenKind::RParen {
                loop {
                    let var = self.expect_ident()?;
                    vars.push(var);
                    if self.peek().kind == TokenKind::Comma {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
            self.expect(TokenKind::RParen)?;
            Pattern::TupleVariant(variant, vars)
        } else if self.peek().kind == TokenKind::LBrace {
            self.bump();
            let mut vars = Vec::new();
            if self.peek().kind != TokenKind::RBrace {
                loop {
                    let var = self.expect_ident()?;
                    vars.push(var);
                    if self.peek().kind == TokenKind::Comma {
                        self.bump();
                    } else {
                        break;
                    }
                }
            }
            self.expect(TokenKind::RBrace)?;
            Pattern::StructVariant(variant, vars)
        } else {
            Pattern::UnitVariant(variant)
        };

        Ok(variant_pattern)
    }

    fn looks_like_struct_init(&self) -> bool {
        if self.peek().kind != TokenKind::LBrace {
            return false;
        }

        let mut i = self.pos;

        i += 1;
        let len = self.tokens.len();
        if i >= len {
            return false;
        }

        if self.tokens.get(i).is_some_and(|t| t.kind == TokenKind::RBrace) {
            return true;
        }

        // Struct literals require at least one `field: value` entry.
        let first = self.tokens.get(i);
        if !matches!(first, Some(tok) if matches!(tok.kind, TokenKind::Ident(_))) {
            return false;
        }

        i += 1;
        while i < len {
            if self.tokens.get(i).is_none() {
                return false;
            }
            let kind = self.tokens.get(i).unwrap().kind.clone();
            if kind == TokenKind::Colon {
                return true;
            }

            if kind == TokenKind::Comma || kind == TokenKind::RBrace
            {
                return false;
            }
            i += 1;
        }

        false
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.peek().kind.clone() {
            TokenKind::Fn => {
                self.bump();
                self.expect(TokenKind::LParen)?;
                let mut args = Vec::new();

                if self.peek().kind != TokenKind::RParen {
                    loop {
                        args.push(self.parse_type()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }

                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::Arrow)?;
                let ret = self.parse_type()?;
                Ok(Type::Function(args, Box::new(ret)))
            }
            TokenKind::IntType => {
                self.bump();
                Ok(Type::Int)
            }
            TokenKind::FloatType => {
                self.bump();
                Ok(Type::Float)
            }
            TokenKind::BoolType => {
                self.bump();
                Ok(Type::Bool)
            }
            TokenKind::StringType => {
                self.bump();
                Ok(Type::String)
            }
            TokenKind::Ampersand => {
                self.bump();
                Ok(Type::Ref(Box::new(self.parse_type()?)))
            }
            TokenKind::LParen => {
                self.bump();
                let mut elements = vec![self.parse_type()?];
                if self.peek().kind == TokenKind::Comma {
                    self.bump();
                    while self.peek().kind != TokenKind::RParen {
                        elements.push(self.parse_type()?);
                        if self.peek().kind == TokenKind::Comma {
                            self.bump();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RParen)?;
                if elements.len() == 1 {
                    Ok(elements.into_iter().next().unwrap())
                } else {
                    Ok(Type::Tuple(elements))
                }
            }
            TokenKind::Ident(name) => {
                let _ = self.bump();
                if name == "List" {
                    if self.peek().kind == TokenKind::Less {
                        self.bump();
                        let inner = self.parse_type()?;
                        self.expect(TokenKind::Greater)?;
                        Ok(Type::List(Box::new(inner)))
                    } else {
                        Ok(Type::Named(name))
                    }
                } else {
                    Ok(Type::Named(name))
                }
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "type".to_string(),
                found: self.peek().clone(),
            }),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    /// A specific token was expected but a different token was found.
    UnexpectedToken { expected: String, found: Token },
    /// Expression parsing was expected to start but failed.
    ExpectedExpression { span: Span },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "unexpected token: expected {}, found {}",
                    expected,
                    found.kind.name()
                )
            }
            ParseError::ExpectedExpression { .. } => write!(f, "expected expression"),
        }
    }
}

/// Prints a human-readable parser error with a source snippet.
pub fn report_parse_error(src: &str, error: &ParseError) {
    match error {
        ParseError::UnexpectedToken { expected, found } => {
            let snippet = render_snippet(src, &found.span);
            eprintln!(
                "ParseError [E-PS01] at line {}, column {}: expected {}, found {}\nhelp: check punctuation/order near this token\n{}\n{}",
                snippet.line,
                snippet.column,
                expected,
                found.kind.name(),
                snippet.source_line,
                snippet.marker_line
            );
        }
        ParseError::ExpectedExpression { span } => {
            let snippet = render_snippet(src, span);
            eprintln!(
                "ParseError [E-PS02] at line {}, column {}: expected expression\nhelp: insert a literal, variable, call, or parenthesized expression\n{}\n{}",
                snippet.line,
                snippet.column,
                snippet.source_line,
                snippet.marker_line
            );
        }
    }
}
