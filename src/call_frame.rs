use std::rc::Rc;

use crate::{chunk::Instruction, function::Function};

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub func: Rc<Function>,
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

    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        let instruction = self.func.chunk.get_instruction(self.ip);
        self.ip += 1;
        instruction
    }
}
