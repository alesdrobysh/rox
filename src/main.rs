mod call_frame;
mod chunk;
mod closure;
mod compilation_context;
mod function;
mod logger;
mod native_functions;
mod parser;
mod run;
mod scanner;
pub mod upvalue;
mod value;
mod vm;

use run::run;
use std::io::{Read, Write};
use std::{env, fs, io, process};

use vm::{InterpretError, InterpretResult, VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = VM::new();

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        match run_file(&args[1], &mut vm) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                process::exit(65);
            }
        }
    } else {
        eprintln!("Usage: rox [path]");
        process::exit(64);
    }
}

fn repl(vm: &mut VM) {
    println!("Welcome to Lox REPL!");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match run(input, vm) {
            Ok(()) => {}
            Err(InterpretError::CompileError(e)) => {
                eprintln!("Compile error: {}", e);
            }
            Err(InterpretError::RuntimeError(message, line)) => {
                eprintln!("Runtime error at line {}: {}", line, message);
            }
        }
    }
}

fn run_file(path: &str, vm: &mut VM) -> InterpretResult {
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read file");
            run(contents, vm)
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            process::exit(1);
        }
    }
}
