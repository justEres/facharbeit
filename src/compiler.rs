use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ast::{Program, TopLevelDecl};
use crate::codegen::module::{CodegenError, ModuleGen};
use crate::lexer::{LexError, lex_file};
use crate::parser::{ParseError, Parser};
use crate::token::Token;
use crate::typing::{TypedProgram, check_program};

/// Compiled artifacts produced by the compiler frontend + backend pipeline.
#[derive(Debug)]
pub struct CompileArtifacts {
    /// Token stream emitted by the lexer.
    pub tokens: Vec<Token>,
    /// Parsed AST program.
    pub program: Program,
    /// Typed program metadata.
    pub typed: TypedProgram,
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
    /// Type checking failed.
    TypeCheck(crate::typing::TypeError),
    /// Code generation failed.
    Codegen(CodegenError),
}

impl Display for CompileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Lex(e) => write!(f, "lex error: {}", e),
            CompileError::Parse(e) => write!(f, "parse error: {}", e),
            CompileError::TypeCheck(e) => write!(f, "type check error: {}", e),
            CompileError::Codegen(e) => write!(f, "codegen error: {}", e),
        }
    }
}

impl Error for CompileError {}

/// Compile source code to WebAssembly bytes and keep intermediate artifacts.
pub fn compile_source(src: &str) -> Result<CompileArtifacts, CompileError> {
    let (tokens, program, typed) = parse_and_check(src)?;

    let mut module_gen = ModuleGen::new().init_with_host_functions();

    for item in &program.items {
        if let TopLevelDecl::Function(func) = item {
            module_gen
                .declare_function(func, &typed.function_infos[&func.name])
                .map_err(CompileError::Codegen)?;
        }
    }

    for item in &program.items {
        if let TopLevelDecl::Function(func) = item {
            let func_info = typed
                .function_infos
                .get(&func.name)
                .expect("type info must exist after checking");
            module_gen
                .emit_function(func, func_info)
                .map_err(CompileError::Codegen)?;
        }
    }

    let main_param_count = program
        .items
        .iter()
        .find_map(|item| match item {
            TopLevelDecl::Function(func) if func.name == "main" => Some(func.params.len()),
            _ => None,
        })
        .unwrap_or(0);

    Ok(CompileArtifacts {
        tokens,
        program,
        typed,
        bytes: module_gen.finish(),
        main_param_count,
    })
}

/// Compile only through parse+typecheck. Useful for `--check` and frontend validation.
pub fn compile_source_check(src: &str) -> Result<CompileArtifacts, CompileError> {
    let (tokens, program, typed) = parse_and_check(src)?;

    let main_param_count = program
        .items
        .iter()
        .find_map(|item| match item {
            TopLevelDecl::Function(func) if func.name == "main" => Some(func.params.len()),
            _ => None,
        })
        .unwrap_or(0);

    Ok(CompileArtifacts {
        tokens,
        program,
        typed,
        bytes: Vec::new(),
        main_param_count,
    })
}

fn parse_and_check(src: &str) -> Result<(Vec<Token>, Program, TypedProgram), CompileError> {
    let tokens = lex_file(src).map_err(CompileError::Lex)?;
    let mut parser = Parser::new(&tokens);
    let program = parser.parse_program().map_err(CompileError::Parse)?;
    let typed = check_program(&program).map_err(CompileError::TypeCheck)?;
    Ok((tokens, program, typed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn compile_source_tracks_main_arity() {
        let src = "fn helper(x: Int) -> Int { return x; } fn main(a: Int, b: Int, c: Int) -> Int { return a; }";
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
            CompileError::TypeCheck(crate::typing::TypeError::DuplicateFunction { name }) => {
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

    #[test]
    fn compile_rejects_untyped_function_parameters() {
        let src = "fn main(x, y) -> Int { return x; }";
        let err = compile_source(src).expect_err("expected parse error");
        match err {
            CompileError::Parse(_) => {}
            _ => panic!("expected parse error"),
        }
    }

    fn read_example(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|e| panic!("unable to read {}: {}", path, e))
    }

    #[test]
    fn compile_examples_check_only() {
        let checks = [
            "examples/check_refs_enums.eres",
            "examples/check_aggregates.eres",
            "examples/check_match.eres",
        ];

        for path in checks {
            let src = read_example(path);
            compile_source_check(&src).expect(&format!("expected frontend check to pass for {}", path));
        }
    }

    #[test]
    fn compile_examples_expect_frontend_codegen_boundary() {
        let checks = [
            "examples/check_refs_enums.eres",
            "examples/check_aggregates.eres",
            "examples/check_match.eres",
        ];

        for path in checks {
            let src = read_example(path);
            let err = compile_source(&src)
                .expect_err(&format!("expected codegen boundary for {}", path));
            match err {
                CompileError::Codegen(_) => {}
                _ => panic!("expected Codegen error for {}", path),
            }
        }
    }
}
