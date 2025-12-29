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

            module_gen.declare_function(&ast.functions[0]);
            module_gen.emit_function(&ast.functions[0]);

            let bytes = module_gen.finish();

            Some((bytes, args.print_wat))
        }
        Err(e) => {
            lexer::report_lex_error(&src, e);
            None
        }
    };

    // run
    if let Some((bytes, print_wat)) = bytes {
        let wat = wasmprinter::print_bytes(&bytes).unwrap();
        if print_wat {
            println!("Generated WAT:\n{}", wat);
        }

        match runner::run_wasm_bytes(&bytes, (64, 8)) {
            Ok(result) => println!("result of main function: {}", result),
            Err(e) => eprintln!("Execution error: {}", e),
        }
    }
}
