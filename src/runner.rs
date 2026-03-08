use std::collections::HashMap;

use crate::host::default_host_functions;
use crate::runtime::{decode_string_literal_import_name, string_eq_import_name};
use eres_abi::{
    AbiType, HostFunction, RuntimeHeap, RuntimeValue, abi_type_to_val_type, host_value_to_val,
    val_to_host_value,
};
use wasmtime::{Caller, Engine, ExternType, Func, FuncType, Instance, Store, Val};

/// Run wasm bytes calling `main` with the provided i64 arguments.
/// Returns Ok(Some(i64)) if the function returns a single i64, Ok(None) if
/// the function has no return, or Err on failure.
pub fn run_wasm_bytes(bytes: &[u8], args: Vec<i64>) -> Result<Option<i64>, String> {
    run_wasm_bytes_with_hosts(bytes, args, &default_host_functions())
}

pub fn run_wasm_bytes_with_hosts(
    bytes: &[u8],
    args: Vec<i64>,
    hosts: &[HostFunction],
) -> Result<Option<i64>, String> {
    let engine = Engine::default();
    let module = wasmtime::Module::from_binary(&engine, bytes)
        .map_err(|e| format!("module compile error: {}", e))?;

    let mut store = Store::new(&engine, RuntimeHeap::default());
    let imports = instantiate_host_imports(&mut store, &engine, &module, hosts)?;
    let imports_ref = imports.iter().cloned().map(Into::into).collect::<Vec<_>>();
    let instance = Instance::new(&mut store, &module, &imports_ref)
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

fn instantiate_host_imports(
    store: &mut Store<RuntimeHeap>,
    engine: &Engine,
    module: &wasmtime::Module,
    hosts: &[HostFunction],
) -> Result<Vec<Func>, String> {
    let host_map = hosts
        .iter()
        .cloned()
        .map(|host| (host.name, host))
        .collect::<HashMap<_, _>>();
    let mut funcs = Vec::new();

    for import in module.imports() {
        if import.module() != "env" {
            return Err(format!(
                "unsupported import module `{}` for `{}`",
                import.module(),
                import.name()
            ));
        }

        let ExternType::Func(_) = import.ty() else {
            return Err(format!("unsupported non-function import `{}`", import.name()));
        };

        let func = if import.name() == string_eq_import_name() {
            instantiate_string_eq_import(store, engine)
        } else if let Some(value) = decode_string_literal_import_name(import.name()) {
            instantiate_const_string_import(store, engine, value)
        } else {
            let host = host_map
                .get(import.name())
                .ok_or_else(|| format!("missing host import `{}`", import.name()))?;
            instantiate_registered_host(store, engine, host)
        }?;
        funcs.push(func);
    }

    Ok(funcs)
}

fn instantiate_registered_host(
    store: &mut Store<RuntimeHeap>,
    engine: &Engine,
    host: &HostFunction,
) -> Result<Func, String> {
    let ty = FuncType::new(
        engine,
        host.params.iter().filter_map(abi_type_to_val_type),
        match &host.result {
            AbiType::Unit => Vec::new(),
            other => vec![
                abi_type_to_val_type(other)
                    .ok_or_else(|| format!("unsupported host return type for `{}`", host.name))?,
            ],
        },
    );
    let host = host.clone();
    Ok(Func::new(
        &mut *store,
        ty,
        move |mut caller: Caller<'_, RuntimeHeap>, params, results| {
            let args = params
                .iter()
                .zip(host.params.iter())
                .map(|(value, ty)| val_to_host_value(value, ty))
                .collect::<Result<Vec<_>, _>>()
                .map_err(wasmtime::Error::msg)?;
            let value = (host.call)(caller.data_mut(), &args).map_err(wasmtime::Error::msg)?;
            if let Some(value) = value {
                results[0] =
                    host_value_to_val(value, &host.result).map_err(wasmtime::Error::msg)?;
            }
            Ok(())
        },
    ))
}

fn instantiate_const_string_import(
    store: &mut Store<RuntimeHeap>,
    engine: &Engine,
    value: String,
) -> Result<Func, String> {
    let ty = FuncType::new(engine, Vec::<wasmtime::ValType>::new(), [wasmtime::ValType::I32]);
    Ok(Func::new(
        &mut *store,
        ty,
        move |mut caller: Caller<'_, RuntimeHeap>, _params, results| {
            let handle = caller
                .data_mut()
                .allocate(RuntimeValue::String(value.clone()));
            results[0] = Val::I32(handle.0 as i32);
            Ok(())
        },
    ))
}

fn instantiate_string_eq_import(
    store: &mut Store<RuntimeHeap>,
    engine: &Engine,
) -> Result<Func, String> {
    let ty = FuncType::new(
        engine,
        [wasmtime::ValType::I32, wasmtime::ValType::I32],
        [wasmtime::ValType::I32],
    );
    Ok(Func::new(
        &mut *store,
        ty,
        move |caller: Caller<'_, RuntimeHeap>, params, results| {
            let left = match params.first() {
                Some(Val::I32(value)) => *value as u32,
                other => {
                    return Err(wasmtime::Error::msg(format!(
                        "invalid left string handle: {:?}",
                        other
                    )))
                }
            };
            let right = match params.get(1) {
                Some(Val::I32(value)) => *value as u32,
                other => {
                    return Err(wasmtime::Error::msg(format!(
                        "invalid right string handle: {:?}",
                        other
                    )))
                }
            };

            let heap = caller.data();
            let left = heap
                .get(eres_abi::RuntimeValueHandle(left))
                .map_err(wasmtime::Error::msg)?;
            let right = heap
                .get(eres_abi::RuntimeValueHandle(right))
                .map_err(wasmtime::Error::msg)?;

            let equals = match (left, right) {
                (RuntimeValue::String(a), RuntimeValue::String(b)) => a == b,
                other => {
                    return Err(wasmtime::Error::msg(format!(
                        "string equality expects String handles, found {:?}",
                        other
                    )))
                }
            };
            results[0] = Val::I32(if equals { 1 } else { 0 });
            Ok(())
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{compile_source, compile_source_with_hosts};
    use crate::host::default_host_functions;
    use eres_abi::{AbiType, EresAbi, HostFunction, HostValue, eres_host_function};
    use std::fs;
    use std::sync::{Arc, Mutex, OnceLock};

    static PRINT_CAPTURE: OnceLock<Arc<Mutex<Vec<i64>>>> = OnceLock::new();

    fn record_print(
        _heap: &mut RuntimeHeap,
        args: &[HostValue],
    ) -> Result<Option<HostValue>, String> {
        let value = match args.first() {
            Some(HostValue::Int(value)) => *value,
            other => return Err(format!("unexpected print args: {:?}", other)),
        };
        PRINT_CAPTURE
            .get()
            .expect("print capture missing")
            .lock()
            .expect("print capture poisoned")
            .push(value);
        Ok(None)
    }

    #[derive(Debug, Clone, PartialEq, EresAbi)]
    struct User {
        name: String,
        active: bool,
    }

    #[derive(Debug, Clone, PartialEq, EresAbi)]
    enum ResultTag {
        Guest,
        Named(String),
        Active { code: i64, label: String },
    }

    fn make_user() -> User {
        User {
            name: "Ada".to_string(),
            active: true,
        }
    }

    fn tag_user(user: User) -> ResultTag {
        if user.active {
            ResultTag::Active {
                code: user.name.len() as i64,
                label: user.name,
            }
        } else {
            ResultTag::Guest
        }
    }

    fn count_user_name(user: User) -> i64 {
        user.name.len() as i64
    }

    fn make_words() -> Vec<String> {
        vec!["one".to_string(), "two".to_string(), "three".to_string()]
    }

    fn count_words(words: Vec<String>) -> i64 {
        words.len() as i64
    }

    fn make_pair() -> (i64, bool) {
        (7, true)
    }

    fn score_pair(pair: (i64, bool)) -> i64 {
        if pair.1 { pair.0 } else { 0 }
    }

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
        let src = "fn main() -> Int { print_int(3); return 0; }";
        let bytes = compile_bytes_from_src(src).expect("compile failed");
        let engine = Engine::default();
        let module = wasmtime::Module::from_binary(&engine, &bytes).expect("module compile failed");
        let printed: Arc<Mutex<Vec<i64>>> = Arc::new(Mutex::new(Vec::new()));
        let mut store = Store::new(&engine, RuntimeHeap::default());
        let mut hosts = default_host_functions()
            .into_iter()
            .filter(|host| host.name != "print_int")
            .collect::<Vec<_>>();
        let _ = PRINT_CAPTURE.set(printed.clone());
        hosts.insert(
            0,
            HostFunction {
                name: "print_int",
                params: vec![AbiType::Int],
                result: AbiType::Unit,
                descriptors: Vec::new(),
                call: record_print,
            },
        );
        let imports = instantiate_host_imports(&mut store, &engine, &module, &hosts)
            .expect("instantiate imports");
        let imports_ref = imports.iter().cloned().map(Into::into).collect::<Vec<_>>();
        let instance = Instance::new(&mut store, &module, &imports_ref).expect("instance creation failed");

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
    fn string_literals_and_string_stdlib_run() {
        let src = "fn main() -> Int { return len(\"hello\"); }";
        let out = run_source(src, vec![]).expect("run failed");
        assert_eq!(out, Some(5));
    }

    #[test]
    fn host_abi_roundtrips_complex_values() {
        let src = r#"
            fn main() -> Int {
                let user = make_user();
                tag_user(user);
                let words = make_words();
                let pair = make_pair();
                return count_words(words) + score_pair(pair) + count_user_name(make_user());
            }
        "#;

        let mut hosts = default_host_functions();
        hosts.push(eres_host_function!(make_user, name = "make_user", params = [], result = User));
        hosts.push(eres_host_function!(tag_user, name = "tag_user", params = [User], result = ResultTag));
        hosts.push(eres_host_function!(count_user_name, name = "count_user_name", params = [User], result = i64));
        hosts.push(eres_host_function!(make_words, name = "make_words", params = [], result = Vec<String>));
        hosts.push(eres_host_function!(count_words, name = "count_words", params = [Vec<String>], result = i64));
        hosts.push(eres_host_function!(make_pair, name = "make_pair", params = [], result = (i64, bool)));
        hosts.push(eres_host_function!(score_pair, name = "score_pair", params = [(i64, bool)], result = i64));
        let bytes = compile_source_with_hosts(src, &hosts)
            .map(|out| out.bytes)
            .expect("compile failed");

        let result = run_wasm_bytes_with_hosts(&bytes, vec![], &hosts).expect("run failed");
        assert_eq!(result, Some(13));
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
