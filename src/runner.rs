use wasmtime::{Engine, Instance, Store, Val};

/// Run wasm bytes calling `main` with the provided i64 arguments.
/// Returns Ok(Some(i64)) if the function returns a single i64, Ok(None) if
/// the function has no return, or Err on failure.
pub fn run_wasm_bytes(bytes: &[u8], args: Vec<i64>) -> Result<Option<i64>, String> {
    let engine = Engine::default();
    let module = wasmtime::Module::from_binary(&engine, bytes)
        .map_err(|e| format!("module compile error: {}", e))?;

    let mut store = Store::new(&engine, ());

    // create host print function
    let print_func = wasmtime::Func::wrap(&mut store, |v: i64| {
        println!("{}", v);
    });

    let instance = Instance::new(&mut store, &module, &[print_func.into()])
        .map_err(|e| format!("instance error: {}", e))?;

    let func = instance
        .get_func(&mut store, "main")
        .ok_or_else(|| "function `main` not found".to_string())?;

    // Prepare Val parameters
    let params: Vec<Val> = args.into_iter().map(Val::I64).collect();

    // Inspect function type to determine result count
    let ty = func.ty(&store);
    let results = ty.results().len();

    let mut results_buf: Vec<Val> = vec![Val::I64(0); results];

    func.call(&mut store, &params, &mut results_buf)
        .map_err(|e| format!("runtime error: {}", e))?;

    if results == 1 {
        match results_buf[0] {
            Val::I64(v) => Ok(Some(v)),
            Val::I32(v) => Ok(Some(v as i64)),
            _ => Err("unexpected return value type".to_string()),
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::compile_source;
    use std::fs;
    use std::sync::{Arc, Mutex};

    fn compile_bytes_from_src(src: &str) -> Result<Vec<u8>, String> {
        compile_source(src).map(|out| out.bytes).map_err(|e| e.to_string())
    }

    fn run_source(src: &str, args: Vec<i64>) -> Result<Option<i64>, String> {
        let bytes = compile_bytes_from_src(src)?;
        run_wasm_bytes(&bytes, args)
    }

    #[test]
    fn run_if_gt_sample() {
        let src = "fn main(x: Int, y: Int) -> Int { if (x > 5) { return 1; } else { return 0; } }";
        let res = run_source(src, vec![64, 8]).expect("run failed");
        assert_eq!(res, Some(1));
    }

    #[test]
    fn run_add_compare() {
        let src = "fn main(a: Int, b: Int) -> Bool { return a + 1 < b; }";
        // a=1, b=3 -> 1+1 < 3 -> true -> 1
        let res = run_source(src, vec![1, 3]).expect("run failed");
        assert_eq!(res, Some(1));
    }

    #[test]
    fn run_parenthesized_complex() {
        let src = "fn main(a: Int, b: Int, c: Int, d: Int) -> Bool { let r = (a + b) <= (c - d); return r; }";
        // Wrong arity should return an error.
        let _res =
            run_source(src, vec![1, 1]).expect_err("expected error because missing c,d args");

        let src2 = "fn main(a: Int, b: Int, c: Int, d: Int) -> Bool { let r = (a + b) <= (c - d); return r; }";
        let bytes = compile_bytes_from_src(src2).expect("compile failed");
        let out =
            run_wasm_bytes(&bytes, vec![2, 1]).expect_err("expected error due to wrong arg count");
        let _ = out;
    }

    fn read_example(path: &str) -> String {
        fs::read_to_string(path).unwrap_or_else(|e| panic!("unable to read {}: {}", path, e))
    }

    #[test]
    fn run_examples_runtime() {
        let cases = vec![
            ("examples/run_arith.eres", vec![2, 4, 6], Some(12)),
            ("examples/run_float_cond.eres", vec![], Some(1)),
        ];

        for (path, args, expected) in cases {
            let src = read_example(path);
            let bytes = compile_source(&src).map(|out| out.bytes).expect("compile failed");
            let out = run_wasm_bytes(&bytes, args).expect("execution failed");
            assert_eq!(out, expected, "failed runtime example {}", path);
        }
    }

    #[test]
    fn print_statement_outputs() {
        // inline source instead of reading a file
        let src = "fn main() -> Int { print(3); return 0; }";

        let bytes = compile_bytes_from_src(src).expect("compile failed");

        // instantiate module with a host `print_i64` that records values
        let engine = Engine::default();
        let module = wasmtime::Module::from_binary(&engine, &bytes).expect("module compile failed");

        let printed: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(Vec::new()));

        let mut store = Store::new(&engine, ());
        let captured = printed.clone();
        let print_func = wasmtime::Func::wrap(&mut store, move |v: i64| {
            captured.lock().unwrap().push(v);
        });

        let instance = Instance::new(&mut store, &module, &[print_func.into()])
            .expect("instance creation failed");

        let func = instance
            .get_func(&mut store, "main")
            .expect("main not found");
        let params: Vec<Val> = Vec::new();
        let result_count = func
            .ty(&mut store)
            .results()
            .len()
            .try_into()
            .expect("result count conversion");
        let mut results_buf: Vec<Val> = vec![Val::I64(0); result_count];

        func.call(&mut store, &params, &mut results_buf)
            .expect("call failed");

        let got = printed.lock().unwrap().clone();
        assert_eq!(got, vec![3]);
    }

    #[test]
    fn compile_fails_on_unknown_local() {
        let src = "fn main() -> Int { return missing; }";
        let err = compile_bytes_from_src(src).expect_err("expected unknown local error");
        assert!(err.contains("unknown variable"));
    }

    #[test]
    fn compile_fails_on_unknown_function_call() {
        let src = "fn main() -> Int { foo(); return 0; }";
        let err = compile_bytes_from_src(src).expect_err("expected unknown function error");
        assert!(err.contains("unknown function `foo`"));
    }

    #[test]
    fn run_if_else_control_flow() {
        let src = "fn main() -> Int { if false { return 1; } else { return 7; } }";
        let res = run_source(src, vec![]).expect("run failed");
        assert_eq!(res, Some(7));
    }

    #[test]
    fn run_while_skips_body_when_false() {
        let src = "fn main() -> Int { while false { return 1; } return 2; }";
        let res = run_source(src, vec![]).expect("run failed");
        assert_eq!(res, Some(2));
    }

    #[test]
    fn run_early_return_in_if() {
        let src = "fn main() -> Int { if true { return 9; } return 1; }";
        let res = run_source(src, vec![]).expect("run failed");
        assert_eq!(res, Some(9));
    }
}
