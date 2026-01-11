mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn decimal_point_at_eof() {
    assert!(interpret_file_result("examples/number/decimal_point_at_eof.lox").is_err());
}

#[test]
fn leading_dot() {
    assert!(interpret_file_result("examples/number/leading_dot.lox").is_err());
}

#[test]
fn literals() {
    assert_eq!(
        interpret_file_stdout("examples/number/literals.lox"),
        "123\n987654\n0\n-0\n123.456\n-0.001\n"
    );
}

// #[test]
// fn nan_equality() {
//     assert_eq!(
//         interpret_file_stdout("examples/number/nan_equality.lox"),
//         "false\ntrue\nfalse\ntrue\n"
//     );
// }

#[test]
fn trailing_dot() {
    assert!(interpret_file_result("examples/number/trailing_dot.lox").is_err());
}
