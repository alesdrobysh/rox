use crate::value::Value;

#[derive(Copy, Debug)]
pub enum OpCode {
    Return,
    Constant(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
}

impl Clone for OpCode {
    fn clone(&self) -> OpCode {
        *self
    }
}

#[derive(Copy, Clone)]
pub struct Instruction {
    pub op_code: OpCode,
    pub line: usize,
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self.op_code {
            OpCode::Return => {
                return format!("{:4} RETURN", self.line);
            }
            OpCode::Constant(value) => {
                return format!("{:4} CONSTANT {:?}", self.line, value);
            }
            OpCode::Negate => {
                return format!("{:4} NEGATE", self.line);
            }
            OpCode::Add => {
                return format!("{:4} ADD", self.line);
            }
            OpCode::Subtract => {
                return format!("{:4} SUBTRACT", self.line);
            }
            OpCode::Multiply => {
                return format!("{:4} MULTIPLY", self.line);
            }
            OpCode::Divide => {
                return format!("{:4} DIVIDE", self.line);
            }
            OpCode::Not => {
                return format!("{:4} NOT", self.line);
            }
            OpCode::Equal => {
                return format!("{:4} EQUAL", self.line);
            }
            OpCode::Greater => {
                return format!("{:4} GREATER", self.line);
            }
            OpCode::Less => {
                return format!("{:4} LESS", self.line);
            }
        }
    }
}

pub struct Chunk {
    pub instructions: Vec<Instruction>,
    pub ip: usize,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: Vec::new(),
            ip: 0,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut result = String::from(format!("== {} ==\n", name));

        for instruction in self.instructions.iter() {
            result.push_str(&instruction.to_string());
            result.push_str("\n");
        }

        result
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
