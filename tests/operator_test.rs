mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn add() {
    assert_eq!(
        interpret_file_stdout("examples/operator/add.lox"),
        "579\n\"string\"\n"
    );
}

#[test]
fn add_bool_nil() {
    assert!(interpret_file_result("examples/operator/add_bool_nil.lox").is_err());
}

#[test]
fn add_bool_num() {
    assert!(interpret_file_result("examples/operator/add_bool_num.lox").is_err());
}

#[test]
fn add_bool_string() {
    assert!(interpret_file_result("examples/operator/add_bool_string.lox").is_err());
}

#[test]
fn add_nil_nil() {
    assert!(interpret_file_result("examples/operator/add_nil_nil.lox").is_err());
}

#[test]
fn add_num_nil() {
    assert!(interpret_file_result("examples/operator/add_num_nil.lox").is_err());
}

#[test]
fn add_string_nil() {
    assert!(interpret_file_result("examples/operator/add_string_nil.lox").is_err());
}

#[test]
fn comparison() {
    assert_eq!(
        interpret_file_stdout("examples/operator/comparison.lox"),
        "true\nfalse\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\ntrue\nfalse\ntrue\ntrue\nfalse\nfalse\nfalse\nfalse\ntrue\ntrue\ntrue\ntrue\n"
    );
}

#[test]
fn divide() {
    assert_eq!(
        interpret_file_stdout("examples/operator/divide.lox"),
        "4\n1\n"
    );
}

#[test]
fn divide_nonnum_num() {
    assert!(interpret_file_result("examples/operator/divide_nonnum_num.lox").is_err());
}

#[test]
fn divide_num_nonnum() {
    assert!(interpret_file_result("examples/operator/divide_num_nonnum.lox").is_err());
}

#[test]
fn equals() {
    assert_eq!(
        interpret_file_stdout("examples/operator/equals.lox"),
        "true\ntrue\nfalse\ntrue\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\n"
    );
}

// #[test]
// fn equals_class() {
//     assert_eq!(
//         interpret_file_stdout("examples/operator/equals_class.lox"),
//         "true\nfalse\nfalse\ntrue\nfalse\nfalse\nfalse\nfalse\n"
//     );
// }

// #[test]
// fn equals_method() {
//     assert_eq!(
//         interpret_file_stdout("examples/operator/equals_method.lox"),
//         "true\nfalse\n"
//     );
// }

#[test]
fn greater_nonnum_num() {
    assert!(interpret_file_result("examples/operator/greater_nonnum_num.lox").is_err());
}

#[test]
fn greater_num_nonnum() {
    assert!(interpret_file_result("examples/operator/greater_num_nonnum.lox").is_err());
}

#[test]
fn greater_or_equal_nonnum_num() {
    assert!(interpret_file_result("examples/operator/greater_or_equal_nonnum_num.lox").is_err());
}

#[test]
fn greater_or_equal_num_nonnum() {
    assert!(interpret_file_result("examples/operator/greater_or_equal_num_nonnum.lox").is_err());
}

#[test]
fn less_nonnum_num() {
    assert!(interpret_file_result("examples/operator/less_nonnum_num.lox").is_err());
}

#[test]
fn less_num_nonnum() {
    assert!(interpret_file_result("examples/operator/less_num_nonnum.lox").is_err());
}

#[test]
fn less_or_equal_nonnum_num() {
    assert!(interpret_file_result("examples/operator/less_or_equal_nonnum_num.lox").is_err());
}

#[test]
fn less_or_equal_num_nonnum() {
    assert!(interpret_file_result("examples/operator/less_or_equal_num_nonnum.lox").is_err());
}

#[test]
fn multiply() {
    assert_eq!(
        interpret_file_stdout("examples/operator/multiply.lox"),
        "15\n3.702\n"
    );
}

#[test]
fn multiply_nonnum_num() {
    assert!(interpret_file_result("examples/operator/multiply_nonnum_num.lox").is_err());
}

#[test]
fn multiply_num_nonnum() {
    assert!(interpret_file_result("examples/operator/multiply_num_nonnum.lox").is_err());
}

#[test]
fn negate() {
    assert_eq!(
        interpret_file_stdout("examples/operator/negate.lox"),
        "-3\n3\n-3\n"
    );
}

#[test]
fn negate_nonnum() {
    assert!(interpret_file_result("examples/operator/negate_nonnum.lox").is_err());
}

#[test]
fn not() {
    assert_eq!(
        interpret_file_stdout("examples/operator/not.lox"),
        "false\ntrue\ntrue\nfalse\nfalse\ntrue\nfalse\nfalse\n"
    );
}

#[test]
fn not_class() {
    assert_eq!(
        interpret_file_stdout("examples/operator/not_class.lox"),
        "false\nfalse\n"
    );
}

#[test]
fn not_equals() {
    assert_eq!(
        interpret_file_stdout("examples/operator/not_equals.lox"),
        "false\nfalse\ntrue\nfalse\ntrue\nfalse\ntrue\ntrue\ntrue\ntrue\n"
    );
}

#[test]
fn subtract() {
    assert_eq!(
        interpret_file_stdout("examples/operator/subtract.lox"),
        "1\n0\n"
    );
}

#[test]
fn subtract_nonnum_num() {
    assert!(interpret_file_result("examples/operator/subtract_nonnum_num.lox").is_err());
}

#[test]
fn subtract_num_nonnum() {
    assert!(interpret_file_result("examples/operator/subtract_num_nonnum.lox").is_err());
}
