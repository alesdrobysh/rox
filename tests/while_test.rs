mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn class_in_body() {
    assert!(interpret_file_result("examples/while/class_in_body.lox").is_err());
}

#[test]
fn closure_in_body() {
    assert_eq!(
        interpret_file_stdout("examples/while/closure_in_body.lox"),
        "1\n2\n3\n"
    );
}

#[test]
fn fun_in_body() {
    assert!(interpret_file_result("examples/while/fun_in_body.lox").is_err());
}

#[test]
fn return_closure() {
    assert_eq!(
        interpret_file_stdout("examples/while/return_closure.lox"),
        "\"i\"\n"
    );
}

#[test]
fn return_inside() {
    assert_eq!(
        interpret_file_stdout("examples/while/return_inside.lox"),
        "\"i\"\n"
    );
}

#[test]
fn syntax() {
    assert_eq!(
        interpret_file_stdout("examples/while/syntax.lox"),
        "1\n2\n3\n0\n1\n2\n"
    );
}

#[test]
fn var_in_body() {
    assert!(interpret_file_result("examples/while/var_in_body.lox").is_err());
}
