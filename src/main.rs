use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::compiler::CompileError;
use crate::compiler::CompileArtifacts;
use crate::compiler::compile_source;
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
    input: Option<String>,

    /// Start a minimal REPL (expects full program snippets per line)
    #[arg(long, default_value_t = false)]
    repl: bool,

    /// Print tokens produced by the lexer
    #[arg(long, default_value_t = false)]
    print_tokens: bool,

    /// Print the parsed AST
    #[arg(long, default_value_t = false)]
    print_ast: bool,

    /// Print generated WAT before running
    #[arg(long, default_value_t = false)]
    print_wat: bool,

    /// Only compile/check; do not execute `main`
    #[arg(long, default_value_t = false)]
    check: bool,

    /// Comma-separated i64 arguments for `main`, e.g. "1,2,3"
    #[arg(long)]
    args: Option<String>,

    /// Write token dump to a file
    #[arg(long)]
    emit_tokens: Option<PathBuf>,

    /// Write AST dump to a file
    #[arg(long)]
    emit_ast: Option<PathBuf>,

    /// Write generated WAT to a file
    #[arg(long)]
    emit_wat: Option<PathBuf>,

    /// Write generated wasm bytes to a file
    #[arg(long)]
    emit_wasm: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.repl {
        run_repl(&args);
        return;
    }

    let input = match &args.input {
        Some(path) => path,
        None => {
            eprintln!("Error: missing input file. Provide <input> or use --repl.");
            return;
        }
    };
    let src = std::fs::read_to_string(input).expect("Failed to read input file");

    let compile_out = match compile_source(&src) {
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
            eprintln!("CodegenError [E-CG01]: {}", e);
            return;
        }
    };

    let _ = handle_compiled_output(&args, &compile_out);
}

fn run_repl(args: &Args) {
    println!("REPL mode. Enter expressions or full programs. Commands: :quit, :help");
    let mut line = String::new();
    loop {
        print!("eres> ");
        let _ = io::stdout().flush();
        line.clear();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("I/O error: {}", e);
                break;
            }
        }
        let src = line.trim();
        if src.is_empty() {
            continue;
        }
        if src == ":quit" || src == ":q" {
            break;
        }
        if src == ":help" {
            println!("Expression: 1 + 2 * 3");
            println!("Program: fn main() -> Int {{ return 1; }}");
            continue;
        }

        let repl_src = normalize_repl_input(src);
        let compile_out = match compile_source(&repl_src) {
            Ok(out) => out,
            Err(CompileError::Lex(e)) => {
                report_lex_error(&repl_src, e);
                continue;
            }
            Err(CompileError::Parse(e)) => {
                report_parse_error(&repl_src, &e);
                continue;
            }
            Err(CompileError::Codegen(e)) => {
                eprintln!("CodegenError [E-CG01]: {}", e);
                continue;
            }
        };

        if args.check {
            println!("check ok");
            continue;
        }

        let run_args = match parse_cli_i64_args(args.args.as_deref(), compile_out.main_param_count) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Argument error [E-CLI01]: {}", e);
                continue;
            }
        };

        match runner::run_wasm_bytes(&compile_out.bytes, run_args) {
            Ok(Some(result)) => println!("= {}", result),
            Ok(None) => println!("(ok)"),
            Err(e) => eprintln!("Execution error [E-RT01]: {}", e),
        }
    }
}

fn handle_compiled_output(args: &Args, compile_out: &CompileArtifacts) -> bool {
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

    if let Some(path) = &args.emit_tokens {
        let data = format!("{:#?}\n", compile_out.tokens);
        if let Err(e) = std::fs::write(path, data) {
            eprintln!("Failed to write {}: {}", path.display(), e);
            return false;
        }
    }
    if let Some(path) = &args.emit_ast {
        let data = format!("{:#?}\n", compile_out.program);
        if let Err(e) = std::fs::write(path, data) {
            eprintln!("Failed to write {}: {}", path.display(), e);
            return false;
        }
    }
    if let Some(path) = &args.emit_wat && let Err(e) = std::fs::write(path, &wat) {
        eprintln!("Failed to write {}: {}", path.display(), e);
        return false;
    }
    if let Some(path) = &args.emit_wasm && let Err(e) = std::fs::write(path, &compile_out.bytes) {
        eprintln!("Failed to write {}: {}", path.display(), e);
        return false;
    }

    if args.check {
        println!("check ok");
        return true;
    }

    let run_args = match parse_cli_i64_args(args.args.as_deref(), compile_out.main_param_count) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Argument error [E-CLI01]: {}", e);
            return false;
        }
    };

    match runner::run_wasm_bytes(&compile_out.bytes, run_args) {
        Ok(Some(result)) => println!("result of main function: {}", result),
        Ok(None) => println!("main returned no value"),
        Err(e) => eprintln!("Execution error [E-RT01]: {}", e),
    }
    true
}

fn parse_cli_i64_args(raw: Option<&str>, default_len: usize) -> Result<Vec<i64>, String> {
    let Some(raw) = raw else {
        return Ok(vec![0; default_len]);
    };
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for (idx, part) in raw.split(',').enumerate() {
        let val = part.trim().parse::<i64>().map_err(|_| {
            format!(
                "failed to parse argument {} ('{}') as i64; use comma-separated integers",
                idx + 1,
                part.trim()
            )
        })?;
        out.push(val);
    }
    Ok(out)
}

fn normalize_repl_input(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.starts_with("fn ") || trimmed.starts_with("fn\t") {
        return trimmed.to_string();
    }

    let expr = trimmed.strip_suffix(';').unwrap_or(trimmed);
    format!("fn main() -> Int {{ return {}; }}", expr)
}

#[cfg(test)]
mod tests {
    use super::{normalize_repl_input, parse_cli_i64_args};

    #[test]
    fn parse_cli_i64_args_defaults_to_zeroed_main_arity() {
        assert_eq!(parse_cli_i64_args(None, 3).expect("parse failed"), vec![0, 0, 0]);
    }

    #[test]
    fn parse_cli_i64_args_parses_csv_values() {
        assert_eq!(
            parse_cli_i64_args(Some("1, -2,3"), 0).expect("parse failed"),
            vec![1, -2, 3]
        );
    }

    #[test]
    fn parse_cli_i64_args_reports_invalid_values() {
        let err = parse_cli_i64_args(Some("1,a"), 0).expect_err("expected parse error");
        assert!(err.contains("failed to parse argument 2"));
    }

    #[test]
    fn normalize_repl_input_wraps_expression() {
        let got = normalize_repl_input("1 + 2 * 3");
        assert_eq!(got, "fn main() -> Int { return 1 + 2 * 3; }");
    }

    #[test]
    fn normalize_repl_input_keeps_function_programs() {
        let src = "fn main() -> Int { return 7; }";
        assert_eq!(normalize_repl_input(src), src);
    }
}
