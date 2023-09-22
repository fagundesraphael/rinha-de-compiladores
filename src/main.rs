use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::fs;

mod interpreter;
mod types;

fn main() {
    let json_str = fs::read_to_string("path/to/fib.json").expect("Falha ao ler o arquivo JSON");
    let json: Value = serde_json::from_str(&json_str).expect("Falha ao fazer o parsing do JSON");
    interpret_file(&json);
}

fn interpret_file(json: &Value) {
    let file = types::File::from_json(json);
    let value = interpreter::interpret_file(&file);
    println!("value: {:?}", value);
}
