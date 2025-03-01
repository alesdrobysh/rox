use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    Return,
    Value(Value),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    DefineGlobal(String),
    GetGlobal(String),
    SetGlobal(String),
}

impl Clone for OpCode {
    fn clone(&self) -> OpCode {
        match self {
            OpCode::Value(Value::String(string)) => OpCode::Value(Value::String(string.clone())),
            OpCode::Value(Value::Number(number)) => OpCode::Value(Value::Number(*number)),
            OpCode::Value(Value::Bool(boolean)) => OpCode::Value(Value::Bool(*boolean)),
            rest => rest.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub op_code: OpCode,
    pub line: usize,
}

impl Instruction {
    pub fn new(op_code: OpCode, line: usize) -> Instruction {
        Instruction { op_code, line }
    }

    pub fn to_string(&self) -> String {
        let op_str = match &self.op_code {
            OpCode::Return => "RETURN".to_string(),
            OpCode::Value(value) => format!("VALUE {:?}", value),
            OpCode::Negate => "NEGATE".to_string(),
            OpCode::Add => "ADD".to_string(),
            OpCode::Subtract => "SUBTRACT".to_string(),
            OpCode::Multiply => "MULTIPLY".to_string(),
            OpCode::Divide => "DIVIDE".to_string(),
            OpCode::Not => "NOT".to_string(),
            OpCode::Equal => "EQUAL".to_string(),
            OpCode::Greater => "GREATER".to_string(),
            OpCode::Less => "LESS".to_string(),
            OpCode::Print => "PRINT".to_string(),
            OpCode::DefineGlobal(name) => format!("DEFINE_GLOBAL {}", name),
            OpCode::GetGlobal(name) => format!("GET_GLOBAL {}", name),
            OpCode::SetGlobal(name) => format!("SET_GLOBAL {}", name),
        };

        format!("{:4} {}", self.line, op_str)
    }
}

#[derive(Debug)]
pub struct Chunk {
    instructions: Vec<Instruction>,
    ip: usize,
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

    pub fn extend(&mut self, instructions: Vec<Instruction>) {
        self.instructions.extend(instructions);
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut result = String::from(format!("== {} ==\n", name));

        for instruction in self.instructions.iter() {
            result.push_str(&instruction.to_string());
            result.push_str("\n");
        }

        result
    }

    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        if self.ip < self.instructions.len() {
            let instruction = &self.instructions[self.ip];
            self.ip += 1;
            Some(instruction)
        } else {
            None
        }
    }
}
