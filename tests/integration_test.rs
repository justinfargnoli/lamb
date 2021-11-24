use core::panic;
use lamb::{
    codegen,
    interpret::{self, Data, Function},
    parse::AST,
    read,
    tokenize::{self, TokenStream},
    type_check::{Type, TypedAST},
};
use std::{collections::VecDeque, convert::TryInto};

enum TestOptions {
    Parse,
    TypeCheck(Option<Type>),
    Interpret(Option<(Type, Data)>),
    Compile(Option<(Type, u64)>),
}

impl TestOptions {
    fn check_interpret(expected_type: &Type, expected_data: &Data) {
        match expected_type {
            Type::Boolean => match expected_data {
                Data::Boolean(_) => (),
                _ => panic!(),
            },
            Type::Number => match expected_data {
                Data::Number(_) => (),
                _ => panic!(),
            },
            Type::Function { .. } => (),
        }
    }

    fn check_compile(expected_type: &Type, expected_data: &Data, expected_result: u64) {
        TestOptions::check_interpret(expected_type, expected_data);

        match expected_data {
            Data::Boolean(boolean) => {
                if *boolean {
                    assert_eq!(1, expected_result);
                } else {
                    assert_eq!(0, expected_result);
                }
            }
            Data::Number(number) => {
                assert!(*number >= 0);
                assert_eq!(*number, expected_result.try_into().unwrap());
            }
            Data::Function(_) => panic!("Compiler can't return a function as its final result"),
        }
    }

    fn check(&self) {
        match self {
            TestOptions::Interpret(option) => {
                if let Some((expected_type, expected_data)) = option {
                    TestOptions::check_interpret(expected_type, expected_data)
                }
            }
            TestOptions::Compile(option) => {
                let option_with_data = compiler_result_to_data((*option).clone());
                if let Some((expected_type, expected_data, expected_result)) = option_with_data {
                    TestOptions::check_compile(&expected_type, &expected_data, expected_result)
                }
            }
            _ => (),
        }
    }
}

fn test_to_file_name(test_name: &str) -> String {
    let mut file_name = "tests/inputs/".to_string();
    file_name.push_str(test_name);
    file_name.push_str(".txt");
    file_name
}

fn test_read(name: &str) -> VecDeque<char> {
    read::build(test_to_file_name(name).as_str()).unwrap()
}

fn test_tokenizer(name: &str) -> TokenStream {
    let character_stream = test_read(name);
    tokenize::TokenStream::build(character_stream)
}

fn test_parser(name: &str) -> AST {
    let mut token_stream = test_tokenizer(name);
    AST::build(&mut token_stream)
}

fn test_typed_ast_and_ast(name: &str, expected: Option<Type>) -> (AST, TypedAST) {
    let ast = test_parser(name);
    let typed_ast = TypedAST::new(&ast);

    if let Some(expected_type) = expected {
        assert_eq!(expected_type, typed_ast.ty)
    }

    (ast, typed_ast)
}

fn test_type_checker(name: &str, expected: Option<Type>) -> TypedAST {
    test_typed_ast_and_ast(name, expected).1
}

fn test_typed_ast_and_data(name: &str, expected: Option<(Type, Data)>) -> (TypedAST, Data) {
    let option_expected_type = match expected.clone() {
        Some(expect) => Some(expect.0),
        None => None,
    };
    let (ast, typed_ast) = test_typed_ast_and_ast(name, option_expected_type);
    let data = interpret::interpret(&ast);

    if let Some((expected_type, expected_data)) = expected {
        match expected_type {
            Type::Boolean => match data {
                Data::Boolean(_) => (),
                _ => unreachable!(),
            },
            Type::Number => match data {
                Data::Number(_) => (),
                _ => unreachable!(),
            },
            Type::Function { .. } => match data {
                Data::Function(_) => (),
                _ => unreachable!(),
            },
        };
        assert_eq!(expected_data, data);
    }

    (typed_ast, data)
}

fn test_interpreter(name: &str, expected: Option<(Type, Data)>) -> Data {
    test_typed_ast_and_data(name, expected).1
}

fn compiler_result_to_data(expected: Option<(Type, u64)>) -> Option<(Type, Data, u64)> {
    match expected {
        Some((expected_type, expected_result)) => {
            let expected_data = match expected_result {
                0 => match expected_type {
                    Type::Boolean => Data::Boolean(false),
                    Type::Number => Data::Number(0),
                    Type::Function { .. } => unreachable!(),
                },
                1 => match expected_type {
                    Type::Boolean => Data::Boolean(true),
                    Type::Number => Data::Number(1),
                    Type::Function { .. } => unreachable!(),
                },
                number => Data::Number(number.try_into().unwrap()),
            };
            Some((expected_type, expected_data, expected_result))
        }
        None => None,
    }
}

fn test_compiler(name: &str, expected: Option<(Type, u64)>) -> u64 {
    let expected_with_data = compiler_result_to_data(expected.clone());
    let option_expected = match expected_with_data {
        Some(expect) => Some((expect.0, expect.1)),
        None => None,
    };
    let (typed_ast, _) = test_typed_ast_and_data(name, option_expected);
    let compiler_result = codegen::run(&typed_ast).unwrap();

    if let Some((_, expected_result)) = expected {
        assert_eq!(expected_result, compiler_result);
    }

    compiler_result
}

fn test(name: &str, options: TestOptions) {
    options.check();

    match options {
        TestOptions::Parse => {
            let _ = test_parser(name);
        }
        TestOptions::TypeCheck(expected_type) => {
            let _ = test_type_checker(name, expected_type);
        }
        TestOptions::Interpret(expected) => {
            let _ = test_interpreter(name, expected);
        }
        TestOptions::Compile(expected) => {
            let _ = test_compiler(name, expected);
        }
    }
}

#[test]
fn advanced() {
    test("advanced", TestOptions::Compile(Some((Type::Number, 15))));
}

#[test]
fn basic() {
    test("basic", TestOptions::Compile(Some((Type::Boolean, 0))));
}

#[test]
fn factorial() {
    test(
        "factorial",
        TestOptions::Interpret(Some((Type::Number, Data::Number(120)))),
    )
}

#[test]
#[should_panic]
fn fail_function_two_plus_one() {
    test("fail_function_two_plus_one", TestOptions::TypeCheck(None));
}

#[test]
fn false_literal() {
    test(
        "false_literal",
        TestOptions::Compile(Some((Type::Boolean, 0))),
    );
}

#[test]
#[should_panic]
fn function_application_parse_fail_2() {
    test("function_application_parse_fail_2", TestOptions::Parse);
}

#[test]
#[should_panic]
fn function_application_parse_fail() {
    test("function_application_parse_fail", TestOptions::Parse);
}

#[test]
fn function_if_argument() {
    test(
        "function_if_argument",
        TestOptions::Compile(Some((Type::Number, 1))),
    );
}

#[test]
fn function_plus_one() {
    let argument = "x".to_string();
    test(
        "function_plus_one",
        TestOptions::Interpret(Some((
            Type::Function {
                argument: Box::new(Type::Number),
                ret: Box::new(Type::Number),
            },
            Data::Function(Function {
                argument_name: argument.clone(),
                body: AST::Plus(
                    Box::new(AST::Identifier(argument)),
                    Box::new(AST::NumberLiteral(1)),
                ),
            }),
        ))),
    );
}

#[test]
fn function_two_plus_one() {
    test(
        "function_two_plus_one",
        TestOptions::Compile(Some((Type::Number, 3))),
    );
}

#[test]
fn identity_function() {
    let argument = "x".to_string();
    test(
        "identity_function",
        TestOptions::Interpret(Some((
            Type::Function {
                argument: Box::new(Type::Number),
                ret: Box::new(Type::Number),
            },
            Data::Function(Function {
                argument_name: argument.clone(),
                body: AST::Identifier(argument),
            }),
        ))),
    );
}

#[test]
fn if_false() {
    test("if_false", TestOptions::Compile(Some((Type::Boolean, 0))));
}

#[test]
fn if_test() {
    test("if", TestOptions::Compile(Some((Type::Boolean, 0))));
}

#[test]
#[should_panic] // TODO: BUG: implement lambdas
fn is_even() {
    test("is_even", TestOptions::Compile(Some((Type::Boolean, 0))));
}

#[test]
fn medium() {
    test(
        "medium",
        TestOptions::Interpret(Some((
            Type::Function {
                argument: Box::new(Type::Number),
                ret: Box::new(Type::Number),
            },
            Data::Function(Function {
                argument_name: "n".to_string(),
                body: AST::NumberLiteral(52),
            }),
        ))),
    );
}

#[test]
#[should_panic] // TODO: BUG: implement lambdas
fn nested_function() {
    test(
        "nested_function",
        TestOptions::Compile(Some((Type::Number, 15))),
    );
}

#[test]
fn number_literal() {
    test(
        "number_literal",
        TestOptions::Compile(Some((Type::Number, 2))),
    );
}

#[test]
fn one_equal_two() {
    test(
        "one_equal_two",
        TestOptions::Compile(Some((Type::Boolean, 0))),
    );
}

#[test]
fn plus() {
    test("plus", TestOptions::Compile(Some((Type::Number, 3))));
}

#[test]
#[should_panic]
fn rec_c_fail() {
    test("rec_c_fail", TestOptions::TypeCheck(None));
}

#[test]
fn summation() {
    test(
        "summation",
        TestOptions::Interpret(Some((Type::Number, Data::Number(55)))),
    );
}

#[test]
fn super_test() {
    test("super", TestOptions::Compile(Some((Type::Boolean, 0))));
}

#[test]
fn true_literal() {
    test(
        "true_literal",
        TestOptions::Compile(Some((Type::Boolean, 1))),
    );
}

#[test]
fn unbound_identifier() {
    test("true_literal", TestOptions::TypeCheck(None));
}

#[test]
#[should_panic] // TODO: BUG: implement lambdas
fn undecidable_nested_function() {
    let argument = "inner_argument".to_string();
    test(
        "undecidable_nested_function",
        TestOptions::Interpret(Some((
            Type::Function {
                argument: Box::new(Type::Boolean),
                ret: Box::new(Type::Boolean),
            },
            Data::Function(Function {
                argument_name: argument.clone(),
                body: AST::Identifier(argument),
            }),
        ))),
    );
}

#[test]
fn unused_rec() {
    test("unused_rec", TestOptions::Compile(Some((Type::Boolean, 1))))
}
