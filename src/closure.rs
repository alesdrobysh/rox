use std::rc::Rc;

use crate::{function::Function, upvalue::Upvalue};

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: Rc<Function>,
    pub upvalues: Vec<Rc<Upvalue>>,
}

impl Closure {
    pub fn new(function: Rc<Function>) -> Closure {
        Closure {
            function,
            upvalues: Vec::new(),
        }
    }
}
