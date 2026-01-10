mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn constructor() {
    assert_eq!(
        interpret_file_stdout("examples/inheritance/constructor.lox"),
        "\"value\"\n"
    );
}

#[test]
fn inherit_methods() {
    assert_eq!(
        interpret_file_stdout("examples/inheritance/inherit_methods.lox"),
        "\"foo\"\n\"bar\"\n\"bar\"\n"
    );
}

#[test]
fn set_fields_from_base_class() {
    assert_eq!(
        interpret_file_stdout("examples/inheritance/set_fields_from_base_class.lox"),
        "\"foo 1\"\n\"foo 2\"\n\"bar 1\"\n\"bar 2\"\n\"bar 1\"\n\"bar 2\"\n"
    );
}

#[test]
fn parenthesized_superclass() {
    assert!(interpret_file_result("examples/inheritance/parenthesized_superclass.lox").is_err());
}

#[test]
fn inherit_from_function() {
    assert!(interpret_file_result("examples/inheritance/inherit_from_function.lox").is_err());
}

#[test]
fn inherit_from_nil() {
    assert!(interpret_file_result("examples/inheritance/inherit_from_nil.lox").is_err());
}

#[test]
fn inherit_from_number() {
    assert!(interpret_file_result("examples/inheritance/inherit_from_number.lox").is_err());
}
