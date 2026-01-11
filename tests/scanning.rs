mod test_utils;

use test_utils::interpret_file_result;

#[test]
fn identifiers() {
    assert!(interpret_file_result("examples/scanning/identifiers.lox").is_err());
}

#[test]
fn keywords() {
    assert!(interpret_file_result("examples/scanning/keywords.lox").is_err());
}

#[test]
fn numbers() {
    assert!(interpret_file_result("examples/scanning/numbers.lox").is_err());
}

#[test]
fn punctuators() {
    assert!(interpret_file_result("examples/scanning/punctuators.lox").is_err());
}

#[test]
fn strings() {
    assert!(interpret_file_result("examples/scanning/strings.lox").is_err());
}

#[test]
fn whitespace() {
    assert!(interpret_file_result("examples/scanning/whitespace.lox").is_err());
}
