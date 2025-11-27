use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{closure::Closure, value::Value};

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Rc<Closure>>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Rc<RefCell<Class>>,
    pub fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Rc<RefCell<Class>>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundMethod {
    pub method: Rc<Closure>,
    pub receiver: Rc<RefCell<Instance>>,
}

impl BoundMethod {
    pub fn new(method: Rc<Closure>, receiver: Rc<RefCell<Instance>>) -> Self {
        Self { method, receiver }
    }
}
