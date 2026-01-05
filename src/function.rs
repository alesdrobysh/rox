use std::fmt;

use crate::{chunk::Chunk, value::Value};

#[derive(Debug, Clone)]
pub enum FunctionType {
    Function,
    Method,
    Initializer,
    Script,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub function_type: FunctionType,
    pub chunk: Chunk,
}

impl Function {
    pub fn new(name: String, arity: usize, function_type: FunctionType) -> Function {
        Function {
            name,
            arity,
            function_type,
            chunk: Chunk::new(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.function_type {
            FunctionType::Function => write!(f, "fn {}", self.name),
            FunctionType::Method => write!(f, "method {}", self.name),
            FunctionType::Initializer => write!(f, "initializer {}", self.name),
            FunctionType::Script => write!(f, "<script>"),
        }
    }
}

pub type NativeFn = fn(args: Vec<Value>) -> Value;

#[derive(Clone, Debug)]
pub struct NativeFunction {
    pub name: String,
    pub function: NativeFn,
}

impl NativeFunction {
    pub fn new(name: &str, function: NativeFn) -> NativeFunction {
        NativeFunction {
            name: name.to_string(),
            function,
        }
    }
}
