use std::{cell::RefCell, rc::Rc};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Upvalue {
    pub location: RefCell<Rc<Value>>,
    pub closed: Option<Value>,
}

impl Upvalue {
    pub fn new(value: Value) -> Self {
        Upvalue {
            location: RefCell::new(Rc::new(value)),
            closed: None,
        }
    }
}
