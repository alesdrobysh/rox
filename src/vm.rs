use crate::call_frame::CallFrame;
use crate::chunk::{Instruction, OpCode};
use crate::class::{BoundMethod, Class, Instance};
use crate::closure::Closure;
use crate::function::NativeFunction;
use crate::native_functions::clock;
use crate::upvalue::Upvalue;
use crate::value::Value;
use std::cell::RefCell;
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
    open_upvalues: Vec<Rc<Upvalue>>,
    debug: bool,
}

impl VM {
    pub fn new() -> VM {
        let mut globals = HashMap::new();
        globals.insert(
            "clock".to_string(),
            Value::NativeFunction(Rc::new(NativeFunction::new("clock", clock))),
        );

        let debug = std::env::var("DEBUG")
            .map(|level| level == "debug")
            .unwrap_or(false);

        VM {
            stack: Vec::new(),
            globals,
            call_frame_stack: CallFrameStack::new(),
            open_upvalues: Vec::new(),
            debug,
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
                        self.close_upvalues(frame.slot_start)?;
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
                            self.call_closure(closure, arg_count, line, callee_index)?;
                        }

                        Value::NativeFunction(native) => {
                            let args_start = callee_index + 1;
                            let args: Vec<Value> = self.stack[args_start..].to_vec();

                            self.stack.truncate(callee_index);

                            let result = (native.function)(args);
                            self.push_stack(result);
                        }

                        Value::Class(class) => {
                            self.stack[callee_index] =
                                Value::instance(Instance::new(class.clone()));

                            if let Some(initializer) = class.borrow().methods.get("init") {
                                self.call_closure(
                                    initializer.clone(),
                                    arg_count,
                                    line,
                                    callee_index,
                                )?;
                            } else if arg_count != 0 {
                                return self.runtime_error(
                                    &format!("Expected 0 arguments but got {}", arg_count),
                                    line,
                                );
                            }
                        }

                        Value::BoundMethod(bound_method) => {
                            let closure = Rc::clone(&bound_method.borrow().method);
                            let top = self.stack.len() - 1;
                            self.stack[top] =
                                Value::Instance(bound_method.borrow().receiver.clone());
                            self.call_closure(closure, arg_count, line, callee_index)?;
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
                    let value = self.pop_stack(line)?;
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

                    if let Some(upvalue) = upvalue {
                        let value = upvalue.get_value();
                        self.push_stack(value);
                    } else {
                        return self
                            .runtime_error(&format!("Invalid upvalue index {}", index), line);
                    }
                }
                OpCode::SetUpvalue(index) => {
                    let value = self.peek_stack(line)?;
                    let frame = match self.call_frame_stack.last_mut() {
                        Some(frame) => frame,
                        None => return self.runtime_error("No call frame found", line),
                    };

                    if let Some(upvalue) = frame.closure.upvalues.get(*index) {
                        upvalue.set_value(value);
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
                OpCode::CloseUpvalue => {
                    let stack_top = self.stack.len();
                    if stack_top > 0 {
                        self.close_upvalues(stack_top - 1)?;
                    }
                    self.pop_stack(line)?;
                }
                OpCode::Class(name) => {
                    let class = Class::new(name.clone());
                    self.push_stack(Value::class(class));
                }
                OpCode::GetProperty(name) => {
                    let instance = self.peek_stack(line)?;
                    match instance {
                        Value::Instance(instance) => {
                            if let Some(property) = instance.borrow().fields.get(name) {
                                self.pop_stack(line)?;
                                self.push_stack(property.clone());
                            } else if let Some(method) =
                                instance.borrow().class.borrow().methods.get(name)
                            {
                                let bound_method =
                                    BoundMethod::new(method.clone(), instance.clone());
                                self.pop_stack(line)?;
                                self.push_stack(Value::bound_method(bound_method));
                            } else {
                                return self.runtime_error(
                                    &format!("Undefined property '{}'", name),
                                    line,
                                );
                            }
                        }
                        _ => {
                            return self.runtime_error(
                                &format!(
                                    "Only instances have properties. Expected instance, got {}",
                                    instance.type_name()
                                ),
                                line,
                            );
                        }
                    }
                }
                OpCode::SetProperty(name) => {
                    // Stack layout: [..., instance, value]
                    // We need to get both instance (depth 1) and value (depth 0)
                    let value = self.peek_stack_at(0, line)?;
                    let instance = self.peek_stack_at(1, line)?;

                    match instance {
                        Value::Instance(instance) => {
                            instance
                                .borrow_mut()
                                .fields
                                .insert(name.clone(), value.clone());
                            // Pop both instance and value, then push value back (assignment returns the value)
                            self.pop_stack(line)?;
                            self.pop_stack(line)?;
                            self.push_stack(value);
                        }
                        _ => {
                            return self.runtime_error(
                                &format!(
                                    "Only instances have properties. Expected instance, got {}",
                                    instance.type_name()
                                ),
                                line,
                            );
                        }
                    }
                }
                OpCode::Method(name) => {
                    let method = self.peek_stack_at(0, line)?;
                    let class_val = self.peek_stack_at(1, line)?;

                    match (&class_val, &method) {
                        (Value::Class(class_rc), Value::Closure(closure_rc)) => {
                            class_rc
                                .borrow_mut()
                                .methods
                                .insert(name.clone(), closure_rc.clone());

                            self.pop_stack(line)?;
                        }
                        _ => {
                            return self.runtime_error(
                                &format!(
                                    "METHOD
                                      requires class and closure, got {} and {}",
                                    class_val.type_name(),
                                    method.type_name()
                                ),
                                line,
                            );
                        }
                    }
                }
            }

            if self.debug {
                eprintln!(
                    "Handled instruction: {:?}\nStack: {}\n",
                    &instruction.op_code,
                    format_stack(&self.stack)
                );
            }
        }
    }

    fn call_closure(
        &mut self,
        closure: Rc<Closure>,
        arg_count: usize,
        line: usize,
        callee_index: usize,
    ) -> Result<(), InterpretError> {
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

        Ok(())
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
        self.peek_stack_at(0, line)
    }

    fn peek_stack_at(&self, depth: usize, line: usize) -> Result<Value, InterpretError> {
        let stack_len = self.stack.len();
        if depth >= stack_len {
            return Err(InterpretError::RuntimeError(
                format!(
                    "Cannot peek at depth {} with stack size {}",
                    depth, stack_len
                ),
                line,
            ));
        }
        Ok(self.stack[stack_len - 1 - depth].clone())
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

        let stack_value = match self.stack.get(absolute_index) {
            Some(value) => value.clone(),
            None => {
                self.runtime_error("Invalid stack index for upvalue capture", 0)?;
                unreachable!()
            }
        };

        for upvalue in &self.open_upvalues {
            if let Some(stack_index) = *upvalue.stack_index.borrow() {
                if stack_index == absolute_index {
                    return Ok(upvalue.clone());
                }
            }
        }

        let location = Rc::new(RefCell::new(stack_value));
        let new_upvalue = Rc::new(Upvalue::new(location, absolute_index));

        self.open_upvalues.push(new_upvalue.clone());
        Ok(new_upvalue)
    }

    fn close_upvalues(&mut self, last_stack_index: usize) -> InterpretResult {
        let mut to_close = Vec::new();

        self.open_upvalues.retain(|upvalue| {
            if let Some(stack_index) = *upvalue.stack_index.borrow() {
                if stack_index >= last_stack_index {
                    to_close.push(upvalue.clone());
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });

        for upvalue in to_close {
            let current_value = upvalue.location.borrow().clone();
            upvalue.close(current_value);
        }

        Ok(())
    }
}
