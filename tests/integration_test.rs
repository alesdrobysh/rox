mod test_utils;

use test_utils::interpret_stdout;

#[test]
fn smoke_test() {
    assert_eq!(interpret_stdout("print 1 + 2;"), "3\n");
}
