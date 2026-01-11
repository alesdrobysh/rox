mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn associativity() {
    assert_eq!(
        interpret_file_stdout("examples/assignment/associativity.lox"),
        "\"c\"\n\"c\"\n\"c\"\n"
    );
}

#[test]
fn global() {
    assert_eq!(
        interpret_file_stdout("examples/assignment/global.lox"),
        "\"before\"\n\"after\"\n\"arg\"\n\"arg\"\n"
    );
}

#[test]
fn grouping() {
    assert!(interpret_file_result("examples/assignment/grouping.lox").is_err());
}

#[test]
fn infix_operator() {
    assert!(interpret_file_result("examples/assignment/infix_operator.lox").is_err());
}

#[test]
fn local() {
    assert_eq!(
        interpret_file_stdout("examples/assignment/local.lox"),
        "\"before\"\n\"after\"\n\"arg\"\n\"arg\"\n"
    );
}

#[test]
fn prefix_operator() {
    assert!(interpret_file_result("examples/assignment/prefix_operator.lox").is_err());
}

#[test]
fn syntax() {
    assert_eq!(
        interpret_file_stdout("examples/assignment/syntax.lox"),
        "\"var\"\n\"var\"\n"
    );
}

#[test]
fn to_this() {
    assert!(interpret_file_result("examples/assignment/to_this.lox").is_err());
}

#[test]
fn undefined() {
    assert!(interpret_file_result("examples/assignment/undefined.lox").is_err());
}
