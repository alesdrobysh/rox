mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn and() {
    assert_eq!(
        interpret_file_stdout("examples/logical_operator/and.lox"),
        "false\n1\nfalse\ntrue\n3\ntrue\nfalse\n"
    );
}

#[test]
fn and_truth() {
    assert_eq!(
        interpret_file_stdout("examples/logical_operator/and_truth.lox"),
        "false\nnil\n\"ok\"\n\"ok\"\n\"ok\"\n"
    );
}

#[test]
fn or() {
    assert_eq!(
        interpret_file_stdout("examples/logical_operator/or.lox"),
        "1\n1\ntrue\nfalse\nfalse\nfalse\ntrue\n"
    );
}

#[test]
fn or_truth() {
    assert_eq!(
        interpret_file_stdout("examples/logical_operator/or_truth.lox"),
        "\"ok\"\n\"ok\"\ntrue\n0\n\"s\"\n"
    );
}
