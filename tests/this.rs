mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn closure() {
    assert_eq!(
        interpret_file_stdout("examples/this/closure.lox"),
        "\"Foo\"\n"
    );
}

// #[test]
// fn nested_class() {
//     assert_eq!(
//         interpret_file_stdout("examples/this/nested_class.lox"),
//         "Outer instance\nOuter instance\nInner instance\n"
//     );
// }

#[test]
fn nested_closure() {
    assert_eq!(
        interpret_file_stdout("examples/this/nested_closure.lox"),
        "\"Foo\"\n"
    );
}

#[test]
fn this_at_top_level() {
    assert!(interpret_file_result("examples/this/this_at_top_level.lox").is_err());
}

#[test]
fn this_in_method() {
    assert_eq!(
        interpret_file_stdout("examples/this/this_in_method.lox"),
        "\"baz\"\n"
    );
}

#[test]
fn this_in_top_level_function() {
    assert!(interpret_file_result("examples/this/this_in_top_level_function.lox").is_err());
}
