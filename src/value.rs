use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    closure::Closure,
    function::{Function, NativeFunction},
    upvalue::Upvalue,
};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil,
    String(Rc<String>),
    Function(Rc<Function>),
    NativeFunction(Rc<NativeFunction>),
    Closure(Rc<Closure>),
    Upvalue(Rc<RefCell<Upvalue>>),
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
            Self::Closure(closure) => write!(f, "closure {}", closure.function.name),
            Self::Upvalue(upvalue) => write!(
                f,
                "upvalue {:?} {:?}",
                upvalue.borrow().location,
                upvalue.borrow().closed
            ),
        }
    }
}
