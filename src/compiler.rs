use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ast::Program;
use crate::codegen::module::{CodegenError, ModuleGen};
use crate::lexer::{LexError, lex_file};
use crate::parser::{ParseError, Parser};
use crate::token::Token;

/// Compiled artifacts produced by the compiler frontend + backend pipeline.
#[derive(Debug)]
pub struct CompileArtifacts {
    /// Token stream emitted by the lexer.
    pub tokens: Vec<Token>,
    /// Parsed AST program.
    pub program: Program,
    /// Generated WebAssembly module bytes.
    pub bytes: Vec<u8>,
    /// Number of parameters expected by `main`.
    pub main_param_count: usize,
}

/// Structured compile errors to keep frontend failures distinct.
#[derive(Debug)]
pub enum CompileError {
    /// Lexing failed.
    Lex(LexError),
    /// Parsing failed.
    Parse(ParseError),
    /// Code generation failed.
    Codegen(CodegenError),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Lex(e) => write!(f, "lex error: {}", e),
            CompileError::Parse(e) => write!(f, "parse error: {}", e),
            CompileError::Codegen(e) => write!(f, "codegen error: {}", e),
        }
    }
}

impl Error for CompileError {}

/// Compile source code to WebAssembly bytes and keep intermediate artifacts.
pub fn compile_source(src: &str) -> Result<CompileArtifacts, CompileError> {
    let tokens = lex_file(src).map_err(CompileError::Lex)?;
    let mut parser = Parser::new(&tokens);
    let program = parser.parse_program().map_err(CompileError::Parse)?;

    // Register host imports first, then declare and emit functions.
    let mut module_gen = ModuleGen::new().init_with_host_functions();
    for func in &program.functions {
        module_gen
            .declare_function(func)
            .map_err(CompileError::Codegen)?;
    }
    for func in &program.functions {
        module_gen.emit_function(func).map_err(CompileError::Codegen)?;
    }

    let main_param_count = program
        .functions
        .iter()
        .find(|f| f.name == "main")
        .map(|f| f.params.len())
        .unwrap_or(0);

    Ok(CompileArtifacts {
        tokens,
        program,
        bytes: module_gen.finish(),
        main_param_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_source_tracks_main_arity() {
        let src = "fn helper(x) -> Int { return x; } fn main(a, b, c) -> Int { return a; }";
        let out = compile_source(src).expect("compile failed");
        assert_eq!(out.main_param_count, 3);
        assert!(!out.bytes.is_empty());
        assert!(!out.tokens.is_empty());
    }

    #[test]
    fn compile_source_rejects_duplicate_function_names() {
        let src = "fn main() -> Int { return 1; } fn main() -> Int { return 2; }";
        let err = compile_source(src).expect_err("expected duplicate-function error");
        match err {
            CompileError::Codegen(CodegenError::DuplicateFunction { name }) => {
                assert_eq!(name, "main")
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn generated_wat_contains_main_export() {
        let src = "fn main() -> Int { return 5; }";
        let out = compile_source(src).expect("compile failed");
        let wat = wasmprinter::print_bytes(&out.bytes).expect("wat conversion failed");
        assert!(wat.contains("(export \"main\" (func"));
        assert!(wat.contains("(func (;"));
    }
}
