mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn body_must_be_block() {
    assert!(interpret_file_result("examples/function/body_must_be_block.lox").is_err());
}

#[test]
fn empty_body() {
    assert_eq!(
        interpret_file_stdout("examples/function/empty_body.lox"),
        "nil\n"
    );
}

#[test]
fn extra_arguments() {
    assert!(interpret_file_result("examples/function/extra_arguments.lox").is_err());
}

#[test]
fn local_mutual_recursion() {
    assert!(interpret_file_result("examples/function/local_mutual_recursion.lox").is_err());
}

#[test]
fn local_recursion() {
    assert_eq!(
        interpret_file_stdout("examples/function/local_recursion.lox"),
        "21\n"
    );
}

#[test]
fn missing_arguments() {
    assert!(interpret_file_result("examples/function/missing_arguments.lox").is_err());
}

#[test]
fn missing_comma_in_parameters() {
    assert!(interpret_file_result("examples/function/missing_comma_in_parameters.lox").is_err());
}

#[test]
fn mutual_recursion() {
    assert_eq!(
        interpret_file_stdout("examples/function/mutual_recursion.lox"),
        "true\ntrue\n"
    );
}

#[test]
fn nested_call_with_arguments() {
    assert_eq!(
        interpret_file_stdout("examples/function/nested_call_with_arguments.lox"),
        "\"hello world\"\n"
    );
}

#[test]
fn parameters() {
    assert_eq!(
        interpret_file_stdout("examples/function/parameters.lox"),
        "0\n1\n3\n6\n10\n15\n21\n28\n36\n"
    );
}

#[test]
fn print() {
    assert_eq!(
        interpret_file_stdout("examples/function/print.lox"),
        "fn foo\nnative fn clock\n"
    );
}

#[test]
fn recursion() {
    assert_eq!(
        interpret_file_stdout("examples/function/recursion.lox"),
        "21\n"
    );
}

#[test]
fn too_many_arguments() {
    assert!(interpret_file_result("examples/function/too_many_arguments.lox").is_err());
}

#[test]
fn too_many_parameters() {
    assert!(interpret_file_result("examples/function/too_many_parameters.lox").is_err());
}
