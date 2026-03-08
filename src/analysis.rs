use std::collections::HashMap;

use crate::ast::{EnumDecl, EnumVariant, Param, Program, StructDecl, TopLevelDecl, Type};
use crate::compiler::CompileError;
use crate::lexer::{LexError, lex_file};
use crate::parser::{ParseError, Parser};
use crate::token::{Span, Token, TokenKind};
use crate::typing::{TypedProgram, check_program};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Parameter,
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

#[derive(Debug, Clone)]
pub struct DefinitionInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub target_span: Span,
}

#[derive(Debug, Clone)]
pub struct CompletionItemInfo {
    pub label: String,
    pub kind: SymbolKind,
    pub detail: String,
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
    pub full_span: Span,
    pub body_span: Span,
    pub params: HashMap<String, Span>,
    pub locals: HashMap<String, Span>,
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
    let token = find_ident_at_offset(&checked.tokens, offset);

    let Some(token) = token else {
        return Ok(None);
    };

    let TokenKind::Ident(name) = &token.kind else {
        return Ok(None);
    };

    let index = SymbolIndex::from_tokens(&checked.tokens, &checked.program);
    if let Some(function_name) = index.function_name_for_offset(offset) {
        if let Some(function) = checked.typed.function_infos.get(function_name) {
            if let Some((slot, ty)) = function.local_map.get(name) {
                let kind = if (*slot as usize) < function.params.len() {
                    SymbolKind::Parameter
                } else {
                    SymbolKind::Local
                };
                return Ok(Some(SymbolInfo {
                    name: name.clone(),
                    kind,
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
            detail: checked
                .typed
                .structs
                .get(name)
                .map(format_struct_decl)
                .unwrap_or_else(|| format!("struct {}", name)),
            span: token.span.clone(),
        }));
    }

    if index.enums.contains_key(name) {
        return Ok(Some(SymbolInfo {
            name: name.clone(),
            kind: SymbolKind::Enum,
            detail: checked
                .typed
                .enums
                .get(name)
                .map(format_enum_decl)
                .unwrap_or_else(|| format!("enum {}", name)),
            span: token.span.clone(),
        }));
    }

    Ok(None)
}

pub fn definition_at(source: &str, offset: usize) -> Result<Option<DefinitionInfo>, CompileError> {
    let checked = check(source)?;
    let token = find_ident_at_offset(&checked.tokens, offset);
    let Some(token) = token else {
        return Ok(None);
    };

    let TokenKind::Ident(name) = &token.kind else {
        return Ok(None);
    };

    let index = SymbolIndex::from_tokens(&checked.tokens, &checked.program);

    if let Some(function_name) = index.function_name_for_offset(offset)
        && let Some(function) = index.functions.get(function_name)
    {
        if let Some(span) = function.params.get(name) {
            return Ok(Some(DefinitionInfo {
                name: name.clone(),
                kind: SymbolKind::Parameter,
                target_span: span.clone(),
            }));
        }
        if let Some(span) = function.locals.get(name) {
            return Ok(Some(DefinitionInfo {
                name: name.clone(),
                kind: SymbolKind::Local,
                target_span: span.clone(),
            }));
        }
    }

    if let Some(function) = index.functions.get(name) {
        return Ok(Some(DefinitionInfo {
            name: name.clone(),
            kind: SymbolKind::Function,
            target_span: function.name_span.clone(),
        }));
    }

    if let Some(span) = index.structs.get(name) {
        return Ok(Some(DefinitionInfo {
            name: name.clone(),
            kind: SymbolKind::Struct,
            target_span: span.clone(),
        }));
    }

    if let Some(span) = index.enums.get(name) {
        return Ok(Some(DefinitionInfo {
            name: name.clone(),
            kind: SymbolKind::Enum,
            target_span: span.clone(),
        }));
    }

    Ok(None)
}

pub fn completions_at(source: &str, offset: usize) -> Result<Vec<CompletionItemInfo>, CompileError> {
    let checked = check(source)?;
    let index = SymbolIndex::from_tokens(&checked.tokens, &checked.program);
    let mut items = Vec::new();

    if let Some(function_name) = index.function_name_for_offset(offset)
        && let Some(function) = checked.typed.function_infos.get(function_name)
    {
        for param in &function.params {
            items.push(CompletionItemInfo {
                label: param.name.clone(),
                kind: SymbolKind::Parameter,
                detail: format!("{}: {}", param.name, param.ty),
            });
        }
        for (name, ty) in &function.locals {
            items.push(CompletionItemInfo {
                label: name.clone(),
                kind: SymbolKind::Local,
                detail: format!("{}: {}", name, ty),
            });
        }
    }

    for function in index.functions.values() {
        items.push(CompletionItemInfo {
            label: function.name.clone(),
            kind: SymbolKind::Function,
            detail: function.signature.clone(),
        });
    }

    for (name, def) in &checked.typed.structs {
        items.push(CompletionItemInfo {
            label: name.clone(),
            kind: SymbolKind::Struct,
            detail: format_struct_decl(def),
        });
    }

    for (name, def) in &checked.typed.enums {
        items.push(CompletionItemInfo {
            label: name.clone(),
            kind: SymbolKind::Enum,
            detail: format_enum_decl(def),
        });
    }

    items.sort_by(|left, right| left.label.cmp(&right.label).then(left.detail.cmp(&right.detail)));
    items.dedup_by(|left, right| left.label == right.label && left.kind == right.kind);
    Ok(items)
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
                    if let Some(symbol) = scan_function(tokens, idx) {
                        let signature = function_signatures
                            .get(&symbol.name)
                            .cloned()
                            .unwrap_or_else(|| format!("fn {}", symbol.name));
                        functions.insert(symbol.name.clone(), FunctionSymbol {
                            signature,
                            ..symbol
                        });
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
            if span_contains(&function.full_span, offset) {
                Some(function.name.as_str())
            } else {
                None
            }
        })
    }
}

fn scan_function(tokens: &[Token], fn_idx: usize) -> Option<FunctionSymbol> {
    let (name, name_span) = next_ident(tokens, fn_idx + 1)?;
    let body_start_idx = tokens[fn_idx..]
        .iter()
        .position(|token| token.kind == TokenKind::LBrace)
        .map(|rel| fn_idx + rel)?;
    let body_end_idx = find_matching_brace(tokens, body_start_idx)?;
    let full_span = Span {
        start: tokens[fn_idx].span.start,
        end: tokens[body_end_idx].span.end,
    };
    let body_span = Span {
        start: tokens[body_start_idx].span.start,
        end: tokens[body_end_idx].span.end,
    };
    Some(FunctionSymbol {
        name,
        name_span,
        full_span: full_span.clone(),
        body_span,
        params: scan_params(tokens, fn_idx, body_start_idx),
        locals: scan_locals(tokens, body_start_idx, body_end_idx),
        signature: String::new(),
    })
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

fn find_ident_at_offset(tokens: &[Token], offset: usize) -> Option<&Token> {
    tokens.iter().find(|token| {
        matches!(token.kind, TokenKind::Ident(_))
            && (span_contains(&token.span, offset)
                || (token.span.end == offset && token.span.start < token.span.end))
    })
}

fn scan_params(tokens: &[Token], fn_idx: usize, body_start_idx: usize) -> HashMap<String, Span> {
    let mut params = HashMap::new();
    let mut paren_depth = 0usize;

    for token_idx in fn_idx..body_start_idx {
        match &tokens[token_idx].kind {
            TokenKind::LParen => paren_depth += 1,
            TokenKind::RParen => paren_depth = paren_depth.saturating_sub(1),
            TokenKind::Ident(name)
                if paren_depth == 1
                    && tokens
                        .get(token_idx + 1)
                        .is_some_and(|next| next.kind == TokenKind::Colon) =>
            {
                params.insert(name.clone(), tokens[token_idx].span.clone());
            }
            _ => {}
        }
    }

    params
}

fn scan_locals(tokens: &[Token], body_start_idx: usize, body_end_idx: usize) -> HashMap<String, Span> {
    let mut locals = HashMap::new();

    for token_idx in body_start_idx..body_end_idx {
        if tokens[token_idx].kind == TokenKind::Let
            && let Some(token) = tokens.get(token_idx + 1)
            && let TokenKind::Ident(name) = &token.kind
        {
            locals.insert(name.clone(), token.span.clone());
        }
    }

    locals
}

fn format_function_signature(name: &str, params: &[Param], return_type: &Type) -> String {
    let params = params
        .iter()
        .map(|param| format!("{}: {}", param.name, param.ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("fn {}({}) -> {}", name, params, return_type)
}

fn format_struct_decl(def: &StructDecl) -> String {
    let fields = def
        .fields
        .iter()
        .map(|(name, ty)| format!("{}: {}", name, ty))
        .collect::<Vec<_>>()
        .join(", ");
    format!("struct {} {{ {} }}", def.name, fields)
}

fn format_enum_decl(def: &EnumDecl) -> String {
    let variants = def
        .variants
        .iter()
        .map(format_enum_variant)
        .collect::<Vec<_>>()
        .join(", ");
    format!("enum {} {{ {} }}", def.name, variants)
}

fn format_enum_variant(variant: &EnumVariant) -> String {
    match variant {
        EnumVariant::Unit(name) => name.clone(),
        EnumVariant::Tuple(name, ty) => format!("{}({})", name, ty),
        EnumVariant::Struct(name, fields) => {
            let fields = fields
                .iter()
                .map(|(field, ty)| format!("{}: {}", field, ty))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{} {{ {} }}", name, fields)
        }
    }
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
    use super::{SymbolKind, analyze_diagnostic, check, completions_at, definition_at, symbol_at};

    #[test]
    fn hover_finds_local_variable_type() {
        let src = "fn main(x: Int) -> Int { let y = x; return y; }";
        let offset = src.rfind("y").expect("missing y");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Local);
        assert_eq!(symbol.detail, "y: Int");
    }

    #[test]
    fn hover_marks_parameters_separately() {
        let src = "fn main(x: Int) -> Int { return x; }";
        let offset = src.find("x: Int").expect("missing x");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Parameter);
        assert_eq!(symbol.detail, "x: Int");
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
    fn hover_formats_structs() {
        let src = "struct Point { x: Int, y: Int } fn main() -> Int { return 0; }";
        let offset = src.find("Point").expect("missing Point");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Struct);
        assert_eq!(symbol.detail, "struct Point { x: Int, y: Int }");
    }

    #[test]
    fn hover_formats_enums() {
        let src = "enum Result { Ok, Err(Int) } fn main() -> Int { return 0; }";
        let offset = src.find("Result").expect("missing Result");
        let symbol = symbol_at(src, offset).expect("analysis failed").expect("missing symbol");
        assert_eq!(symbol.kind, SymbolKind::Enum);
        assert_eq!(symbol.detail, "enum Result { Ok, Err(Int) }");
    }

    #[test]
    fn definition_finds_local_declaration() {
        let src = "fn main(x: Int) -> Int { let y = x; return y; }";
        let offset = src.rfind("y").expect("missing y");
        let definition = definition_at(src, offset)
            .expect("analysis failed")
            .expect("missing definition");
        assert_eq!(definition.kind, SymbolKind::Local);
        assert_eq!(&src[definition.target_span.start..definition.target_span.end], "y");
        assert_eq!(src.find("let y").expect("missing let y") + 4, definition.target_span.start);
    }

    #[test]
    fn definition_finds_function_name() {
        let src = "fn inc(x: Int) -> Int { return x; } fn main() -> Int { return inc(1); }";
        let offset = src.rfind("inc").expect("missing inc");
        let definition = definition_at(src, offset)
            .expect("analysis failed")
            .expect("missing definition");
        assert_eq!(definition.kind, SymbolKind::Function);
        assert_eq!(src.find("inc").expect("missing definition inc"), definition.target_span.start);
    }

    #[test]
    fn completions_include_scope_and_globals() {
        let src = "struct Point { x: Int } fn helper(v: Int) -> Int { return v; } fn main(x: Int) -> Int { let y = x; return y; }";
        let offset = src.find("return y").expect("missing return y");
        let completions = completions_at(src, offset).expect("analysis failed");
        assert!(completions.iter().any(|item| item.label == "x" && item.kind == SymbolKind::Parameter));
        assert!(completions.iter().any(|item| item.label == "y" && item.kind == SymbolKind::Local));
        assert!(completions.iter().any(|item| item.label == "helper" && item.kind == SymbolKind::Function));
        assert!(completions.iter().any(|item| item.label == "Point" && item.kind == SymbolKind::Struct));
    }

    #[test]
    fn check_returns_typed_program() {
        let checked = check("fn main() -> Int { return 1; }").expect("check failed");
        assert!(checked.typed.function_infos.contains_key("main"));
    }
}
