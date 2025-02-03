mod chunk;
mod vm;

use chunk::{Chunk, Instruction, OpCode};

fn main() {
    let mut chunk = Chunk::new(Vec::new());

    chunk.push(Instruction {
        op_code: OpCode::Constant(1.567),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Constant(28.0),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Return,
        line: 2,
    });

    let mut vm = vm::VM::new(chunk);

    match vm.interpret() {
        Ok(_) => println!("Interpretation successful"),
        Err(e) => println!("Interpretation failed: {}", e),
    };
}
