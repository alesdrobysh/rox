use crate::scanner::Scanner;

pub fn compile(source: String) {
    Scanner::new(source).scan_tokens();
}
