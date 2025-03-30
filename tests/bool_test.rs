mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn equality() {
    assert_eq!(
        interpret_file_stdout("examples/bool/equality.lox"),
        "true\nfalse\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\nfalse\ntrue\ntrue\ntrue\ntrue\ntrue\n"
    );
}

#[test]
fn not() {
    assert_eq!(
        interpret_file_stdout("examples/bool/not.lox"),
        "false\ntrue\ntrue\n"
    );
}
