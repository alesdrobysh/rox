use crate::call_frame::CallFrame;
use crate::chunk::{Instruction, OpCode};
use crate::closure::Closure;
use crate::function::NativeFunction;
use crate::native_functions::clock;
use crate::upvalue::Upvalue;
use crate::value::Value;
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

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

pub type CallFrameStack = Vec<CallFrame>;

pub fn format_stack(stack: &Vec<Value>) -> String {
    stack
        .iter()
        .map(|value| format!("{}", value))
        .collect::<Vec<String>>()
        .join(", ")
}

#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    call_frame_stack: CallFrameStack,
}

impl VM {
    pub fn new() -> VM {
        let mut globals = HashMap::new();
        globals.insert(
            "clock".to_string(),
            Value::NativeFunction(Rc::new(NativeFunction::new("clock", clock))),
        );

        VM {
            stack: Vec::new(),
            globals,
            call_frame_stack: CallFrameStack::new(),
        }
    }

    pub fn interpret(&mut self, frame: CallFrame) -> InterpretResult {
        self.call_frame_stack.push(frame);
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = match self.next_instruction() {
                Some(instr) => instr.clone(),
                None => return self.runtime_error("No more instructions", 0),
            };

            let line = instruction.line;
            match &instruction.op_code {
                OpCode::Return => match self.call_frame_stack.pop() {
                    Some(frame) => {
                        if self.call_frame_stack.is_empty() {
                            return Ok(());
                        }

                        let result = self.pop_stack(line)?;
                        self.stack.truncate(frame.slot_start);
                        self.push_stack(result);
                    }
                    None => return self.runtime_error("No call frame to return from", line),
                },
                OpCode::Call(arg_count) => {
                    let arg_count = *arg_count;

                    let callee_index = self.stack.len() - arg_count - 1;
                    let callee = self.stack[callee_index].clone();

                    match callee {
                        Value::Closure(closure) => {
                            let arity = closure.function.arity;

                            if arity != arg_count {
                                return self.runtime_error(
                                    &format!("Expected {} arguments but got {}", arity, arg_count),
                                    line,
                                );
                            }

                            self.call_frame_stack.push(CallFrame {
                                closure,
                                ip: 0,
                                slot_start: callee_index,
                            });
                        }

                        Value::NativeFunction(native) => {
                            let args_start = callee_index + 1;
                            let args: Vec<Value> = self.stack[args_start..].to_vec();

                            self.stack.truncate(callee_index);

                            let result = (native.function)(args);
                            self.push_stack(result);
                        }

                        _ => return self.runtime_error("Cannot call non-function value", line),
                    }
                }
                OpCode::Closure(function) => {
                    let mut closure = Closure::new(function.clone());

                    let upvalue_instructions: Vec<_> = std::iter::from_fn(|| {
                        if let Some(instruction) = self.peek_instruction() {
                            match &instruction.op_code {
                                OpCode::Upvalue(_, _) => {
                                    let op_code = Some(instruction.op_code.clone());
                                    self.next_instruction();
                                    op_code
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                    for op_code in upvalue_instructions {
                        if let OpCode::Upvalue(index, is_local) = op_code {
                            if is_local {
                                // unwrap should be safe here because since we are handling an
                                // operation we've read it from the non-empty call frame stack
                                // let slot_start = self.call_frame_stack.last().unwrap().slot_start;
                                closure.upvalues.push(self.capture_upvalue(index)?);
                            } else {
                                if let Some(frame) = self.call_frame_stack.last_mut() {
                                    closure.upvalues.push(frame.closure.upvalues[index].clone());
                                } else {
                                    return self.runtime_error("No call frame found", line);
                                }
                            }
                        }
                    }

                    self.push_stack(Value::Closure(Rc::new(closure)));
                }
                OpCode::Value(value) => {
                    self.push_stack(value.clone());
                }
                OpCode::Negate => match self.stack.pop() {
                    Some(Value::Number(value)) => self.push_stack(Value::Number(-value)),
                    Some(_) => return self.runtime_error("Cannot negate non-number value", line),
                    None => return self.runtime_error("Not enough values to negate", line),
                },
                OpCode::Add => match (self.pop_stack(line)?, self.pop_stack(line)?) {
                    (Value::Number(b), Value::Number(a)) => self.push_stack(Value::Number(a + b)),
                    (Value::String(b), Value::String(a)) => {
                        self.push_stack(Value::String(Rc::new(format!("{}{}", a, b))))
                    }
                    (a, b) => {
                        return self.runtime_error(
                            &format!(
                                "Operands must be numbers or strings, found: {} and {}",
                                a, b
                            ),
                            line,
                        );
                    }
                },
                OpCode::Subtract => self.binary_op(|a, b| Ok(Value::Number(a - b)), line)?,
                OpCode::Multiply => self.binary_op(|a, b| Ok(Value::Number(a * b)), line)?,
                OpCode::Divide => {
                    if let (Value::Number(b), Value::Number(a)) =
                        (self.pop_stack(line)?, self.pop_stack(line)?)
                    {
                        if b == 0.0 {
                            return self.runtime_error("Division by zero", line);
                        }
                        self.push_stack(Value::Number(a / b));
                    } else {
                        return self.runtime_error("Operands must be numbers", line);
                    }
                }
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
                    let value = self.pop_stack(line)?;

                    match value {
                        Value::Closure(closure) => {
                            writeln!(handle, "{}", closure.function).unwrap()
                        }
                        _ => writeln!(handle, "{}", value).unwrap(),
                    }
                }
                OpCode::DefineGlobal(name) => {
                    let value = self.peek_stack(line)?;
                    self.globals.insert(name.clone(), value);
                }
                OpCode::GetGlobal(name) => {
                    let value = match self.globals.get(name) {
                        Some(val) => val,
                        None => {
                            return self
                                .runtime_error(&format!("Undefined variable '{}'", name), line);
                        }
                    };

                    self.stack.push(value.clone());
                }
                OpCode::SetGlobal(name) => {
                    if !self.globals.contains_key(&name[..]) {
                        return self.runtime_error(&format!("Undefined variable '{}'", name), line);
                    }
                    let value = self.peek_stack(line)?;
                    self.globals.insert(name.clone(), value);
                }
                OpCode::SetLocal(local) => {
                    let absolute_index = self.to_absolute_index(*local);
                    if absolute_index >= self.stack.len() {
                        return self.runtime_error(
                            &format!("Invalid local variable index {}", local),
                            line,
                        );
                    }

                    self.stack[absolute_index] = self.peek_stack(line)?;
                }
                OpCode::GetLocal(index) => {
                    let absolute_index = self.to_absolute_index(*index);
                    if absolute_index >= self.stack.len() {
                        return self.runtime_error(
                            &format!("Invalid local variable index {}", index),
                            line,
                        );
                    }

                    let value = self.stack[absolute_index].clone();
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

                    self.offset(effective_offset);
                }
                OpCode::Jump(offset) => {
                    let offset = *offset;
                    self.offset(offset);
                }
                OpCode::Loop(offset) => {
                    let offset = *offset;
                    self.offset_backward(offset);
                }
                OpCode::GetUpvalue(index) => {
                    let upvalue = self
                        .call_frame_stack
                        .last()
                        .map(|frame| frame.closure.upvalues.get(*index))
                        .flatten();

                    if let Some(value) = upvalue {
                        let location_rc = value.location.borrow().clone();
                        self.push_stack((*location_rc).clone());
                    } else {
                        return self
                            .runtime_error(&format!("Invalid upvalue index {}", index), line);
                    }
                }
                OpCode::SetUpvalue(index) => {
                    let value = Rc::new(self.peek_stack(line)?);
                    // unwrap should be safe here because since we are handling an
                    // operation we've read it from the non-empty call frame stack
                    let frame = self.call_frame_stack.last_mut().unwrap();

                    if let Some(upvalue) = frame.closure.upvalues.get(*index) {
                        *upvalue.location.borrow_mut() = value.clone();
                    } else {
                        return self
                            .runtime_error(&format!("Invalid upvalue index {}", index), line);
                    }
                }
                OpCode::Upvalue(_, _) => {
                    return self.runtime_error(
                        "Unexpected upvalue instruction encountered outside of closure creation",
                        line,
                    );
                }
            }

            if let Ok(level) = std::env::var("DEBUG") {
                if level == "debug" {
                    eprintln!("{}", format_stack(&self.stack));
                }
            }
        }
    }

    fn next_instruction(&mut self) -> Option<&Instruction> {
        self.call_frame_stack.last_mut()?.next()
    }

    fn peek_instruction(&self) -> Option<&Instruction> {
        self.call_frame_stack.last()?.peek()
    }

    fn offset(&mut self, offset: usize) {
        if let Some(frame) = self.call_frame_stack.last_mut() {
            frame.offset(offset);
        }
    }

    fn offset_backward(&mut self, offset: usize) {
        if let Some(frame) = self.call_frame_stack.last_mut() {
            frame.offset_backward(offset);
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
            (_, _) => self.runtime_error("Operands must be numbers", line),
        }
    }

    fn pop_stack(&mut self, line: usize) -> Result<Value, InterpretError> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => {
                self.runtime_error("Stack is empty, cannot pop", line)?;
                unreachable!()
            }
        }
    }

    fn peek_stack(&mut self, line: usize) -> Result<Value, InterpretError> {
        match self.stack.last() {
            Some(value) => Ok(value.clone()),
            None => {
                self.runtime_error("Stack is empty, cannot peek", line)?;
                unreachable!()
            }
        }
    }

    fn push_stack(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn runtime_error(&mut self, message: &str, line: usize) -> InterpretResult {
        let stacktrace = self
            .call_frame_stack
            .iter()
            .map(|frame| {
                format!(
                    "[line {}] in {}",
                    frame
                        .closure
                        .function
                        .chunk
                        .get_instruction(frame.ip)
                        .map(|instruction| instruction.line)
                        .unwrap_or(0),
                    frame.closure.function.name
                )
            })
            .rev()
            .collect::<Vec<String>>();

        Err(InterpretError::RuntimeError(
            format!("{}\n{}", message, stacktrace.join("\n")),
            line,
        ))
    }

    fn to_absolute_index(&self, index: usize) -> usize {
        if let Some(frame) = self.call_frame_stack.last() {
            let shift = frame.slot_start;
            return index + shift;
        }

        return index;
    }

    fn capture_upvalue(&mut self, index: usize) -> Result<Rc<Upvalue>, InterpretError> {
        let absolute_index = self.to_absolute_index(index);
        match self.stack.get(absolute_index) {
            Some(value) => Ok(Rc::new(Upvalue::new(value.clone()))),
            None => {
                self.runtime_error("Stack is empty, cannot capture upvalue", 0)?;
                unreachable!()
            }
        }
    }
}
