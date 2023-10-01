use std::fmt::Debug;

use crate::vm::VM;

#[derive(Clone)]
pub enum Value {
    Nil,
    String(String),
    Function(fn(ctx: &mut VM) -> i32),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::String(val) => write!(f, "{val}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}
