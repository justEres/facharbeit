use clap::Parser;
use facharbeit::compile_source_details;
use facharbeit::runner::run_wasm_bytes;

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
    let result = match compile_source_details(&src) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    if args.print_tokens {
        println!("Tokens:\n{}", result.tokens);
    }

    if args.print_ast {
        println!("AST:\n{}", result.ast);
    }

    if args.print_wat {
        println!("Generated WAT:\n{}", result.wat);
    }

    let args: Vec<i64> = vec![0; result.main_param_count];

    match run_wasm_bytes(&result.wasm_bytes, args) {
        Ok(Some(value)) => println!("result of main function: {}", value),
        Ok(None) => println!("main returned no value"),
        Err(error) => eprintln!("Execution error: {}", error),
    }
}
