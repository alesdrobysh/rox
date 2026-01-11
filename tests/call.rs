mod test_utils;

use test_utils::interpret_file_result;

#[test]
fn bool() {
    assert!(interpret_file_result("examples/call/bool.lox").is_err());
}

#[test]
fn nil() {
    assert!(interpret_file_result("examples/call/nil.lox").is_err());
}

#[test]
fn num() {
    assert!(interpret_file_result("examples/call/num.lox").is_err());
}

#[test]
fn object() {
    assert!(interpret_file_result("examples/call/object.lox").is_err());
}

#[test]
fn string() {
    assert!(interpret_file_result("examples/call/string.lox").is_err());
}
