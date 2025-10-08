use std::rc::Rc;

use crate::call_frame::CallFrame;
use crate::chunk::{Instruction, OpCode};
use crate::closure::Closure;
use crate::function::{Function, FunctionType};
use crate::logger;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::vm::{InterpretError, InterpretResult, VM};

pub fn run(source: String, vm: &mut VM) -> InterpretResult {
    let mut parser = Parser::new(Scanner::new(&source));
    let mut instructions = parser.parse().map_err(InterpretError::CompileError)?;
    let line = parser
        .previous
        .ok_or(InterpretError::CompileError("Expected token".to_string()))?
        .line;
    instructions.push(Instruction::new(OpCode::Return, line));

    let mut function = Function::new("<script>".to_string(), 0, FunctionType::Script);
    function.chunk.extend(instructions);

    logger::info(&function.chunk.disassemble("<script>"));

    let frame = CallFrame {
        closure: Rc::new(Closure::new(Rc::new(function))),
        ip: 0,
        slot_start: 0,
    };

    vm.interpret(frame)
}
