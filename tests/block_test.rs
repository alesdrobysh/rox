mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn empty() {
    assert_eq!(
        interpret_file_stdout("examples/block/empty.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn scope() {
    assert_eq!(
        interpret_file_stdout("examples/block/scope.lox"),
        "\"inner\"\n\"outer\"\n"
    );
}
