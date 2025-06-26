use std::{fmt, rc::Rc};

use crate::function::{Function, NativeFunction};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil,
    String(Rc<String>),
    Function(Rc<Function>),
    NativeFunction(Rc<NativeFunction>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(b) => !b,
            Value::Nil => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Nil => write!(f, "nil"),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Function(func) => write!(f, "fn {}", func.name),
            Self::NativeFunction(func) => write!(f, "native fn {}", func.name),
        }
    }
}
