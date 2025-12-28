use clap::Parser;

mod ast;
mod lexer;
mod parser;
mod token;
mod codegen;

#[derive(Parser)]
struct Args {
    input: String,
}

//Buchstabensuppeverarbeitungsmaschine

fn main() {
    let args = Args::parse();

    let src = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    match lexer::lex_file(&src) {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(&tokens);
            let ast = parser.parse_program();

            dbg!(ast);

        }
        Err(e) => {
            lexer::report_lex_error(&src, e);
        }
    }
}
