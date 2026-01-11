mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn error_after_multiline() {
    assert!(interpret_file_result("examples/string/error_after_multiline.lox").is_err());
}

// #[test]
// fn literals() {
//     assert_eq!(
//         interpret_file_stdout("examples/string/literals.lox"),
//         "\"()\"\n\"a string\"\n\"A~¶Þॐஃ\"\n"
//     );
// }

#[test]
fn multiline() {
    assert_eq!(
        interpret_file_stdout("examples/string/multiline.lox"),
        "\"1\n2\n3\"\n"
    );
}

// #[test]
// fn unterminated() {
//     assert!(interpret_file_result("examples/string/unterminated.lox").is_err());
// }
