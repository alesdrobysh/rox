mod chunk;
mod compiler;
mod logger;
mod parser;
mod scanner;
mod vm;

use std::io::{Read, Write};
use std::{env, fs, io, process};

use compiler::Compiler;
use vm::{InterpretError, InterpretResult};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(InterpretError::CompileError(e)) => {
                eprintln!("Compile error: {}", e);
                process::exit(65);
            }
            Err(InterpretError::RuntimeError(e)) => {
                eprintln!("Runtime error: {}", e);
                process::exit(70);
            }
        }
    } else {
        eprintln!("Usage: rox [path]");
        process::exit(64);
    }
}

fn repl() {
    println!("Welcome to Lox REPL!");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match run(input) {
            Ok(()) => {}
            Err(InterpretError::CompileError(e)) => {
                eprintln!("Compile error: {}", e);
            }
            Err(InterpretError::RuntimeError(e)) => {
                eprintln!("Runtime error: {}", e);
            }
        }
    }
}

fn run_file(path: &str) -> InterpretResult {
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            run(contents)
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    }
}

fn run(source: String) -> InterpretResult {
    let mut compiler = Compiler::new(&source);
    let chunk = compiler
        .compile()
        .map_err(|error| InterpretError::CompileError(error))?;

    vm::VM::new(chunk).interpret()
}
