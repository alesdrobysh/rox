use std::rc::Rc;

use crate::{chunk::Instruction, closure::Closure};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub closure: Rc<Closure>,
    pub slot_start: usize,
    pub ip: usize,
}

impl CallFrame {
    pub fn offset(&mut self, offset: usize) {
        self.ip += offset;
    }

    pub fn offset_backward(&mut self, offset: usize) {
        self.ip -= offset;
    }

    pub fn next(&mut self) -> Option<&Instruction> {
        let instruction = self.closure.function.chunk.get_instruction(self.ip);
        self.ip += 1;
        instruction
    }

    pub fn peek(&self) -> Option<&Instruction> {
        self.closure.function.chunk.get_instruction(self.ip)
    }
}
