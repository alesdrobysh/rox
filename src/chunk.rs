pub type Value = f64;

#[derive(Copy)]
pub enum OpCode {
    Return,
    Constant(Value),
}

impl Clone for OpCode {
    fn clone(&self) -> OpCode {
        *self
    }
}

#[derive(Copy)]
pub struct Instruction {
    pub op_code: OpCode,
    pub line: u32,
}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        *self
    }
}

pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub ip: usize,
}

impl Chunk {
    pub fn new(instructions: Vec<Instruction>) -> Chunk {
        Chunk {
            instructions,
            ip: 0,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn next_instruction(&mut self) -> Option<Instruction> {
        if self.ip < self.instructions.len() {
            let instruction = self.instructions[self.ip];
            self.ip += 1;
            Some(instruction)
        } else {
            None
        }
    }
}

pub fn print_instruction(instruction: Instruction) {
    match instruction.op_code {
        OpCode::Return => {
            println!("{:4} RETURN", instruction.line);
        }
        OpCode::Constant(value) => {
            println!("{:4} CONSTANT {}", instruction.line, value);
        }
    }
}
