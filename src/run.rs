use crate::vm::{InterpretError, InterpretResult, VM};

use crate::compiler::Compiler;

pub fn run(source: String, vm: &mut VM) -> InterpretResult {
    let mut compiler = Compiler::new(&source);
    let chunk = compiler.compile().map_err(InterpretError::CompileError)?;

    vm.interpret(chunk)
}
