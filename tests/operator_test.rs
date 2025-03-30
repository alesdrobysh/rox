mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn add() {
    assert_eq!(
        interpret_file_stdout("examples/operator/add.lox"),
        "579\n\"string\"\n"
    );
}

#[test]
fn comparison() {
    assert_eq!(
        interpret_file_stdout("examples/operator/comparison.lox"),
        "true\nfalse\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\ntrue\ntrue\n"
    );
}

#[test]
fn divide() {
    assert_eq!(
        interpret_file_stdout("examples/operator/divide.lox"),
        "4\n1\n"
    );
}

#[test]
fn equals() {
    assert_eq!(
        interpret_file_stdout("examples/operator/equals.lox"),
        "true\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\n"
    );
}

#[test]
fn multiply() {
    assert_eq!(
        interpret_file_stdout("examples/operator/multiply.lox"),
        "15\n3.702\n"
    );
}

#[test]
fn negate() {
    assert_eq!(
        interpret_file_stdout("examples/operator/negate.lox"),
        "-3\n3\n-3\n"
    );
}

#[test]
fn subtract() {
    assert_eq!(
        interpret_file_stdout("examples/operator/subtract.lox"),
        "1\n0\n"
    );
}
