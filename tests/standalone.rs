mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn empty_file() {
    assert_eq!(
        interpret_file_stdout("examples/empty_file.lox"),
        ""
    );
}

#[test]
fn precedence() {
    assert_eq!(
        interpret_file_stdout("examples/precedence.lox"),
        "14\n8\n4\n0\ntrue\ntrue\ntrue\ntrue\n0\n0\n0\n0\n4\n"
    );
}

#[test]
fn unexpected_character() {
    assert!(interpret_file_result("examples/unexpected_character.lox").is_err());
}
