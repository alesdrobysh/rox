mod test_utils;

use test_utils::interpret_file_stdout;

#[test]
fn basic_syntax() {
    let output = interpret_file_stdout("examples/for/basic_syntax.lox");
    assert_eq!(output, "1\n2\n3\n0\n1\n2\n0\n1\n0\n1\n");
}
