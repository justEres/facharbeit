use std::collections::HashMap;

use crate::ast::{Param, Program, TopLevelDecl, Type};
use crate::compiler::CompileError;
use crate::lexer::{LexError, lex_file};
use crate::parser::{ParseError, Parser};
use crate::token::{Span, Token, TokenKind};
use crate::typing::{TypedProgram, check_program};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Local,
    Struct,
    Enum,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub detail: String,
    pub span: Span,
}

#[derive(Debug)]
pub struct CheckedSource {
    pub tokens: Vec<Token>,
    pub program: Program,
    pub typed: TypedProgram,
}

#[derive(Debug)]
pub struct AnalysisDiagnostic {
    pub message: String,
    pub span: Option<Span>,
}

#[derive(Debug)]
pub struct FunctionSymbol {
    pub name: String,
    pub name_span: Span,
    pub body_span: Span,
    pub signature: String,
}

#[derive(Debug)]
pub struct SymbolIndex {
    functions: HashMap<String, FunctionSymbol>,
    structs: HashMap<String, Span>,
    enums: HashMap<String, Span>,
}

pub fn parse(source: &str) -> Result<Program, CompileError> {
    let tokens = lex_file(source).map_err(CompileError::Lex)?;
    let mut parser = Parser::new(&tokens);
    parser.parse_program().map_err(CompileError::Parse)
}

pub fn check(source: &str) -> Result<CheckedSource, CompileError> {
    let tokens = lex_file(source).map_err(CompileError::Lex)?;
    let mut parser = Parser::new(&tokens);
    let program = parser.parse_program().map_err(CompileError::Parse)?;
    let typed = check_program(&program).map_err(CompileError::TypeCheck)?;
    Ok(CheckedSource {
        tokens,
        program,
        typed,
    })
}

pub fn analyze_diagnostic(source: &str) -> Option<AnalysisDiagnostic> {
    match check(source) {
        Ok(_) => None,
        Err(CompileError::Lex(err)) => Some(AnalysisDiagnostic {
            message: err.to_string(),
            span: Some(lex_error_span(&err)),
        }),
        Err(CompileError::Parse(err)) => Some(AnalysisDiagnostic {
            message: err.to_string(),
            span: Some(parse_error_span(&err)),
        }),
        Err(CompileError::TypeCheck(err)) => Some(AnalysisDiagnostic {
            message: err.to_string(),
            span: None,
        }),
        Err(CompileError::Codegen(_)) => None,
    }
}

pub fn symbol_at(source: &str, offset: usize) -> Result<Option<SymbolInfo>, CompileError> {
    let checked = check(source)?;
    let token = checked
        .tokens
        .iter()
        .find(|token| matches!(token.kind, TokenKind::Ident(_)) && span_contains(&token.span, offset));

    let Some(token) = token else {
        return Ok(None);
    };

    let TokenKind::Ident(name) = &token.kind else {
        return Ok(None);
    };

    let index = SymbolIndex::from_tokens(&checked.tokens, &checked.program);
    if let Some(function_name) = index.function_name_for_offset(offset) {
        if let Some(function) = checked.typed.function_infos.get(function_name) {
            if let Some((_, ty)) = function.local_map.get(name) {
                return Ok(Some(SymbolInfo {
                    name: name.clone(),
                    kind: SymbolKind::Local,
                    detail: format!("{}: {}", name, ty),
                    span: token.span.clone(),
                }));
            }
        }
    }

    if let Some(function) = index.functions.get(name) {
        return Ok(Some(SymbolInfo {
            name: name.clone(),
            kind: SymbolKind::Function,
            detail: function.signature.clone(),
            span: token.span.clone(),
        }));
    }

    if index.structs.contains_key(name) {
        return Ok(Some(SymbolInfo {
            name: name.clone(),
            kind: SymbolKind::Struct,
            detail: format!("struct {}", name),
            span: token.span.clone(),
        }));
    }

    if index.enums.contains_key(name) {
        return Ok(Some(SymbolInfo {
            name: name.clone(),
            kind: SymbolKind::Enum,
            detail: format!("enum {}", name),
            span: token.span.clone(),
        }));
    }

    Ok(None)
}

impl SymbolIndex {
    pub fn from_tokens(tokens: &[Token], program: &Program) -> Self {
        let mut functions = HashMap::new();
        let mut structs = HashMap::new();
        let mut enums = HashMap::new();
        let mut function_signatures = HashMap::new();

        for item in &program.items {
            if let TopLevelDecl::Function(function) = item {
                function_signatures.insert(function.name.clone(), format_function_signature(
                    &function.name,
                    &function.params,
                    &function.return_type,
                ));
            }
        }

        let mut idx = 0usize;
        while idx < tokens.len() {
            match &tokens[idx].kind {
                TokenKind::Fn => {
                    if let Some((name, name_span, body_span)) = scan_function(tokens, idx) {
                        let signature = function_signatures
                            .get(&name)
                            .cloned()
                            .unwrap_or_else(|| format!("fn {}", name));
                        functions.insert(
                            name.clone(),
                            FunctionSymbol {
                                name,
                                name_span,
                                body_span,
                                signature,
                            },
                        );
                    }
                }
                TokenKind::Struct => {
                    if let Some((name, span)) = next_ident(tokens, idx + 1) {
                        structs.insert(name, span);
                    }
                }
                TokenKind::Enum => {
                    if let Some((name, span)) = next_ident(tokens, idx + 1) {
                        enums.insert(name, span);
                    }
                }
                _ => {}
            }
            idx += 1;
        }

        Self {
            functions,
            structs,
            enums,
        }
    }

    fn function_name_for_offset(&self, offset: usize) -> Option<&str> {
        self.functions.values().find_map(|function| {
            if span_contains(&function.body_span, offset) {
                Some(function.name.as_str())
            } else {
                None
            }
        })
    }
}

fn scan_function(tokens: &[Token], fn_idx: usize) -> Option<(String, Span, Span)> {
    let (name, name_span) = next_ident(tokens, fn_idx + 1)?;
    let body_start_idx = tokens[fn_idx..]
        .iter()
        .position(|token| token.kind == TokenKind::LBrace)
        .map(|rel| fn_idx + rel)?;
    let body_end_idx = find_matching_brace(tokens, body_start_idx)?;
    let body_span = Span {
        start: tokens[body_start_idx].span.start,
        end: tokens[body_end_idx].span.end,
    };
    Some((name, name_span, body_span))
}

fn find_matching_brace(tokens: &[Token], open_idx: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (idx, token) in tokens.iter().enumerate().skip(open_idx) {
        match token.kind {
            TokenKind::LBrace => depth += 1,
            TokenKind::RBrace => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(idx);
                }
            }
            _ => {}
        }
    }
    None
}

fn next_ident(tokens: &[Token], start_idx: usize) -> Option<(String, Span)> {
    tokens.iter().skip(start_idx).find_map(|token| match &token.kind {
        TokenKind::Ident(name) => Some((name.clone(), token.span.clone())),
        _ => None,
    })
}

fn format_function_signature(name: &str, params: &[Param], return_type: &Type) -> String {
    let params = params
        .iter()
        .map(|param| format!("{}: {}", param.name, param.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("fn {}({}) -> {}", name, params, return_type)
}

fn span_contains(span: &Span, offset: usize) -> bool {
    span.start <= offset && offset < span.end
}

fn lex_error_span(error: &LexError) -> Span {
    match error {
        LexError::UnexpectedChar { span, .. } | LexError::InvalidNumber { span } => span.clone(),
    }
}

fn parse_error_span(error: &ParseError) -> Span {
    match error {
        ParseError::UnexpectedToken { found, .. } => found.span.clone(),
        ParseError::ExpectedExpression { span } => span.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{SymbolKind, analyze_diagnostic, check, symbol_at};

    #[test]
    fn hover_finds_local_variable_type() {
        let src = "fn main(x: Int) -> Int { let y = x; return y; }";
        let offset = src.rfind("y").expect("missing y");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Local);
        assert_eq!(symbol.detail, "y: Int");
    }

    #[test]
    fn hover_finds_function_signature() {
        let src = "fn inc(x: Int) -> Int { return x; } fn main() -> Int { return inc(1); }";
        let offset = src.rfind("inc").expect("missing inc");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Function);
        assert_eq!(symbol.detail, "fn inc(x: Int) -> Int");
    }

    #[test]
    fn diagnostic_reports_parse_error() {
        let diagnostic = analyze_diagnostic("fn main( -> Int { return 1; }").expect("expected diagnostic");
        assert!(diagnostic.message.contains("unexpected token"));
        assert!(diagnostic.span.is_some());
    }

    #[test]
    fn check_returns_typed_program() {
        let checked = check("fn main() -> Int { return 1; }").expect("check failed");
        assert!(checked.typed.function_infos.contains_key("main"));
    }
}
