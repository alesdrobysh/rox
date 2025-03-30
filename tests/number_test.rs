mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn decimal_point_at_eof() {
    assert_eq!(
        interpret_file_stdout("examples/number/decimal_point_at_eof.lox"),
        ""
    );
}

#[test]
fn leading_dot() {
    assert_eq!(interpret_file_stdout("examples/number/leading_dot.lox"), "");
}

#[test]
fn literals() {
    assert_eq!(
        interpret_file_stdout("examples/number/literals.lox"),
        "123\n987654\n0\n-0\n123.456\n-0.001\n"
    );
}

#[test]
fn trailing_dot() {
    assert_eq!(
        interpret_file_stdout("examples/number/trailing_dot.lox"),
        ""
    );
}
