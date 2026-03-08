use eres_abi::{HostFunction, eres_host_function};

fn print_string(value: String) {
    println!("{}", value);
}

fn print_i64(value: i64) {
    println!("{}", value);
}

fn print_f64(value: f64) {
    println!("{}", value);
}

fn print_bool(value: bool) {
    println!("{}", value);
}

fn add_one(value: i64) -> i64 {
    value + 1
}

fn is_positive(value: i64) -> bool {
    value > 0
}

fn half(value: f64) -> f64 {
    value / 2.0
}

fn len(value: String) -> i64 {
    value.chars().count() as i64
}

pub fn standard_library() -> Vec<HostFunction> {
    vec![
        eres_host_function!(print_string, name = "print", params = [String], result = ()),
        eres_host_function!(print_i64, name = "print_int", params = [i64], result = ()),
        eres_host_function!(print_f64, name = "print_float", params = [f64], result = ()),
        eres_host_function!(print_bool, name = "print_bool", params = [bool], result = ()),
        eres_host_function!(len, name = "len", params = [String], result = i64),
        eres_host_function!(add_one, name = "add_one", params = [i64], result = i64),
        eres_host_function!(is_positive, name = "is_positive", params = [i64], result = bool),
        eres_host_function!(half, name = "half", params = [f64], result = f64),
    ]
}
