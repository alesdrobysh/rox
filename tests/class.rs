mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn empty() {
    assert_eq!(interpret_file_stdout("examples/class/empty.lox"), "Foo\n");
}

#[test]
fn inherit_self() {
    assert!(interpret_file_result("examples/class/inherit_self.lox").is_err());
}

#[test]
fn inherited_method() {
    assert_eq!(
        interpret_file_stdout("examples/class/inherited_method.lox"),
        "\"in foo\"\n\"in bar\"\n\"in baz\"\n"
    );
}

#[test]
fn local_inherit_other() {
    assert_eq!(
        interpret_file_stdout("examples/class/local_inherit_other.lox"),
        "B\n"
    );
}

#[test]
fn local_inherit_self() {
    assert!(interpret_file_result("examples/class/local_inherit_self.lox").is_err());
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
