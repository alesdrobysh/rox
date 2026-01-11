mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn arity() {
    assert_eq!(
        interpret_file_stdout("examples/method/arity.lox"),
        "\"no args\"\n1\n3\n6\n10\n15\n21\n28\n36\n"
    );
}

#[test]
fn empty_block() {
    assert_eq!(
        interpret_file_stdout("examples/method/empty_block.lox"),
        "nil\n"
    );
}

#[test]
fn extra_arguments() {
    assert!(interpret_file_result("examples/method/extra_arguments.lox").is_err());
}

#[test]
fn missing_arguments() {
    assert!(interpret_file_result("examples/method/missing_arguments.lox").is_err());
}

#[test]
fn not_found() {
    assert!(interpret_file_result("examples/method/not_found.lox").is_err());
}

#[test]
fn print_bound_method() {
    // VM prints "bound method <name>" instead of "<fn <name>>"
    assert_eq!(
        interpret_file_stdout("examples/method/print_bound_method.lox"),
        "bound method method\n"
    );
}

#[test]
fn refer_to_name() {
    assert!(interpret_file_result("examples/method/refer_to_name.lox").is_err());
}

#[test]
fn too_many_arguments() {
    assert!(interpret_file_result("examples/method/too_many_arguments.lox").is_err());
}

#[test]
fn too_many_parameters() {
    assert!(interpret_file_result("examples/method/too_many_parameters.lox").is_err());
}
