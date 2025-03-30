mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn line_at_eof() {
    assert_eq!(
        interpret_file_stdout("examples/comments/line_at_eof.lox"),
        "\"ok\"\n"
    );
}

#[test]
fn only_line_comment() {
    assert_eq!(
        interpret_file_stdout("examples/comments/only_line_comment.lox"),
        ""
    );
}

#[test]
fn only_line_comment_and_line() {
    assert_eq!(
        interpret_file_stdout("examples/comments/only_line_comment_and_line.lox"),
        ""
    );
}

// #[test]
// fn unicode() {
//     assert_eq!(
//         interpret_file_stdout("examples/comments/unicode.lox"),
//         "\"ok\"\n"
//     );
// }
