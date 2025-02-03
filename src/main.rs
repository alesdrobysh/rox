mod chunk;
mod vm;

use chunk::{Chunk, Instruction, OpCode};

fn main() {
    let mut chunk = Chunk::new(Vec::new());

    // (- (1.2 + 3.4) / 5.6) * 7.8 - 9.0

    chunk.push(Instruction {
        op_code: OpCode::Constant(1.2),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Constant(3.4),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Add,
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Constant(5.6),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Divide,
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Negate,
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Constant(7.8),
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Multiply,
        line: 1,
    });

    chunk.push(Instruction {
        op_code: OpCode::Constant(9.0),
        line: 2,
    });

    chunk.push(Instruction {
        op_code: OpCode::Subtract,
        line: 2,
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
