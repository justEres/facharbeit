use std::fs;

use clap::Parser;
use wasmtime::{Engine, Instance, Store};

use crate::codegen::module::ModuleGen;

mod ast;
mod codegen;
mod lexer;
mod parser;
mod token;

#[derive(Parser)]
struct Args {
    input: String,
}

//Buchstabensuppeverarbeitungsmaschine

fn main() {
    let args = Args::parse();

    let src = std::fs::read_to_string(&args.input).expect("Failed to read input file");

    let bytes = match lexer::lex_file(&src) {
        Ok(tokens) => {
            let mut parser = parser::Parser::new(&tokens);
            let ast = parser.parse_program().unwrap();

            dbg!(&ast);

            let mut module_gen = ModuleGen::new();

            module_gen.declare_function(&ast.functions[0]);

            module_gen.emit_function(&ast.functions[0]);

            let bytes = module_gen.finish();

            Some(bytes)
        }
        Err(e) => {
            lexer::report_lex_error(&src, e);
            None
        }
    };

    //run

    if let Some(bytes) = bytes {
        let wat = wasmprinter::print_bytes(&bytes).unwrap();
        println!("Generated WAT:\n{}", wat);

        let engine = Engine::default();
        let module = wasmtime::Module::from_binary(&engine, &bytes).unwrap();

        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();

        let main = instance
            .get_typed_func::<(i64, i64), i64>(&mut store, "main")
            .unwrap();

        let result = main.call(&mut store, (64, 8)).unwrap();

        println!("result of main function: {}", result)
    }
}
