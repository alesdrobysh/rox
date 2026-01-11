mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn assign_to_closure() {
    assert_eq!(
        interpret_file_stdout("examples/closure/assign_to_closure.lox"),
        "\"local\"\n\"after f\"\n\"after f\"\n\"after g\"\n"
    );
}

#[test]
fn assign_to_shadowed_later() {
    assert_eq!(
        interpret_file_stdout("examples/closure/assign_to_shadowed_later.lox"),
        "\"inner\"\n\"assigned\"\n"
    );
}

#[test]
fn close_over_function_parameter() {
    assert_eq!(
        interpret_file_stdout("examples/closure/close_over_function_parameter.lox"),
        "\"param\"\n"
    );
}

#[test]
fn close_over_later_variable() {
    assert_eq!(
        interpret_file_stdout("examples/closure/close_over_later_variable.lox"),
        "\"b\"\n\"a\"\n"
    );
}

#[test]
fn close_over_method_parameter() {
    assert_eq!(
        interpret_file_stdout("examples/closure/close_over_method_parameter.lox"),
        "\"param\"\n"
    );
}

#[test]
fn closed_closure_in_function() {
    assert_eq!(
        interpret_file_stdout("examples/closure/closed_closure_in_function.lox"),
        "\"local\"\n"
    );
}

#[test]
fn nested_closure() {
    assert_eq!(
        interpret_file_stdout("examples/closure/nested_closure.lox"),
        "\"a\"\n\"b\"\n\"c\"\n"
    );
}

#[test]
fn open_closure_in_function() {
    assert_eq!(
        interpret_file_stdout("examples/closure/open_closure_in_function.lox"),
        "\"local\"\n"
    );
}

#[test]
fn reference_closure_multiple_times() {
    assert_eq!(
        interpret_file_stdout("examples/closure/reference_closure_multiple_times.lox"),
        "\"a\"\n\"a\"\n"
    );
}

#[test]
fn reuse_closure_slot() {
    assert_eq!(
        interpret_file_stdout("examples/closure/reuse_closure_slot.lox"),
        "\"a\"\n"
    );
}

#[test]
fn shadow_closure_with_local() {
    assert_eq!(
        interpret_file_stdout("examples/closure/shadow_closure_with_local.lox"),
        "\"closure\"\n\"shadow\"\n\"closure\"\n"
    );
}

#[test]
fn unused_closure() {
    assert_eq!(
        interpret_file_stdout("examples/closure/unused_closure.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn unused_later_closure() {
    assert_eq!(
        interpret_file_stdout("examples/closure/unused_later_closure.lox"),
        "\"a\"\n"
    );
}
