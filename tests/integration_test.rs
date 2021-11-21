use core::panic;

use tlc::type_check::Type;

fn file_test_type(file_name: &str, expected_type: Type) {
    assert_eq!(tlc::type_check(file_name), expected_type);
}

fn file_test(file_name: &str, expected_type: Type, expected_result: u64) {
    if let Type::Function { .. } = expected_type {
        panic!();
    }
    assert_eq!(tlc::type_check("tests/inputs/input1.txt"), expected_type);
    assert_eq!(tlc::compile("tests/inputs/input1.txt"), expected_result);
}

#[test]
fn type_check_input_1() {
    file_test("tests/inputs/input1.txt", Type::Number, 2);
}

#[test]
fn type_check_input_2() {
    file_test("tests/inputs/input2.txt", Type::Number, 3);
}

#[test]
fn type_check_input_3() {
    file_test_type(
        "tests/inputs/input3.txt",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
fn type_check_input_4() {
    file_test_type(
        "tests/inputs/input4.txt",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
#[should_panic]
fn type_check_input_5() {
    tlc::type_check("tests/inputs/input5.txt");
}

#[test]
fn type_check_input_6() {
    file_test("tests/inputs/input6.txt", Type::Number, 3);
}

#[test]
#[should_panic]
fn type_check_input_7() {
    tlc::type_check("tests/inputs/input7.txt");
}

#[test]
fn type_check_input_8() {
    file_test("tests/inputs/input8.txt", Type::Boolean, 1);
}

#[test]
fn type_check_input_9() {
    file_test("tests/inputs/input9.txt", Type::Boolean, 0);
}

#[test]
fn type_check_input_10() {
    file_test("tests/inputs/input10.txt", Type::Number, 1);
}

#[test]
#[should_panic]
fn type_check_input_11() {
    tlc::type_check("tests/inputs/input11.txt");
}

#[test]
#[should_panic]
fn type_check_input_12() {
    tlc::type_check("tests/inputs/input12.txt");
}

#[test]
fn type_check_input_14() {
    file_test("tests/inputs/input14.txt", Type::Boolean, 0);
}

#[test]
fn type_check_input_15() {
    file_test("tests/inputs/input15.txt", Type::Boolean, 0);
}

#[test]
fn type_check_input_basic() {
    file_test("tests/inputs/input_basic.txt", Type::Boolean, 0);
}

#[test]
fn type_check_input_medium() {
    file_test_type(
        "tests/inputs/input_medium.txt",
        Type::Function {
            argument: Box::new(Type::Number),
            ret: Box::new(Type::Number),
        },
    );
}

#[test]
fn type_check_input_advanced() {
    file_test("tests/inputs/input_advanced.txt", Type::Number, 15);
}

#[test]
fn type_check_input_super() {
    file_test("tests/inputs/input_super.txt", Type::Boolean, 0);
}

#[test]
fn type_check_input_rec_c_summation() {
    file_test("tests/inputs/input_rec_c_summation.txt", Type::Number, 55);
}

#[test]
#[should_panic]
fn type_check_input_rec_c_fail() {
    tlc::type_check("tests/inputs/input_rec_c_fail.txt");
}

#[test]
fn type_check_input_rec_c_factorial() {
    file_test("tests/inputs/input_rec_c_factorial.txt", Type::Number, 120);
}
