use std::collections::HashMap;

use crate::types::*;

struct Env {
    objects: HashMap<String, Value>,
}

impl Env {
    fn clone_env(&self) -> Env {
        Env {
            objects: self.objects.clone(),
        }
    }
}

struct Closure {
    body: Term,
    parameters: Vec<String>,
    env: Env,
}

enum Value {
    Boolean(bool),
    String(String),
    Number(f64),
    Closure(Closure),
    Tuple(Box<Value>, Box<Value>),
}

fn unimplemented() -> ! {
    panic!("unimplemented yet");
}

pub fn interpret_file(file: &File) -> Value {
    let env = Env {
        objects: HashMap::new(),
    };
    interpret(&file.expression, env)
}

fn type_mismatch(type_str: &str) -> ! {
    panic!("not a {}", type_str);
}

fn assert_int(value: &Value) -> f64 {
    if let Value::Number(num) = value {
        *num
    } else {
        type_mismatch("int")
    }
}

fn assert_tuple(value: &Value) -> (&Value, &Value) {
    if let Value::Tuple(fst, snd) = value {
        (fst, snd)
    } else {
        type_mismatch("tuple")
    }
}

fn assert_closure(value: &Value) -> &Closure {
    if let Value::Closure(closure) = value {
        closure
    } else {
        type_mismatch("closure")
    }
}

fn assert_bool(value: &Value) -> bool {
    if let Value::Boolean(b) = value {
        *b
    } else {
        type_mismatch("closure")
    }
}

fn cast_to_string(value: &Value) -> String {
    match value {
        Value::Number(num) => num.to_string(),
        Value::String(s) => s.clone(),
        _ => type_mismatch("string or int"),
    }
}

fn is_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => (l - r).abs() < f64::EPSILON,
        (Value::String(l), Value::String(r)) => l == r,
        (Value::Boolean(l), Value::Boolean(r)) => l == r,
        _ => type_mismatch("number or string or boolean"),
    }
}

fn interpret_binary(left: &Value, right: &Value, op: &BinaryOp) -> Value {
    match op {
        BinaryOp::Add => {
            if let (Value::Number(l), Value::Number(r)) = (left, right) {
                Value::Number(l + r)
            } else {
                let left_val = cast_to_string(left);
                let right_val = cast_to_string(right);
                Value::String(format!("{}{}", left_val, right_val))
            }
        }
        BinaryOp::Eq => {
            let value = is_equal(left, right);
            Value::Boolean(value)
        }
        BinaryOp::Neq => {
            let value = !is_equal(left, right);
            Value::Boolean(value)
        }
        BinaryOp::Sub => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Number(left_val - right_val)
        }
        BinaryOp::Mul => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Number(left_val * right_val)
        }
        BinaryOp::Div => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Number((left_val / right_val).floor())
        }
        BinaryOp::Rem => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Number((left_val % right_val).floor())
        }
        BinaryOp::Lt => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Boolean(left_val < right_val)
        }
        BinaryOp::Gt => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Boolean(left_val > right_val)
        }
        BinaryOp::Lte => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Boolean(left_val <= right_val)
        }
        BinaryOp::Gte => {
            let left_val = assert_int(left);
            let right_val = assert_int(right);
            Value::Boolean(left_val >= right_val)
        }
        BinaryOp::And => {
            let left_val = assert_bool(left);
            let right_val = assert_bool(right);
            Value::Boolean(left_val && right_val)
        }
        BinaryOp::Or => {
            let left_val = assert_bool(left);
            let right_val = assert_bool(right);
            Value::Boolean(left_val || right_val)
        }
    }
}

fn show_value(value: &Value) -> String {
    match value {
        Value::Number(num) => num.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::String(s) => s.clone(),
        Value::Closure(_) => "<#closure>".to_string(),
        Value::Tuple(fst, snd) => format!("({}, {})", show_value(fst), show_value(snd)),
    }
}

fn interpret(term: &Term, env: Env) -> Value {
    match term {
        Term::Str { value, .. } => Value::String(value.clone()),
        Term::Bool { value, .. } => Value::Boolean(*value),
        Term::Int { value, .. } => Value::Number(*value),
        Term::If {
            condition,
            then,
            otherwise,
            ..
        } => {
            let condition_value = interpret(condition, env.clone());
            let boolean = assert_bool(&condition_value);
            if boolean {
                interpret(then, env)
            } else {
                interpret(otherwise, env)
            }
        }
        Term::Tuple { first, second, .. } => {
            let fst = interpret(first, env.clone());
            let snd = interpret(second, env);
            Value::Tuple(Box::new(fst), Box::new(snd))
        }
        Term::First { value, .. } => {
            let fst = interpret(value, env);
            let (first, _) = assert_tuple(&fst);
            first.clone()
        }
        Term::Second { value, .. } => {
            let fst = interpret(value, env);
            let (_, second) = assert_tuple(&fst);
            second.clone()
        }
        Term::Binary { lhs, op, rhs, .. } => {
            let left = interpret(lhs, env.clone());
            let right = interpret(rhs, env);
            interpret_binary(&left, &right, op)
        }
        Term::Print { value, .. } => {
            let value = interpret(value, env.clone());
            let value_str = show_value(&value);
            println!("{}", value_str);
            value
        }
        Term::Var { text, .. } => {
            if let Some(value) = env.objects.get(text) {
                value.clone()
            } else {
                panic!("cannot find variable {}", text);
            }
        }
        Term::Let {
            name, value, next, ..
        } => {
            let mut new_env = env.clone_env();
            let value = interpret(value, env.clone());
            new_env.objects.insert(name.text.clone(), value);
            interpret(next, new_env)
        }
        Term::Call {
            callee, arguments, ..
        } => {
            let func = interpret(callee, env.clone());
            let closure = assert_closure(&func);

            if closure.parameters.len() != arguments.len() {
                panic!(
                    "expected {} arguments but instead got {}",
                    closure.parameters.len(),
                    arguments.len()
                );
            }

            let mut function_env = closure.env.clone_env();

            for (param, arg) in closure.parameters.iter().zip(arguments.iter()) {
                let arg_value = interpret(arg, env.clone());
                function_env.objects.insert(param.clone(), arg_value);
            }

            interpret(&closure.body, function_env)
        }
        Term::Function {
            parameters, value, ..
        } => Value::Closure(Closure {
            body: value.clone(),
            parameters: parameters.iter().map(|p| p.text.clone()).collect(),
            env,
        }),
    }
}
