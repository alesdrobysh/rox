use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String),
    RuntimeError(String, usize),
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpretError::CompileError(msg) => write!(f, "Compile Error: {}", msg),
            InterpretError::RuntimeError(msg, line) => {
                write!(f, "Runtime Error at line {}: {}", line, msg)
            }
        }
    }
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
                    self.push_stack(value.clone());
                }
                OpCode::Negate => match self.stack.pop() {
                    Some(Value::Number(value)) => self.push_stack(Value::Number(-value)),
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
                OpCode::Add => match (self.pop_stack(line)?, self.pop_stack(line)?) {
                    (Value::Number(b), Value::Number(a)) => self.push_stack(Value::Number(a + b)),
                    (Value::String(b), Value::String(a)) => {
                        self.push_stack(Value::String(format!("{}{}", a, b)))
                    }
                    _ => {
                        return Err(InterpretError::RuntimeError(
                            "Operands must be numbers or strings".to_string(),
                            line,
                        ))
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
                    let value = self.pop_stack(line)?;
                    self.push_stack(Value::Bool(value.is_falsey()));
                }
                OpCode::Equal => match (self.pop_stack(line)?, self.pop_stack(line)?) {
                    (Value::Number(a), Value::Number(b)) => {
                        self.push_stack(Value::Bool(a == b));
                    }
                    (Value::Bool(a), Value::Bool(b)) => {
                        self.push_stack(Value::Bool(a == b));
                    }
                    (Value::Nil, Value::Nil) => {
                        self.push_stack(Value::Bool(true));
                    }
                    (Value::String(a), Value::String(b)) => {
                        self.push_stack(Value::Bool(a == b));
                    }
                    (_, _) => {
                        self.push_stack(Value::Bool(false));
                    }
                },
                OpCode::Greater => self.binary_op(|a, b| Ok(Value::Bool(a > b)), line)?,
                OpCode::Less => self.binary_op(|a, b| Ok(Value::Bool(a < b)), line)?,
                OpCode::Print => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    writeln!(handle, "{}", self.pop_stack(line)?).unwrap();
                }
                OpCode::DefineGlobal(name) => {
                    let value = self.peek_stack(line)?;
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
                    if !self.globals.contains_key(&name[..]) {
                        return Err(InterpretError::RuntimeError(
                            format!("Undefined variable '{}'", name),
                            line,
                        ));
                    }
                    let value = self.peek_stack(line)?;
                    self.globals.insert(name.clone(), value);
                }
                OpCode::SetLocal(local) => {
                    if *local >= self.stack.len() {
                        return Err(InterpretError::RuntimeError(
                            format!("Invalid local variable index {}", local),
                            line,
                        ));
                    }

                    self.stack[*local] = self.peek_stack(line)?;
                }
                OpCode::GetLocal(index) => {
                    if *index >= self.stack.len() {
                        return Err(InterpretError::RuntimeError(
                            format!("Local variable access out of bounds: index {}", index),
                            line,
                        ));
                    }

                    let value = self.stack[*index].clone();
                    self.push_stack(value);
                }
                OpCode::Pop => {
                    self.pop_stack(line)?;
                }
                OpCode::JumpIfFalse(offset) => {
                    let effective_offset = if self.peek_stack(line)?.is_falsey() {
                        *offset
                    } else {
                        0
                    };

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
        match (self.pop_stack(line)?, self.pop_stack(line)?) {
            (Value::Number(b), Value::Number(a)) => {
                self.push_stack(op(a, b)?);
                Ok(())
            }
            (_, _) => Err(InterpretError::RuntimeError(
                "Operands must be numbers".to_string(),
                line,
            )),
        }
    }

    fn pop_stack(&mut self, line: usize) -> Result<Value, InterpretError> {
        self.stack.pop().ok_or(InterpretError::RuntimeError(
            "Stack is empty, cannot pop".to_string(),
            line,
        ))
    }

    fn peek_stack(&mut self, line: usize) -> Result<Value, InterpretError> {
        let top = self.stack.last().ok_or(InterpretError::RuntimeError(
            "Stack is empty, cannot peek".to_string(),
            line,
        ))?;
        Ok(top.clone())
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value)
    }
}
