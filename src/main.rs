use clap::Parser;

use crate::compiler::CompileError;
use crate::lexer::report_lex_error;
use crate::parser::report_parse_error;

mod ast;
mod codegen;
mod compiler;
mod diagnostics;
mod lexer;
mod parser;
mod runner;
mod token;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source file
    input: String,

    /// Print tokens produced by the lexer
    #[arg(long, default_value_t = false)]
    print_tokens: bool,

    /// Print the parsed AST
    #[arg(long, default_value_t = false)]
    print_ast: bool,

    /// Print generated WAT before running
    #[arg(long, default_value_t = false)]
    print_wat: bool,
}

fn main() {
    let args = Args::parse();

    let src = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    let compile_out = match compiler::compile_source(&src) {
        Ok(out) => out,
        Err(CompileError::Lex(e)) => {
            report_lex_error(&src, e);
            return;
        }
        Err(CompileError::Parse(e)) => {
            report_parse_error(&src, &e);
            return;
        }
        Err(CompileError::Codegen(e)) => {
            eprintln!("CodegenError: {}", e);
            return;
        }
    };

    if args.print_tokens {
        println!("Tokens:\n{:#?}", compile_out.tokens);
    }
    if args.print_ast {
        println!("AST:\n{:#?}", compile_out.program);
    }

    let wat = wasmprinter::print_bytes(&compile_out.bytes).unwrap();
    if args.print_wat {
        println!("Generated WAT:\n{}", wat);
    }

    // Prepare zero-initialized i64 args matching the function's param count.
    let args: Vec<i64> = vec![0; compile_out.main_param_count];

    match runner::run_wasm_bytes(&compile_out.bytes, args) {
        Ok(Some(result)) => println!("result of main function: {}", result),
        Ok(None) => println!("main returned no value"),
        Err(e) => eprintln!("Execution error: {}", e),
    }
}
