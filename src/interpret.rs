use crate::parse::AST;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
pub enum Data {
    Boolean(bool),
    Number(i64),
    Function(Function),
}

impl Data {
    fn boolean(&self) -> bool {
        if let Data::Boolean(boolean) = self {
            *boolean
        } else {
            panic!("Trying to access a non-active variant of Data enum")
        }
    }

    fn number(&self) -> i64 {
        if let Data::Number(number) = self {
            *number
        } else {
            panic!("Trying to access a non-active variant of Data enum")
        }
    }

    fn function(self) -> Function {
        if let Data::Function(function) = self {
            function
        } else {
            panic!("Trying to access a non-active variant of Data enum")
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Function {
    pub argument_name: String,
    pub body: AST,
}

pub fn interpret(ast: &AST) -> Data {
    interpreter(ast, &mut HashMap::new())
}

fn interpreter(ast: &AST, map: &mut HashMap<String, Vec<Data>>) -> Data {
    match ast {
        AST::NumberLiteral(number) => Data::Number(*number),
        AST::Plus(op1, op2) => {
            Data::Number(interpreter(op1, map).number() + interpreter(op2, map).number())
        }
        AST::Multiply(op1, op2) => {
            Data::Number(interpreter(&*op1, map).number() * interpreter(&*op2, map).number())
        }
        AST::TrueLiteral => Data::Boolean(true),
        AST::FalseLiteral => Data::Boolean(false),
        AST::Equals(lhs, rhs) => {
            let lhs_data = interpreter(lhs, map);
            let rhs_data = interpreter(rhs, map);

            if let Data::Function(_) = lhs_data {
                panic!()
            } else if let Data::Function(_) = rhs_data {
                panic!()
            } else {
                Data::Boolean(lhs_data == rhs_data)
            }
        }
        AST::If(if_struct) => {
            if interpreter(&*if_struct.condition, map).boolean() {
                interpreter(&*if_struct.then, map)
            } else {
                interpreter(&*if_struct.els, map)
            }
        }
        AST::Identifier(string) => (*map
            .get(string)
            .unwrap_or_else(|| panic!("Unable to find identifier: {:?}", string.as_str())))
        .last()
        .unwrap()
        .clone(),
        AST::FunctionDefinition(function_definition) => Data::Function(Function {
            argument_name: function_definition.argument_name.clone(),
            body: *function_definition.body.clone(),
        }),
        AST::FunctionApplication(function_application) => {
            let function = interpreter(&*function_application.function, map).function();
            let argument = interpreter(&*function_application.argument, map);

            map.entry(function.argument_name.clone())
                .or_insert_with(Vec::new)
                .push(argument);

            let return_data = interpreter(&function.body, map);

            match map.get_mut(&function.argument_name) {
                Some(data_values) => {
                    data_values.pop();
                    if data_values.is_empty() {
                        map.remove(&function.argument_name);
                    }
                }
                None => panic!(),
            }

            return_data
        }
        AST::RecursiveFunction(recursive_function) => {
            map.entry(recursive_function.function_name.clone())
                .or_insert_with(Vec::new)
                .push(Data::Function(Function {
                    argument_name: recursive_function.argument_name.clone(),
                    body: *recursive_function.body.clone(),
                })); // add the function to the current scope
            let return_data = interpreter(&*recursive_function.function_use, map);

            match map.get_mut(&recursive_function.function_name) {
                Some(data_values) => {
                    data_values.pop();
                    if data_values.is_empty() {
                        map.remove(&recursive_function.function_name);
                    }
                }
                None => panic!(),
            }

            return_data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parse::{FunctionApplication, FunctionDefinition, If, RecursiveFunction},
        type_check::Type,
    };

    #[test]
    fn interpret_numc() {
        assert_eq!(interpret(&AST::NumberLiteral(5)).number(), 5 as i64)
    }

    #[test]
    fn interpret_plusc() {
        assert_eq!(
            interpret(&AST::Plus(
                Box::new(AST::NumberLiteral(5)),
                Box::new(AST::NumberLiteral(-1))
            ))
            .number(),
            4 as i64
        )
    }

    #[test]
    fn interpret_multc() {
        assert_eq!(
            interpret(&AST::Multiply(
                Box::new(AST::NumberLiteral(5)),
                Box::new(AST::NumberLiteral(-1))
            ))
            .number(),
            -5 as i64
        )
    }

    #[test]
    fn interpret_truec() {
        assert_eq!(interpret(&AST::TrueLiteral), Data::Boolean(true))
    }

    #[test]
    fn interpret_falsec() {
        assert_eq!(interpret(&AST::FalseLiteral), Data::Boolean(false))
    }

    #[test]
    fn interpret_eqc_true() {
        assert_eq!(
            interpret(&AST::Equals(
                Box::new(AST::NumberLiteral(0)),
                Box::new(AST::NumberLiteral(0))
            )),
            Data::Boolean(true)
        )
    }

    #[test]
    fn interpret_eqc_false() {
        assert_eq!(
            interpret(&AST::Equals(
                Box::new(AST::NumberLiteral(0)),
                Box::new(AST::NumberLiteral(1))
            )),
            Data::Boolean(false)
        )
    }

    #[test]
    fn interpret_ifc_then() {
        assert_eq!(
            interpret(&AST::If(If {
                condition: Box::new(AST::TrueLiteral),
                then: Box::new(AST::NumberLiteral(88)),
                els: Box::new(AST::NumberLiteral(33)),
            })),
            Data::Number(88)
        )
    }

    #[test]
    fn interpret_ifc_els() {
        assert_eq!(
            interpret(&AST::If(If {
                condition: Box::new(AST::FalseLiteral),
                then: Box::new(AST::NumberLiteral(88)),
                els: Box::new(AST::NumberLiteral(33)),
            })),
            Data::Number(33)
        )
    }

    #[test]
    fn interpret_fdc() {
        let body = AST::Identifier("argument".to_string());
        assert_eq!(
            interpret(&AST::FunctionDefinition(FunctionDefinition {
                argument_name: "argument".to_string(),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(body.clone()),
            })),
            Data::Function(Function {
                argument_name: "argument".to_string(),
                body,
            })
        )
    }

    #[test]
    fn interpret_appc() {
        let body = AST::Identifier("argument".to_string());
        assert_eq!(
            interpret(&AST::FunctionApplication(FunctionApplication {
                function: Box::new(AST::FunctionDefinition(FunctionDefinition {
                    argument_name: "argument".to_string(),
                    argument_type: Type::Number,
                    return_type: Type::Number,
                    body: Box::new(body.clone()),
                })),
                argument: Box::new(AST::NumberLiteral(-3)),
            })),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_no_call() {
        assert_eq!(
            interpret(&AST::RecursiveFunction(RecursiveFunction {
                function_name: "recursive_fn".to_string(),
                argument_name: "argument".to_string(),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::FunctionDefinition(FunctionDefinition {
                    argument_name: "argument".to_string(),
                    argument_type: Type::Number,
                    return_type: Type::Number,
                    body: Box::new(AST::Identifier("argument".to_string())),
                })),
                function_use: Box::new(AST::NumberLiteral(-3)),
            })),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_no_recursion() {
        assert_eq!(
            interpret(&AST::RecursiveFunction(RecursiveFunction {
                function_name: "recursive_fn".to_string(),
                argument_name: "argument".to_string(),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::Identifier("argument".to_string())),
                function_use: Box::new(AST::FunctionApplication(FunctionApplication {
                    function: Box::new(AST::Identifier("recursive_fn".to_string())),
                    argument: Box::new(AST::NumberLiteral(-3))
                })),
            })),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_w_recursion() {
        assert_eq!(
            interpret(&AST::RecursiveFunction(RecursiveFunction {
                function_name: "recursive_fn".to_string(),
                argument_name: "argument".to_string(),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::If(If {
                    condition: Box::new(AST::Equals(
                        Box::new(AST::Identifier("argument".to_string())),
                        Box::new(AST::NumberLiteral(1))
                    )),
                    then: Box::new(AST::NumberLiteral(1)),
                    els: Box::new(AST::FunctionApplication(FunctionApplication {
                        function: Box::new(AST::Identifier("recursive_fn".to_string())),
                        argument: Box::new(AST::Plus(
                            Box::new(AST::Identifier("argument".to_string())),
                            Box::new(AST::NumberLiteral(-1))
                        ))
                    }))
                })),
                function_use: Box::new(AST::FunctionApplication(FunctionApplication {
                    function: Box::new(AST::Identifier("recursive_fn".to_string())),
                    argument: Box::new(AST::NumberLiteral(3))
                })),
            })),
            Data::Number(1)
        )
    }

    #[test]
    fn interpret_recc_w_recursion_hard() {
        assert_eq!(
            interpret(&AST::RecursiveFunction(RecursiveFunction {
                function_name: "recursive_fn".to_string(),
                argument_name: "argument".to_string(),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::If(If {
                    condition: Box::new(AST::Equals(
                        Box::new(AST::Identifier("argument".to_string())),
                        Box::new(AST::NumberLiteral(1))
                    )),
                    then: Box::new(AST::NumberLiteral(1)),
                    els: Box::new(AST::Multiply(
                        Box::new(AST::Identifier("argument".to_string())),
                        Box::new(AST::FunctionApplication(FunctionApplication {
                            function: Box::new(AST::Identifier("recursive_fn".to_string())),
                            argument: Box::new(AST::Plus(
                                Box::new(AST::Identifier("argument".to_string())),
                                Box::new(AST::NumberLiteral(-1))
                            ))
                        }))
                    ))
                })),
                function_use: Box::new(AST::FunctionApplication(FunctionApplication {
                    function: Box::new(AST::Identifier("recursive_fn".to_string())),
                    argument: Box::new(AST::NumberLiteral(3))
                })),
            })),
            Data::Number(6)
        )
    }
}
