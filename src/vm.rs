use crate::chunk::{Chunk, OpCode, Value};

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String),
}
pub type InterpretResult = Result<(), InterpretError>;

pub struct VM<'a> {
    pub chunk: &'a mut Chunk,
    pub stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a mut Chunk) -> VM<'a> {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self
                .chunk
                .next_instruction()
                .ok_or(InterpretError::RuntimeError(
                    "No more instructions".to_string(),
                ))?;

            match instruction.op_code {
                OpCode::Return => {
                    return {
                        println!("{:?}", self.stack);
                        Ok(())
                    }
                }
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
