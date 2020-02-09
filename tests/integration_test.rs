use type_checker::Type;

#[test]
fn type_check_input_1() {
    assert_eq!(
        type_checker::type_check("tests/inputs/input1.txt"),
        Type::NumT
    );
}

#[test]
fn type_check_input_2() {
    assert_eq!(
        type_checker::type_check("tests/inputs/input2.txt"),
        Type::NumT
    );
}

#[test]
fn type_check_input_3() {
    assert_eq!(
        type_checker::type_check("tests/inputs/input3.txt"),
        Type::FunT {
            arg: Box::new(Type::NumT),
            ret: Box::new(Type::NumT)
        }
    );
}

#[test]
fn type_check_input_4() {
    assert_eq!(
        type_checker::type_check("tests/inputs/input4.txt"),
        Type::FunT {
            arg: Box::new(Type::NumT),
            ret: Box::new(Type::NumT)
        }
    );
}
