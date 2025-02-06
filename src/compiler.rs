use crate::scanner::Scanner;

pub fn compile(source: String) {
    let scanner = Scanner::new(&source);

    for token in scanner {
        println!("{:?}", token);
    }
}
