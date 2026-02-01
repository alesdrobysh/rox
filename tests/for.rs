mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn basic_syntax() {
    assert_eq!(
        interpret_file_stdout("examples/for/basic_syntax.lox"),
        "1\n2\n3\n0\n1\n2\n0\n1\n0\n1\n"
    );
}

#[test]
fn class_in_body() {
    assert!(interpret_file_result("examples/for/class_in_body.lox").is_err());
}

#[test]
fn closure_in_body() {
    assert_eq!(
        interpret_file_stdout("examples/for/closure_in_body.lox"),
        "4\n1\n4\n2\n4\n3\n"
    );
}

#[test]
fn fun_in_body() {
    assert!(interpret_file_result("examples/for/fun_in_body.lox").is_err());
}

#[test]
fn return_closure() {
    assert_eq!(
        interpret_file_stdout("examples/for/return_closure.lox"),
        "\"i\"\n"
    );
}

#[test]
fn return_inside() {
    assert_eq!(
        interpret_file_stdout("examples/for/return_inside.lox"),
        "\"i\"\n"
    );
}

#[test]
fn scope() {
    assert_eq!(
        interpret_file_stdout("examples/for/scope.lox"),
        "0\n-1\n\"after\"\n0\n"
    );
}

#[test]
fn statement_condition() {
    assert!(interpret_file_result("examples/for/statement_condition.lox").is_err());
}

#[test]
fn statement_increment() {
    assert!(interpret_file_result("examples/for/statement_increment.lox").is_err());
}

#[test]
fn statement_initializer() {
    assert!(interpret_file_result("examples/for/statement_initializer.lox").is_err());
}

#[test]
fn syntax() {
    assert_eq!(
        interpret_file_stdout("examples/for/syntax.lox"),
        "1\n2\n3\n0\n1\n2\n\"done\"\n0\n1\n0\n1\n2\n0\n1\n"
    );
}

#[test]
fn var_in_body() {
    assert!(interpret_file_result("examples/for/var_in_body.lox").is_err());
}
