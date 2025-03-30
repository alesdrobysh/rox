mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn else_test() {
    assert_eq!(
        interpret_file_stdout("examples/if/else.lox"),
        "\"good\"\n\"good\"\n\"block\"\n"
    );
}

#[test]
fn if_test() {
    assert_eq!(
        interpret_file_stdout("examples/if/if.lox"),
        "\"good\"\n\"block\"\ntrue\n"
    );
}

#[test]
fn fun_in_else_test() {
    assert!(interpret_file_result("examples/if/fun_in_else.lox").is_err());
}

#[test]
fn var_in_then_test() {
    assert!(interpret_file_result("examples/if/var_in_then.lox").is_err());
}

#[test]
fn var_in_else_test() {
    assert!(interpret_file_result("examples/if/var_in_else.lox").is_err());
}

#[test]
fn fun_in_then_test() {
    assert!(interpret_file_result("examples/if/fun_in_then.lox").is_err());
}

#[test]
fn class_in_else_test() {
    assert!(interpret_file_result("examples/if/class_in_else.lox").is_err());
}

#[test]
fn class_in_then_test() {
    assert!(interpret_file_result("examples/if/class_in_then.lox").is_err());
}

#[test]
fn dangling_else_test() {
    assert_eq!(
        interpret_file_stdout("examples/if/dangling_else.lox"),
        "\"good\"\n"
    );
}

#[test]
fn truth_test() {
    assert_eq!(
        interpret_file_stdout("examples/if/truth.lox"),
        "\"false\"\n\"nil\"\ntrue\n0\n\"empty\"\n"
    );
}
