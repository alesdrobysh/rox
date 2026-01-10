mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn bound_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/bound_method.lox"),
        "\"A.method(arg)\"\n"
    );
}

#[test]
fn call_same_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/call_same_method.lox"),
        "\"Derived.foo()\"\n\"Base.foo()\"\n"
    );
}

#[test]
fn call_other_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/call_other_method.lox"),
        "\"Derived.bar()\"\n\"Base.foo()\"\n"
    );
}

#[test]
fn closure() {
    assert_eq!(
        interpret_file_stdout("examples/super/closure.lox"),
        "\"Base\"\n"
    );
}

#[test]
fn constructor() {
    assert_eq!(
        interpret_file_stdout("examples/super/constructor.lox"),
        "\"Derived.init()\"\n\"Base.init(a, b)\"\n"
    );
}

#[test]
fn indirectly_inherited() {
    assert_eq!(
        interpret_file_stdout("examples/super/indirectly_inherited.lox"),
        "\"C.foo()\"\n\"A.foo()\"\n"
    );
}

#[test]
fn this_in_superclass_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/this_in_superclass_method.lox"),
        "\"a\"\n\"b\"\n"
    );
}

#[test]
fn reassign_superclass() {
    assert_eq!(
        interpret_file_stdout("examples/super/reassign_superclass.lox"),
        "\"Base.method()\"\n\"Base.method()\"\n"
    );
}

#[test]
fn super_in_inherited_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/super_in_inherited_method.lox"),
        "\"A\"\n"
    );
}

#[test]
fn super_in_closure_in_inherited_method() {
    assert_eq!(
        interpret_file_stdout("examples/super/super_in_closure_in_inherited_method.lox"),
        "\"A\"\n"
    );
}

#[test]
fn super_at_top_level() {
    assert!(interpret_file_result("examples/super/super_at_top_level.lox").is_err());
}

#[test]
fn no_superclass_call() {
    assert!(interpret_file_result("examples/super/no_superclass_call.lox").is_err());
}

#[test]
fn no_superclass_bind() {
    assert!(interpret_file_result("examples/super/no_superclass_bind.lox").is_err());
}

#[test]
fn super_without_name() {
    assert!(interpret_file_result("examples/super/super_without_name.lox").is_err());
}

#[test]
fn super_without_dot() {
    assert!(interpret_file_result("examples/super/super_without_dot.lox").is_err());
}

#[test]
fn parenthesized() {
    assert!(interpret_file_result("examples/super/parenthesized.lox").is_err());
}

#[test]
fn no_superclass_method() {
    assert!(interpret_file_result("examples/super/no_superclass_method.lox").is_err());
}

#[test]
fn extra_arguments() {
    assert!(interpret_file_result("examples/super/extra_arguments.lox").is_err());
}

#[test]
fn missing_arguments() {
    assert!(interpret_file_result("examples/super/missing_arguments.lox").is_err());
}

#[test]
fn super_in_top_level_function() {
    assert!(interpret_file_result("examples/super/super_in_top_level_function.lox").is_err());
}
