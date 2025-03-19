use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String, usize),
}
pub type InterpretResult = Result<(), InterpretError>;

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, chunk: &mut Chunk) -> InterpretResult {
        self.run(chunk)
    }

    fn run(&mut self, chunk: &mut Chunk) -> InterpretResult {
        loop {
            let instruction = chunk
                .next_instruction()
                .ok_or(InterpretError::RuntimeError(
                    "No more instructions".to_string(),
                    0,
                ))?;

            let line = instruction.line;
            match &instruction.op_code {
                OpCode::Return => return Ok(()),
                OpCode::Value(value) => {
                    self.stack.push(value.clone());
                }
                OpCode::Negate => match self.stack.pop() {
                    Some(Value::Number(value)) => self.stack.push(Value::Number(-value)),
                    Some(_) => {
                        return Err(InterpretError::RuntimeError(
                            "Cannot negate non-number value".to_string(),
                            line,
                        ))
                    }
                    None => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to negate".to_string(),
                            line,
                        ))
                    }
                },
                OpCode::Add => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(b)), Some(Value::Number(a))) => {
                        self.stack.push(Value::Number(a + b));
                    }
                    (Some(Value::String(b)), Some(Value::String(a))) => {
                        self.stack.push(Value::String(format!("{}{}", a, b)));
                    }
                    (Some(_), Some(_)) => {
                        return Err(InterpretError::RuntimeError(
                            "Operands must be numbers or strings".to_string(),
                            line,
                        ));
                    }
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough operands".to_string(),
                            line,
                        ));
                    }
                },
                OpCode::Subtract => self.binary_op(|a, b| Ok(Value::Number(a - b)), line)?,
                OpCode::Multiply => self.binary_op(|a, b| Ok(Value::Number(a * b)), line)?,
                OpCode::Divide => self.binary_op(
                    |a, b| {
                        if b == 0.0 {
                            return Err(InterpretError::RuntimeError(
                                "Division by zero".to_string(),
                                line,
                            ));
                        }

                        Ok(Value::Number(a / b))
                    },
                    line,
                )?,
                OpCode::Not => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to negate".to_string(),
                        line,
                    ))?;

                    self.stack.push(Value::Bool(value.is_falsey()));
                }
                OpCode::Equal => match (self.stack.pop(), self.stack.pop()) {
                    (Some(Value::Number(a)), Some(Value::Number(b))) => {
                        self.stack.push(Value::Bool(a == b));
                    }
                    (Some(Value::Bool(a)), Some(Value::Bool(b))) => {
                        self.stack.push(Value::Bool(a == b));
                    }
                    (Some(Value::Nil), Some(Value::Nil)) => {
                        self.stack.push(Value::Bool(true));
                    }
                    (Some(Value::String(a)), Some(Value::String(b))) => {
                        self.stack.push(Value::Bool(a == b));
                    }
                    (Some(_), Some(_)) => {
                        self.stack.push(Value::Bool(false));
                    }
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Not enough values to compare".to_string(),
                            line,
                        ));
                    }
                },
                OpCode::Greater => self.binary_op(|a, b| Ok(Value::Bool(a > b)), line)?,
                OpCode::Less => self.binary_op(|a, b| Ok(Value::Bool(a < b)), line)?,
                OpCode::Print => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to print".to_string(),
                        line,
                    ))?;

                    println!("{}", value);
                }
                OpCode::DefineGlobal(name) => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to define global variable".to_string(),
                        line,
                    ))?;
                    self.globals.insert(name.clone(), value);
                }
                OpCode::GetGlobal(name) => {
                    let value = self.globals.get(name).ok_or(InterpretError::RuntimeError(
                        format!("Undefined variable '{}'", name),
                        line,
                    ))?;

                    self.stack.push(value.clone());
                }
                OpCode::SetGlobal(name) => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to set global variable".to_string(),
                        line,
                    ))?;
                    self.globals.insert(name.clone(), value);
                }
                OpCode::SetLocal(local) => {
                    let value = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to set local variable".to_string(),
                        line,
                    ))?;

                    if *local >= self.stack.len() {
                        return Err(InterpretError::RuntimeError(
                            format!("Invalid local variable index {}", local),
                            line,
                        ));
                    }

                    self.stack[*local] = value;
                }
                OpCode::GetLocal(index) => {
                    if *index >= self.stack.len() {
                        return Err(InterpretError::RuntimeError(
                            format!("Local variable access out of bounds: index {}", index),
                            line,
                        ));
                    }

                    let value = self.stack[*index].clone();
                    self.stack.push(value);
                }
                OpCode::Pop => {
                    self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to pop".to_string(),
                        line,
                    ))?;
                }
                OpCode::JumpIfFalse(offset) => {
                    let condition = self.stack.pop().ok_or(InterpretError::RuntimeError(
                        "Not enough values to jump".to_string(),
                        line,
                    ))?;

                    let effective_offset = if condition.is_falsey() { *offset } else { 0 };

                    chunk.offset(effective_offset);
                }
                OpCode::Jump(offset) => {
                    let offset = *offset;
                    chunk.offset(offset);
                }
            }
        }
    }

    fn binary_op<F>(&mut self, op: F, line: usize) -> InterpretResult
    where
        F: Fn(f64, f64) -> Result<Value, InterpretError>,
    {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Number(b)), Some(Value::Number(a))) => {
                let result = op(a, b)?;
                self.stack.push(result);
                Ok(())
            }
            (Some(_), Some(_)) => Err(InterpretError::RuntimeError(
                "Operands must be numbers".to_string(),
                line,
            )),
            _ => Err(InterpretError::RuntimeError(
                "Not enough operands".to_string(),
                line,
            )),
        }
    }
}
