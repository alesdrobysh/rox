mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn function_print() {
    assert_eq!(
        interpret_file_stdout("examples/function/print.lox"),
        "fn foo\nnative fn clock\n"
    );
}

#[test]
fn function_parameters() {
    assert_eq!(
        interpret_file_stdout("examples/function/parameters.lox"),
        "0\n1\n3\n6\n10\n15\n21\n28\n36\n"
    );
}

#[test]
fn function_empty_body() {
    assert_eq!(
        interpret_file_stdout("examples/function/empty_body.lox"),
        "nil\n"
    );
}

#[test]
fn function_recursion() {
    assert_eq!(
        interpret_file_stdout("examples/function/recursion.lox"),
        "21\n"
    );
}

#[test]
fn function_mutual_recursion() {
    assert_eq!(
        interpret_file_stdout("examples/function/mutual_recursion.lox"),
        "true\ntrue\n"
    );
}

#[test]
fn function_nested_call_with_arguments() {
    assert_eq!(
        interpret_file_stdout("examples/function/nested_call_with_arguments.lox"),
        "\"hello world\"\n"
    );
}
