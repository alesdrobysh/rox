use crate::{function::Function, value::Value};
use std::{fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Call(usize),
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
    SetLocal(usize),
    GetLocal(usize),
    Pop,
    JumpIfFalse(usize),
    Jump(usize),
    Loop(usize),
    Closure(Rc<Function>),
    GetUpvalue(usize),
    SetUpvalue(usize),
    Upvalue(usize, bool),
    CloseUpvalue,
    Class(String),
    SetProperty(String),
    GetProperty(String),
    Method(String),
}

fn format_function(function: &Function) -> String {
    format!(
        "fn({}):\n{}",
        function.name,
        function
            .chunk
            .disassemble(&function.name)
            .trim_end()
            .split('\n')
            .map(|line| format!("  {}", line))
            .collect::<Vec<_>>()
            .join("\n")
    )
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
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            OpCode::SetLocal(index) => format!("SET_LOCAL {}", index),
            OpCode::GetLocal(index) => format!("GET_LOCAL {}", index),
            OpCode::Pop => "POP".to_string(),
            OpCode::JumpIfFalse(offset) => format!("JUMP_IF_FALSE {}", offset),
            OpCode::Jump(offset) => format!("JUMP {}", offset),
            OpCode::Loop(offset) => format!("LOOP {}", offset),
            OpCode::Call(arg_count) => format!("CALL {}", arg_count),
            OpCode::Closure(function) => format!("CLOSURE {}", format_function(function)),
            OpCode::GetUpvalue(index) => format!("GET_UPVALUE {}", index),
            OpCode::SetUpvalue(index) => format!("SET_UPVALUE {}", index),
            OpCode::Upvalue(index, is_local) => format!("UPVALUE {} {}", index, is_local),
            OpCode::CloseUpvalue => "CLOSE_UPVALUE".to_string(),
            OpCode::Class(name) => format!("CLASS {}", name),
            OpCode::SetProperty(name) => format!("SET_PROPERTY {}", name),
            OpCode::GetProperty(name) => format!("GET_PROPERTY {}", name),
            OpCode::Method(name) => format!("METHOD {}", name),
        };

        write!(f, "line {:3}: {}", self.line, op_str)
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: Vec::new(),
        }
    }

    pub fn extend(&mut self, instructions: Vec<Instruction>) {
        self.instructions.extend(instructions);
    }

    pub fn disassemble(&self, name: &str) -> String {
        let mut result = String::from(format!("== {} ==\n", name));

        for instruction in self.instructions.iter() {
            result.push_str(&format!("{}", instruction));
            result.push_str("\n");
        }

        result
    }

    pub fn get_instruction(&self, ip: usize) -> Option<&Instruction> {
        self.instructions.get(ip)
    }

    pub fn get_last_instruction(&self) -> Option<&Instruction> {
        self.instructions.last()
    }
}
