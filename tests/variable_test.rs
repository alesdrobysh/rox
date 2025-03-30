mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn in_middle_of_block() {
    assert_eq!(
        interpret_file_stdout("examples/variable/in_middle_of_block.lox"),
        "\"a\"\n\"a b\"\n\"a c\"\n\"a b d\"\n"
    );
}

#[test]
fn in_nested_block() {
    assert_eq!(
        interpret_file_stdout("examples/variable/in_nested_block.lox"),
        "\"outer\"\n"
    );
}

#[test]
fn redefine_global() {
    assert_eq!(
        interpret_file_stdout("examples/variable/redefine_global.lox"),
        "\"2\"\n"
    );
}

#[test]
fn scope_reuse_in_different_blocks() {
    assert_eq!(
        interpret_file_stdout("examples/variable/scope_reuse_in_different_blocks.lox"),
        "\"first\"\n\"second\"\n"
    );
}

#[test]
fn shadow_and_local() {
    assert_eq!(
        interpret_file_stdout("examples/variable/shadow_and_local.lox"),
        "\"outer\"\n\"inner\"\n"
    );
}

#[test]
fn shadow_global() {
    assert_eq!(
        interpret_file_stdout("examples/variable/shadow_global.lox"),
        "\"shadow\"\n\"global\"\n"
    );
}

#[test]
fn shadow_local() {
    assert_eq!(
        interpret_file_stdout("examples/variable/shadow_local.lox"),
        "\"shadow\"\n\"local\"\n"
    );
}

#[test]
fn uninitialized() {
    assert_eq!(
        interpret_file_stdout("examples/variable/uninitialized.lox"),
        "nil\n"
    );
}

#[test]
fn unreached_undefined() {
    assert_eq!(
        interpret_file_stdout("examples/variable/unreached_undefined.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn use_global_in_initializer() {
    assert_eq!(
        interpret_file_stdout("examples/variable/use_global_in_initializer.lox"),
        "\"value\"\n"
    );
}

#[test]
fn undefined_global() {
    assert!(interpret_file_result("examples/variable/undefined_global.lox").is_err());
}

#[test]
fn undefined_local() {
    assert!(interpret_file_result("examples/variable/undefined_local.lox").is_err());
}

#[test]
fn use_false_as_var() {
    assert!(interpret_file_result("examples/variable/use_false_as_var.lox").is_err());
}

#[test]
fn use_nil_as_var() {
    assert!(interpret_file_result("examples/variable/use_nil_as_var.lox").is_err());
}

#[test]
fn use_this_as_var() {
    assert!(interpret_file_result("examples/variable/use_this_as_var.lox").is_err());
}

#[test]
fn duplicate_local() {
    assert!(interpret_file_result("examples/variable/duplicate_local.lox").is_err());
}

#[test]
fn duplicate_parameter() {
    assert!(interpret_file_result("examples/variable/duplicate_parameter.lox").is_err());
}

#[test]
fn collide_with_parameter() {
    assert!(interpret_file_result("examples/variable/collide_with_parameter.lox").is_err());
}

#[test]
fn use_local_in_initializer() {
    assert!(interpret_file_result("examples/variable/use_local_in_initializer.lox").is_err());
}
