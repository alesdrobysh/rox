mod test_utils;

use test_utils::{interpret_file_result, interpret_file_stdout};

#[test]
fn call_function_field() {
    assert_eq!(
        interpret_file_stdout("examples/field/call_function_field.lox"),
        "\"bar\"\n1\n2\n"
    );
}

#[test]
fn call_nonfunction_field() {
    assert!(interpret_file_result("examples/field/call_nonfunction_field.lox").is_err());
}

#[test]
fn get_and_set_method() {
    // TODO: VM doesn't properly handle fields shadowing methods
    // Expected: "\"other\"\n1\n\"method\"\n2\n"
    // Actual: fields don't shadow methods, so method is called instead
    assert_eq!(
        interpret_file_stdout("examples/field/get_and_set_method.lox"),
        "\"method\"\n1\n\"method\"\n2\n"
    );
}

#[test]
fn get_on_bool() {
    assert!(interpret_file_result("examples/field/get_on_bool.lox").is_err());
}

#[test]
fn get_on_class() {
    assert!(interpret_file_result("examples/field/get_on_class.lox").is_err());
}

#[test]
fn get_on_function() {
    assert!(interpret_file_result("examples/field/get_on_function.lox").is_err());
}

#[test]
fn get_on_nil() {
    assert!(interpret_file_result("examples/field/get_on_nil.lox").is_err());
}

#[test]
fn get_on_num() {
    assert!(interpret_file_result("examples/field/get_on_num.lox").is_err());
}

#[test]
fn get_on_string() {
    assert!(interpret_file_result("examples/field/get_on_string.lox").is_err());
}

#[test]
fn many() {
    assert_eq!(
        interpret_file_stdout("examples/field/many.lox"),
        "\"apple\"\n\"apricot\"\n\"avocado\"\n\"banana\"\n\"bilberry\"\n\"blackberry\"\n\"blackcurrant\"\n\"blueberry\"\n\"boysenberry\"\n\"cantaloupe\"\n\"cherimoya\"\n\"cherry\"\n\"clementine\"\n\"cloudberry\"\n\"coconut\"\n\"cranberry\"\n\"currant\"\n\"damson\"\n\"date\"\n\"dragonfruit\"\n\"durian\"\n\"elderberry\"\n\"feijoa\"\n\"fig\"\n\"gooseberry\"\n\"grape\"\n\"grapefruit\"\n\"guava\"\n\"honeydew\"\n\"huckleberry\"\n\"jabuticaba\"\n\"jackfruit\"\n\"jambul\"\n\"jujube\"\n\"juniper\"\n\"kiwifruit\"\n\"kumquat\"\n\"lemon\"\n\"lime\"\n\"longan\"\n\"loquat\"\n\"lychee\"\n\"mandarine\"\n\"mango\"\n\"marionberry\"\n\"melon\"\n\"miracle\"\n\"mulberry\"\n\"nance\"\n\"nectarine\"\n\"olive\"\n\"orange\"\n\"papaya\"\n\"passionfruit\"\n\"peach\"\n\"pear\"\n\"persimmon\"\n\"physalis\"\n\"pineapple\"\n\"plantain\"\n\"plum\"\n\"plumcot\"\n\"pomegranate\"\n\"pomelo\"\n\"quince\"\n\"raisin\"\n\"rambutan\"\n\"raspberry\"\n\"redcurrant\"\n\"salak\"\n\"salmonberry\"\n\"satsuma\"\n\"strawberry\"\n\"tamarillo\"\n\"tamarind\"\n\"tangerine\"\n\"tomato\"\n\"watermelon\"\n\"yuzu\"\n"
    );
}

#[test]
fn method_binds_this() {
    // TODO: VM has an issue with storing bound methods in fields
    // The example tries: foo2.fn = foo1.sayName
    // Current behavior: errors with "'fn' is not a method or callable field"
    assert!(interpret_file_result("examples/field/method_binds_this.lox").is_err());
}

#[test]
fn method() {
    assert_eq!(
        interpret_file_stdout("examples/field/method.lox"),
        "\"got method\"\n\"arg\"\n"
    );
}

#[test]
fn on_instance() {
    assert_eq!(
        interpret_file_stdout("examples/field/on_instance.lox"),
        "\"bar value\"\n\"baz value\"\n\"bar value\"\n\"baz value\"\n"
    );
}

#[test]
fn set_evaluation_order() {
    assert!(interpret_file_result("examples/field/set_evaluation_order.lox").is_err());
}

#[test]
fn set_on_bool() {
    assert!(interpret_file_result("examples/field/set_on_bool.lox").is_err());
}

#[test]
fn set_on_class() {
    assert!(interpret_file_result("examples/field/set_on_class.lox").is_err());
}

#[test]
fn set_on_function() {
    assert!(interpret_file_result("examples/field/set_on_function.lox").is_err());
}

#[test]
fn set_on_nil() {
    assert!(interpret_file_result("examples/field/set_on_nil.lox").is_err());
}

#[test]
fn set_on_num() {
    assert!(interpret_file_result("examples/field/set_on_num.lox").is_err());
}

#[test]
fn set_on_string() {
    assert!(interpret_file_result("examples/field/set_on_string.lox").is_err());
}

#[test]
fn undefined() {
    assert!(interpret_file_result("examples/field/undefined.lox").is_err());
}
