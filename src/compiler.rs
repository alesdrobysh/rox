use crate::chunk::{self, Chunk};
use crate::logger;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Compiler<'a> {
    compiling_chunk: Chunk,
    parser: Parser<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Compiler<'a> {
        let scanner = Scanner::new(source);

        Compiler {
            parser: Parser::new(scanner),
            compiling_chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self) -> Result<&mut Chunk, String> {
        let instructions = self.parser.parse()?;

        self.current_chunk().extend(instructions);

        self.emit_return()?;

        logger::info(&self.compiling_chunk.disassemble("code"));

        Ok(self.current_chunk())
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn emit_byte(&mut self, byte: chunk::OpCode) -> Result<(), String> {
        let line = self.parser.previous.ok_or("Expected token")?.line;

        self.current_chunk()
            .push(chunk::Instruction::new(byte, line));

        Ok(())
    }

    fn emit_return(&mut self) -> Result<(), String> {
        self.emit_byte(chunk::OpCode::Return)
    }
}
