use crate::chunk::{print_instruction, Chunk, OpCode, Value};
use std::env;

pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
}
pub type InterpretResult = Result<(), InterpretError>;

pub struct VM {
    pub chunk: Chunk,
    pub stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            let instruction = &self.chunk.next_instruction();

            match instruction {
                Some(_) => {}
                None => {
                    return Err(InterpretError::RuntimeError(
                        "No more instructions".to_string(),
                    ))
                }
            }

            let instruction = instruction.unwrap();

            match env::var("DEBUG") {
                Ok(_) => {
                    println!("");
                    self.stack.iter().for_each(|v| println!("[{}]", v));
                    print_instruction(instruction);
                }
                Err(_) => {}
            }

            match instruction.op_code {
                OpCode::Return => return Ok(()),
                OpCode::Constant(value) => {
                    self.stack.push(value);
                }
                OpCode::Negate => match self.stack.pop() {
                    Some(value) => self.stack.push(-value),
                    None => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to negate".to_string(),
                        ))
                    }
                },
                OpCode::Add => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(a + b),
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to add".to_string(),
                        ))
                    }
                },
                OpCode::Subtract => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b - a),
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to subtract".to_string(),
                        ))
                    }
                },
                OpCode::Multiply => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(a * b),
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to multiply".to_string(),
                        ))
                    }
                },
                OpCode::Divide => match (self.stack.pop(), self.stack.pop()) {
                    (Some(a), Some(b)) => self.stack.push(b / a),
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to divide".to_string(),
                        ))
                    }
                },
            }
        }
    }
}
