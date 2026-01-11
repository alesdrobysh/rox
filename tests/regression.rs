mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn _394() {
    assert_eq!(interpret_file_stdout("examples/regression/394.lox"), "B\n");
}

#[test]
fn _40() {
    assert_eq!(
        interpret_file_stdout("examples/regression/40.lox"),
        "false\n"
    );
}
