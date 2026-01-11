mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn arguments() {
    assert_eq!(
        interpret_file_stdout("examples/constructor/arguments.lox"),
        "\"init\"\n1\n2\n"
    );
}

#[test]
fn call_init_early_return() {
    // Currently fails because parser doesn't allow any return in init
    assert!(interpret_file_result("examples/constructor/call_init_early_return.lox").is_err());
}

#[test]
fn call_init_explicitly() {
    assert_eq!(
        interpret_file_stdout("examples/constructor/call_init_explicitly.lox"),
        "\"Foo.init(one)\"\n\"Foo.init(two)\"\ninstance Foo\n\"init\"\n"
    );
}

#[test]
fn default_arguments() {
    assert!(interpret_file_result("examples/constructor/default_arguments.lox").is_err());
}

#[test]
fn default() {
    assert_eq!(
        interpret_file_stdout("examples/constructor/default.lox"),
        "instance Foo\n"
    );
}

#[test]
fn early_return() {
    // Currently fails because parser doesn't allow any return in init
    assert!(interpret_file_result("examples/constructor/early_return.lox").is_err());
}

#[test]
fn extra_arguments() {
    assert!(interpret_file_result("examples/constructor/extra_arguments.lox").is_err());
}

#[test]
fn init_not_method() {
    assert_eq!(
        interpret_file_stdout("examples/constructor/init_not_method.lox"),
        "\"not initializer\"\n"
    );
}

#[test]
fn missing_arguments() {
    assert!(interpret_file_result("examples/constructor/missing_arguments.lox").is_err());
}

#[test]
fn return_in_nested_function() {
    assert_eq!(
        interpret_file_stdout("examples/constructor/return_in_nested_function.lox"),
        "\"bar\"\ninstance Foo\n"
    );
}

#[test]
fn return_value() {
    assert!(interpret_file_result("examples/constructor/return_value.lox").is_err());
}
