use std::collections::BTreeSet;

use crate::ast::{Expr, MatchArm, Program, Stmt, TopLevelDecl};

pub const STRING_EQ_IMPORT: &str = "__eres_string_eq";
const STRING_LITERAL_PREFIX: &str = "__eres_const_string_";

#[derive(Debug, Default, Clone)]
pub struct RuntimeImports {
    pub string_literals: Vec<String>,
    pub needs_string_eq: bool,
}

pub fn string_eq_import_name() -> &'static str {
    STRING_EQ_IMPORT
}

pub fn string_literal_import_name(value: &str) -> String {
    format!("{}{}", STRING_LITERAL_PREFIX, hex_encode(value.as_bytes()))
}

pub fn decode_string_literal_import_name(name: &str) -> Option<String> {
    let payload = name.strip_prefix(STRING_LITERAL_PREFIX)?;
    let bytes = hex_decode(payload)?;
    String::from_utf8(bytes).ok()
}

pub fn collect_runtime_imports(program: &Program) -> RuntimeImports {
    let mut imports = RuntimeImports::default();
    let mut strings = BTreeSet::new();

    for item in &program.items {
        if let TopLevelDecl::Function(func) = item {
            for stmt in &func.body {
                visit_stmt(stmt, &mut strings, &mut imports.needs_string_eq);
            }
        }
    }

    imports.string_literals = strings.into_iter().collect();
    imports
}

fn visit_stmt(stmt: &Stmt, strings: &mut BTreeSet<String>, needs_string_eq: &mut bool) {
    match stmt {
        Stmt::Let { value, .. } => visit_expr(value, strings, needs_string_eq),
        Stmt::Return(Some(expr)) | Stmt::Expr(expr) => visit_expr(expr, strings, needs_string_eq),
        Stmt::Return(None) => {}
        Stmt::If {
            cond,
            then_block,
            else_block,
        } => {
            visit_expr(cond, strings, needs_string_eq);
            for stmt in then_block {
                visit_stmt(stmt, strings, needs_string_eq);
            }
            for stmt in else_block {
                visit_stmt(stmt, strings, needs_string_eq);
            }
        }
        Stmt::While { cond, body } => {
            visit_expr(cond, strings, needs_string_eq);
            for stmt in body {
                visit_stmt(stmt, strings, needs_string_eq);
            }
        }
    }
}

fn visit_expr(expr: &Expr, strings: &mut BTreeSet<String>, needs_string_eq: &mut bool) {
    match expr {
        Expr::String(value) => {
            strings.insert(value.clone());
        }
        Expr::Binary { op, left, right } => {
            if matches!(op, crate::ast::BinOp::Eq | crate::ast::BinOp::NotEq) {
                *needs_string_eq = true;
            }
            visit_expr(left, strings, needs_string_eq);
            visit_expr(right, strings, needs_string_eq);
        }
        Expr::Call { args, .. } | Expr::MethodCall { args, .. } => {
            for arg in args {
                visit_expr(arg, strings, needs_string_eq);
            }
        }
        Expr::StructInit { fields, .. } => {
            for (_, expr) in fields {
                visit_expr(expr, strings, needs_string_eq);
            }
        }
        Expr::EnumInit { payload, .. } | Expr::TupleLiteral(payload) | Expr::ListLiteral(payload) => {
            for expr in payload {
                visit_expr(expr, strings, needs_string_eq);
            }
        }
        Expr::Match { subject, arms } => {
            visit_expr(subject, strings, needs_string_eq);
            for MatchArm { body, .. } in arms {
                visit_expr(body, strings, needs_string_eq);
            }
        }
        Expr::Index { base, index } => {
            visit_expr(base, strings, needs_string_eq);
            visit_expr(index, strings, needs_string_eq);
        }
        Expr::Ref(inner) | Expr::Deref(inner) => visit_expr(inner, strings, needs_string_eq),
        Expr::Local(_) | Expr::Int(_) | Expr::Float(_) | Expr::Bool(_) => {}
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(char::from_digit((byte >> 4) as u32, 16).expect("hex digit"));
        out.push(char::from_digit((byte & 0x0f) as u32, 16).expect("hex digit"));
    }
    out
}

fn hex_decode(input: &str) -> Option<Vec<u8>> {
    if input.len() % 2 != 0 {
        return None;
    }

    let mut out = Vec::with_capacity(input.len() / 2);
    let chars = input.as_bytes();
    let mut idx = 0;
    while idx < chars.len() {
        let high = char::from(chars[idx]).to_digit(16)?;
        let low = char::from(chars[idx + 1]).to_digit(16)?;
        out.push(((high << 4) | low) as u8);
        idx += 2;
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinOp, Expr, FunctionDecl, Program, Stmt, TopLevelDecl, Type};

    #[test]
    fn runtime_imports_collect_strings_and_eq() {
        let program = Program {
            items: vec![TopLevelDecl::Function(FunctionDecl {
                name: "main".to_string(),
                params: vec![],
                return_type: Type::Bool,
                body: vec![Stmt::Return(Some(Expr::Binary {
                    op: BinOp::Eq,
                    left: Box::new(Expr::String("a".to_string())),
                    right: Box::new(Expr::String("b".to_string())),
                }))],
            })],
        };
        let imports = collect_runtime_imports(&program);
        assert!(imports.needs_string_eq);
        assert_eq!(imports.string_literals, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn string_literal_import_name_roundtrips() {
        let name = string_literal_import_name("hi\n");
        assert_eq!(decode_string_literal_import_name(&name), Some("hi\n".to_string()));
    }
}
