use tc200::Type;

#[test]
fn type_check_input_1() {
    assert_eq!(tc200::type_check("tests/inputs/input1.txt"), Type::NumT);
}

#[test]
fn type_check_input_2() {
    assert_eq!(tc200::type_check("tests/inputs/input2.txt"), Type::NumT);
}

#[test]
fn type_check_input_3() {
    assert_eq!(
        tc200::type_check("tests/inputs/input3.txt"),
        Type::FunT {
            arg: Box::new(Type::NumT),
            ret: Box::new(Type::NumT)
        }
    );
}

#[test]
fn type_check_input_4() {
    assert_eq!(
        tc200::type_check("tests/inputs/input4.txt"),
        Type::FunT {
            arg: Box::new(Type::NumT),
            ret: Box::new(Type::NumT)
        }
    );
}

#[test]
#[should_panic]
fn type_check_input_5() {
    tc200::type_check("tests/inputs/input5.txt");
}

#[test]
fn type_check_input_6() {
    assert_eq!(tc200::type_check("tests/inputs/input6.txt"), Type::NumT);
}

#[test]
#[should_panic]
fn type_check_input_7() {
    tc200::type_check("tests/inputs/input7.txt");
}

#[test]
fn type_check_input_8() {
    assert_eq!(tc200::type_check("tests/inputs/input8.txt"), Type::BoolT);
}

#[test]
fn type_check_input_9() {
    assert_eq!(tc200::type_check("tests/inputs/input9.txt"), Type::BoolT);
}

#[test]
fn type_check_input_10() {
    assert_eq!(tc200::type_check("tests/inputs/input10.txt"), Type::NumT);
}

#[test]
#[should_panic]
fn type_check_input_11() {
    tc200::type_check("tests/inputs/input11.txt");
}

#[test]
#[should_panic]
fn type_check_input_12() {
    tc200::type_check("tests/inputs/input12.txt");
}

#[test]
fn type_check_input_14() {
    assert_eq!(tc200::type_check("tests/inputs/input14.txt"), Type::BoolT);
}

#[test]
fn type_check_input_15() {
    assert_eq!(tc200::type_check("tests/inputs/input15.txt"), Type::BoolT);
}

#[test]
fn type_check_input_basic() {
    assert_eq!(
        tc200::type_check("tests/inputs/input_basic.txt"),
        Type::BoolT
    );
}

#[test]
fn type_check_input_medium() {
    assert_eq!(
        tc200::type_check("tests/inputs/input_medium.txt"),
        Type::FunT {
            arg: Box::new(Type::NumT),
            ret: Box::new(Type::NumT)
        }
    );
}

#[test]
fn type_check_input_advanced() {
    assert_eq!(
        tc200::type_check("tests/inputs/input_advanced.txt"),
        Type::NumT
    );
}

#[test]
fn type_check_input_super() {
    assert_eq!(
        tc200::type_check("tests/inputs/input_super.txt"),
        Type::BoolT
    );
}
