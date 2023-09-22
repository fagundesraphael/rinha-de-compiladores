use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Deserialize)]
struct Location {
    start: i32,
    end: i32,
    filename: String,
}

#[derive(Clone, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Clone, Deserialize)]
pub struct Parameter {
    text: String,
    location: Location,
}

#[derive(Clone, Deserialize)]
pub enum Term {
    Int {
        kind: String,
        value: f64,
        location: Location,
    },
    Str {
        kind: String,
        value: String,
        location: Location,
    },
    Bool {
        kind: String,
        value: bool,
        location: Location,
    },
    If {
        kind: String,
        condition: Rc<Term>,
        then: Rc<Term>,
        otherwise: Rc<Term>,
        location: Location,
    },
    Let {
        kind: String,
        name: Parameter,
        value: Rc<Term>,
        next: Rc<Term>,
        location: Location,
    },
    Binary {
        kind: String,
        lhs: Rc<Term>,
        op: BinaryOp,
        rhs: Rc<Term>,
        location: Location,
    },
    Call {
        kind: String,
        callee: Rc<Term>,
        arguments: Vec<Rc<Term>>,
        location: Location,
    },
    Function {
        kind: String,
        parameters: Vec<Parameter>,
        value: Rc<Term>,
        location: Location,
    },
    First {
        kind: String,
        value: Rc<Term>,
        location: Location,
    },
    Print {
        kind: String,
        value: Rc<Term>,
        location: Location,
    },
    Second {
        kind: String,
        value: Rc<Term>,
        location: Location,
    },
    Tuple {
        kind: String,
        first: Rc<Term>,
        second: Rc<Term>,
        location: Location,
    },
    Var {
        kind: String,
        text: String,
        location: Location,
    },
}

#[derive(Clone, Deserialize)]
pub struct File {
    expression: Rc<Term>,
    location: Location,
}
