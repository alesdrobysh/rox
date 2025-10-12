use std::{cell::RefCell, rc::Rc};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Upvalue {
    pub location: Rc<RefCell<Value>>,
    pub closed: RefCell<Option<Value>>,
    pub stack_index: RefCell<Option<usize>>,
}

impl Upvalue {
    pub fn new(location: Rc<RefCell<Value>>, stack_index: usize) -> Self {
        Upvalue {
            location,
            closed: RefCell::new(None),
            stack_index: RefCell::new(Some(stack_index)),
        }
    }

    pub fn is_open(&self) -> bool {
        self.closed.borrow().is_none()
    }

    pub fn close(&self, value: Value) {
        *self.closed.borrow_mut() = Some(value);
        *self.stack_index.borrow_mut() = None;
    }

    pub fn get_value(&self) -> Value {
        if let Some(closed_value) = self.closed.borrow().as_ref() {
            closed_value.clone()
        } else {
            self.location.borrow().clone()
        }
    }

    pub fn set_value(&self, value: Value) {
        if self.closed.borrow().is_some() {
            *self.closed.borrow_mut() = Some(value);
        } else {
            *self.location.borrow_mut() = value;
        }
    }
}
