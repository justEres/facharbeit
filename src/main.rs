use clap::Parser;
// runner handles executing wasm; main doesn't need direct wasmtime imports anymore

use crate::codegen::module::ModuleGen;

mod ast;
mod codegen;
mod lexer;
mod parser;
mod token;
mod runner;

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

// Buchstabensuppeverarbeitungsmaschine

fn main() {
    let args = Args::parse();

    let src = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    let bytes = match lexer::lex_file(&src) {
        Ok(tokens) => {
            if args.print_tokens {
                println!("Tokens:\n{:#?}", tokens);
            }

            let mut parser = parser::Parser::new(&tokens);
            let ast = parser.parse_program().unwrap();

            if args.print_ast {
                println!("AST:\n{:#?}", ast);
            }

            let mut module_gen = ModuleGen::new();
            module_gen = module_gen.init_with_host_functions();

            // Declare and emit all functions in the input so cross-calls work.
            for func in &ast.functions {
                module_gen.declare_function(func);
            }
            for func in &ast.functions {
                module_gen.emit_function(func);
            }

            let bytes = module_gen.finish();

            // Determine the parameter count of the exported `main` function if it exists.
            let main_param_count = ast
                .functions
                .iter()
                .find(|f| f.name == "main")
                .map(|f| f.params.len())
                .unwrap_or(0);

            Some((bytes, args.print_wat, main_param_count))
        }
        Err(e) => {
            lexer::report_lex_error(&src, e);
            None
        }
    };

    // run
    if let Some((bytes, print_wat, param_count)) = bytes {
        let wat = wasmprinter::print_bytes(&bytes).unwrap();
        if print_wat {
            println!("Generated WAT:\n{}", wat);
        }

        // Prepare zero-initialized i64 args matching the function's param count.
        let args: Vec<i64> = vec![0; param_count];

        match runner::run_wasm_bytes(&bytes, args) {
            Ok(Some(result)) => println!("result of main function: {}", result),
            Ok(None) => println!("main returned no value"),
            Err(e) => eprintln!("Execution error: {}", e),
        }
    }
}
