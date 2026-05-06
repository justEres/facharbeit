pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;
pub mod token;

#[cfg(not(target_arch = "wasm32"))]
pub mod runner;

use codegen::module::ModuleGen;
use lexer::LexError;
use parser::ParseError;

pub struct CompilationArtifacts {
    pub tokens_json: String,
    pub ast_json: String,
    pub tokens: String,
    pub ast: String,
    pub wat: String,
    pub wasm_bytes: Vec<u8>,
    pub main_param_count: usize,
}

pub fn compile_source_details(src: &str) -> Result<CompilationArtifacts, String> {
    let tokens = lexer::lex_file(src).map_err(format_lex_error)?;
    let tokens_json = serde_json::to_string(&tokens)
        .map_err(|error| format!("failed to serialize tokens: {error}"))?;
    let tokens_debug = format!("{:#?}", tokens);

    let mut parser = parser::Parser::new(&tokens);
    let ast = parser.parse_program().map_err(format_parse_error)?;
    let ast_json =
        serde_json::to_string(&ast).map_err(|error| format!("failed to serialize AST: {error}"))?;
    let ast_debug = format!("{:#?}", ast);

    let mut module_gen = ModuleGen::new().init_with_host_functions();
    for func in &ast.functions {
        module_gen.declare_function(func);
    }
    for func in &ast.functions {
        module_gen.emit_function(func);
    }

    let wasm_bytes = module_gen.finish();
    let wat = wasmprinter::print_bytes(&wasm_bytes)
        .map_err(|error| format!("failed to convert generated Wasm to WAT: {error}"))?;

    let main_param_count = ast
        .functions
        .iter()
        .find(|func| func.name == "main")
        .map(|func| func.params.len())
        .unwrap_or(0);

    Ok(CompilationArtifacts {
        tokens: tokens_debug,
        ast: ast_debug,
        tokens_json,
        ast_json,
        wat,
        wasm_bytes,
        main_param_count,
    })
}

pub fn compile_to_wasm_bytes(src: &str) -> Result<Vec<u8>, String> {
    Ok(compile_source_details(src)?.wasm_bytes)
}

fn format_lex_error(error: LexError) -> String {
    match error {
        LexError::UnexpectedChar { ch, span } => {
            format!(
                "lex error: unexpected character '{ch}' at {}:{}",
                span.start, span.end
            )
        }
        LexError::InvalidNumber { span } => {
            format!("lex error: invalid number at {}:{}", span.start, span.end)
        }
    }
}

fn format_parse_error(error: ParseError) -> String {
    match error {
        ParseError::UnexpectedToken { expected, found } => {
            format!(
                "parse error: expected {expected}, found {:?} at {}:{}",
                found.kind, found.span.start, found.span.end
            )
        }
        ParseError::ExpectedExpression { span } => {
            format!(
                "parse error: expected expression at {}:{}",
                span.start, span.end
            )
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use wasm_bindgen::prelude::*;

    use crate::compile_source_details;

    #[wasm_bindgen(getter_with_clone)]
    pub struct PlaygroundCompileResult {
        pub tokens_json: String,
        pub ast_json: String,
        pub tokens: String,
        pub ast: String,
        pub wat: String,
        wasm_bytes: Vec<u8>,
        main_param_count: usize,
    }

    #[wasm_bindgen]
    impl PlaygroundCompileResult {
        #[wasm_bindgen(getter)]
        pub fn wasm_bytes(&self) -> Vec<u8> {
            self.wasm_bytes.clone()
        }

        #[wasm_bindgen(getter)]
        pub fn main_param_count(&self) -> usize {
            self.main_param_count
        }
    }

    #[wasm_bindgen(start)]
    pub fn start() {
        console_error_panic_hook::set_once();
    }

    #[wasm_bindgen]
    pub fn compile_playground(src: &str) -> Result<PlaygroundCompileResult, JsValue> {
        let result = compile_source_details(src).map_err(|error| JsValue::from_str(&error))?;

        Ok(PlaygroundCompileResult {
            tokens_json: result.tokens_json,
            ast_json: result.ast_json,
            tokens: result.tokens,
            ast: result.ast,
            wat: result.wat,
            wasm_bytes: result.wasm_bytes,
            main_param_count: result.main_param_count,
        })
    }
}
