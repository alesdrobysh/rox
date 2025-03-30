use gag::BufferRedirect;
use rox::vm::InterpretError;
use rox::{run::run, vm::VM};
use std::fs;
use std::io::Read;

fn capture_stdout<F>(func: F) -> String
where
    F: FnOnce(),
{
    let mut buf = BufferRedirect::stdout().unwrap();

    func();

    let mut output = String::new();
    buf.read_to_string(&mut output).unwrap();

    output
}

pub fn interpret_stdout(code: &str) -> String {
    capture_stdout(|| {
        let _ = run(code.to_string(), &mut VM::new());
    })
}

pub fn interpret_file_stdout(path: &str) -> String {
    let mut file = fs::File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    interpret_stdout(&contents)
}

pub fn interpret_file_result(path: &str) -> Result<(), InterpretError> {
    let mut file = fs::File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    run(contents, &mut VM::new())
}
