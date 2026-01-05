use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    class::{BoundMethod, Class, Instance},
    closure::Closure,
    function::NativeFunction,
    upvalue::Upvalue,
};

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Nil,
    String(Rc<String>),
    NativeFunction(Rc<NativeFunction>),
    Closure(Rc<Closure>),
    Upvalue(Rc<RefCell<Upvalue>>),
    Class(Rc<RefCell<Class>>),
    Instance(Rc<RefCell<Instance>>),
    BoundMethod(Rc<RefCell<BoundMethod>>),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Bool(b) => !b,
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::Nil => "nil",
            Value::String(_) => "string",
            Value::NativeFunction(_) => "native function",
            Value::Closure(_) => "closure",
            Value::Upvalue(_) => "upvalue",
            Value::Class(_) => "class",
            Value::Instance(_) => "instance",
            Value::BoundMethod(_) => "bound method",
        }
    }

    pub fn class(class: Class) -> Self {
        Value::Class(Rc::new(RefCell::new(class)))
    }

    pub fn instance(instance: Instance) -> Self {
        Value::Instance(Rc::new(RefCell::new(instance)))
    }

    pub fn bound_method(bound_method: BoundMethod) -> Self {
        Value::BoundMethod(Rc::new(RefCell::new(bound_method)))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Bool(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::Nil => write!(f, "nil"),
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::NativeFunction(func) => write!(f, "native fn {}", func.name),
            Self::Closure(closure) => write!(f, "closure {}", closure.function.name),
            Self::Upvalue(upvalue) => write!(
                f,
                "upvalue {:?} {:?}",
                upvalue.borrow().location,
                upvalue.borrow().closed
            ),
            Self::Class(class) => write!(f, "{}", class.borrow().name),
            Self::Instance(instance) => {
                write!(f, "instance {}", instance.borrow().class.borrow().name)
            }
            Self::BoundMethod(bound_method) => {
                write!(
                    f,
                    "bound method {}",
                    bound_method.borrow().method.function.name
                )
            }
        }
    }
}
