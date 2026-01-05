mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn empty() {
    assert_eq!(interpret_file_stdout("examples/class/empty.lox"), "Foo\n");
}

#[test]
fn local_reference_self() {
    assert_eq!(
        interpret_file_stdout("examples/class/local_reference_self.lox"),
        "Foo\n"
    );
}

#[test]
fn reference_self() {
    assert_eq!(
        interpret_file_stdout("examples/class/reference_self.lox"),
        "Foo\n"
    );
}
