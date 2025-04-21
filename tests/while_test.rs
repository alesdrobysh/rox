mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn syntax() {
    assert_eq!(
        interpret_file_stdout("examples/while/syntax.lox"),
        "1\n2\n3\n0\n1\n2\n"
    );
}

#[test]
fn var_in_body() {
    assert!(interpret_file_result("examples/while/var_in_body.lox").is_err());
}
