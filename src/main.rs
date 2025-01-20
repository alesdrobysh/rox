type Value = f64;
enum OpCode {
    Return,
    Constant(Value),
}

struct Chunk {
    op_code: OpCode,
    line: u32,
}

fn print_chunk(chunk: Chunk) {
    match chunk.op_code {
        OpCode::Return => println!("{:4} RETURN", chunk.line),
        OpCode::Constant(value) => println!("{:4} CONSTANT {}", chunk.line, value),
    }
}

fn main() {
    print_chunk(Chunk {
        op_code: OpCode::Return,
        line: 1,
    });

    print_chunk(Chunk {
        op_code: OpCode::Constant(1.0),
        line: 2,
    });
}
