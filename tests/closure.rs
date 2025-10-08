mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn nested_closure() {
    assert_eq!(
        interpret_file_stdout("examples/closure/nested_closure.lox"),
        "\"a\"\n\"b\"\n\"c\"\n"
    );
}
