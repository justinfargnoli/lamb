use core::panic;

use lamb::type_check::Type;

fn test_to_file_name(test_name: &str) -> String {
    let mut file_name = "tests/inputs/input_".to_string();
    file_name.push_str(test_name);
    file_name.push_str(".txt");
    file_name
}

fn file_test_type(test_name: &str, expected_type: Type) {
    let file_name = test_to_file_name(test_name);
    assert_eq!(lamb::type_check(file_name.as_str()), expected_type);
}

fn file_test(test_name: &str, expected_type: Type, expected_result: u64) {
    if let Type::Function { .. } = expected_type {
        panic!();
    }

    let file_name = test_to_file_name(test_name);
    assert_eq!(lamb::type_check(file_name.as_str()), expected_type);
    assert_eq!(lamb::compile(file_name.as_str()).unwrap(), expected_result);
}

#[test]
fn input_1() {
    file_test("1", Type::Number, 2);
}

#[test]
fn input_2() {
    file_test("2", Type::Number, 3);
}

#[test]
fn input_3() {
    file_test_type(
        "3",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
fn input_4() {
    file_test_type(
        "4",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
#[should_panic]
fn input_5() {
    lamb::type_check("5");
}

#[test]
fn input_6() {
    file_test("6", Type::Number, 3);
}

#[test]
#[should_panic]
fn input_7() {
    lamb::type_check("7");
}

#[test]
fn input_8() {
    file_test("8", Type::Boolean, 1);
}

#[test]
fn input_9() {
    file_test("9", Type::Boolean, 0);
}

#[test]
fn input_10() {
    file_test("10", Type::Number, 1);
}

#[test]
#[should_panic]
fn input_11() {
    lamb::type_check("11");
}

#[test]
#[should_panic]
fn input_12() {
    lamb::type_check("12");
}

#[test]
fn input_14() {
    file_test("14", Type::Boolean, 0);
}

#[test]
fn input_15() {
    file_test("15", Type::Boolean, 0);
}

#[test]
fn input_basic() {
    file_test("basic", Type::Boolean, 0);
}

#[test]
fn input_medium() {
    file_test_type(
        "medium",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
fn input_advanced() {
    file_test("advanced", Type::Number, 15);
}

#[test]
fn input_super() {
    file_test("super", Type::Boolean, 0);
}

#[test]
#[should_panic]
fn input_rec_c_summation() {
    file_test("rec_c_summation", Type::Number, 55);
}

#[test]
#[should_panic]
fn input_rec_c_fail() {
    lamb::type_check("rec_c_fail");
}

#[test]
#[should_panic]
fn input_rec_c_factorial() {
    file_test("rec_c_factorial", Type::Number, 120);
}

#[test]
fn input_is_even() {
    file_test("is_even", Type::Boolean, 0);
}

#[test]
fn input_if() {
    file_test("if", Type::Boolean, 0);
}

#[test]
fn input_nested_function() {
    file_test("nested_function", Type::Number, 15);
}

#[test]
fn input_undecidable_nested_function() {
    file_test("undecidable_nested_function", Type::Boolean, 0);
}
