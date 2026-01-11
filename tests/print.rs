mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn missing_argument() {
    assert_eq!(
        interpret_file_stdout("examples/print/missing_argument.lox"),
        ""
    );
}
