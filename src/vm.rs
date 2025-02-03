use crate::chunk::{print_instruction, Chunk, OpCode, Value};
use std::env;

pub type InterpretResult = Result<(), String>;

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
                None => return Err("No more instructions".to_string()),
            }

            let instruction = instruction.unwrap();

            match env::var("DEBUG") {
                Ok(_) => {
                    self.stack.iter().for_each(|v| println!("[{}]", v));
                    print_instruction(instruction)
                }
                Err(_) => {}
            }

            match instruction.op_code {
                OpCode::Return => return Ok(()),
                OpCode::Constant(value) => {
                    self.stack.push(value);
                }
            }
        }
    }
}
