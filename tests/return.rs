mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn after_else() {
    assert_eq!(
        interpret_file_stdout("examples/return/after_else.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn after_if() {
    assert_eq!(
        interpret_file_stdout("examples/return/after_if.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn after_while() {
    assert_eq!(
        interpret_file_stdout("examples/return/after_while.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn at_top_level() {
    assert!(interpret_file_result("examples/return/at_top_level.lox").is_err());
}

#[test]
fn in_function() {
    assert_eq!(
        interpret_file_stdout("examples/return/in_function.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn in_method() {
    assert_eq!(
        interpret_file_stdout("examples/return/in_method.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn return_nil_if_no_value() {
    assert_eq!(
        interpret_file_stdout("examples/return/return_nil_if_no_value.lox"),
        "nil\n"
    );
}
