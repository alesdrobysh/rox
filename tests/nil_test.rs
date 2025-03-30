mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn literal() {
    assert_eq!(
        interpret_file_stdout("examples/nil/literal.lox"),
        "nil\n"
    );
}
