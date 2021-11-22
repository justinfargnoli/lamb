use crate::parse::AST;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone)]
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

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Function {
    arg_name: String,
    body: AST,
}

pub fn interpret(ast: AST) -> Data {
    interpreter(ast, &mut HashMap::new())
}

fn interpreter(ast: AST, map: &mut HashMap<String, Vec<Data>>) -> Data {
    match ast {
        AST::NumC(number) => Data::Number(number),
        AST::PlusC(op1, op2) => {
            Data::Number(interpreter(*op1, map).number() + interpreter(*op2, map).number())
        }
        AST::MultC(op1, op2) => {
            Data::Number(interpreter(*op1, map).number() * interpreter(*op2, map).number())
        }
        AST::TrueC => Data::Boolean(true),
        AST::FalseC => Data::Boolean(false),
        AST::EqC(lhs, rhs) => {
            let lhs_data = interpreter(*lhs, map);
            let rhs_data = interpreter(*rhs, map);

            if let Data::Function(_) = lhs_data {
                panic!()
            } else if let Data::Function(_) = rhs_data {
                panic!()
            } else {
                Data::Boolean(lhs_data == rhs_data)
            }
        }
        AST::IfC { cnd, then, els } => {
            if interpreter(*cnd, map).boolean() {
                interpreter(*then, map)
            } else {
                interpreter(*els, map)
            }
        }
        AST::IdC(string) => (*map
            .get(&string)
            .unwrap_or_else(|| panic!("Unable to find identifier: {:?}", string.as_str())))
        .last()
        .unwrap()
        .clone(),
        AST::FdC { arg_name, body, .. } => Data::Function(Function {
            arg_name,
            body: *body,
        }),
        AST::AppC { func, arg } => {
            let function = interpreter(*func, map).function();
            let argument = interpreter(*arg, map);

            map.entry(function.arg_name.clone())
                .or_insert_with(Vec::new)
                .push(argument);

            let return_data = interpreter(function.body, map);

            match map.get_mut(&function.arg_name) {
                Some(data_values) => {
                    data_values.pop();
                    if data_values.is_empty() {
                        map.remove(&function.arg_name);
                    }
                }
                None => panic!()
            }

            return_data
        }
        AST::RecC {
            func_name,
            arg_name,
            body,
            func_use,
            ..
        } => {
            map.entry(func_name.clone())
                .or_insert_with(Vec::new)
                .push(Data::Function(Function {
                    arg_name,
                    body: *body,
                })); // add the function to the current scope
            let return_data = interpreter(*func_use, map);
            
            match map.get_mut(&func_name) {
                Some(data_values) => {
                    data_values.pop();
                    if data_values.is_empty() {
                        map.remove(&func_name);
                    }
                }
                None => panic!()
            }

            return_data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_check::Type;

    #[test]
    fn interpret_numc() {
        assert_eq!(interpret(AST::NumC(5)).number(), 5 as i64)
    }

    #[test]
    fn interpret_plusc() {
        assert_eq!(
            interpret(AST::PlusC(Box::new(AST::NumC(5)), Box::new(AST::NumC(-1)))).number(),
            4 as i64
        )
    }

    #[test]
    fn interpret_multc() {
        assert_eq!(
            interpret(AST::MultC(Box::new(AST::NumC(5)), Box::new(AST::NumC(-1)))).number(),
            -5 as i64
        )
    }

    #[test]
    fn interpret_truec() {
        assert_eq!(interpret(AST::TrueC), Data::Boolean(true))
    }

    #[test]
    fn interpret_falsec() {
        assert_eq!(interpret(AST::FalseC), Data::Boolean(false))
    }

    #[test]
    fn interpret_eqc_true() {
        assert_eq!(
            interpret(AST::EqC(Box::new(AST::NumC(0)), Box::new(AST::NumC(0)))),
            Data::Boolean(true)
        )
    }

    #[test]
    fn interpret_eqc_false() {
        assert_eq!(
            interpret(AST::EqC(Box::new(AST::NumC(0)), Box::new(AST::NumC(1)))),
            Data::Boolean(false)
        )
    }

    #[test]
    fn interpret_ifc_then() {
        assert_eq!(
            interpret(AST::IfC {
                cnd: Box::new(AST::TrueC),
                then: Box::new(AST::NumC(88)),
                els: Box::new(AST::NumC(33)),
            }),
            Data::Number(88)
        )
    }

    #[test]
    fn interpret_ifc_els() {
        assert_eq!(
            interpret(AST::IfC {
                cnd: Box::new(AST::FalseC),
                then: Box::new(AST::NumC(88)),
                els: Box::new(AST::NumC(33)),
            }),
            Data::Number(33)
        )
    }

    #[test]
    fn interpret_fdc() {
        let body = AST::IdC("argument".to_string());
        assert_eq!(
            interpret(AST::FdC {
                arg_name: "argument".to_string(),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(body.clone()),
            }),
            Data::Function(Function {
                arg_name: "argument".to_string(),
                body,
            })
        )
    }

    #[test]
    fn interpret_appc() {
        let body = AST::IdC("argument".to_string());
        assert_eq!(
            interpret(AST::AppC {
                func: Box::new(AST::FdC {
                    arg_name: "argument".to_string(),
                    arg_type: Type::NumT,
                    ret_type: Type::NumT,
                    body: Box::new(body.clone()),
                }),
                arg: Box::new(AST::NumC(-3)),
            }),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_no_call() {
        assert_eq!(
            interpret(AST::RecC {
                func_name: "recursive_fn".to_string(),
                arg_name: "argument".to_string(),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::FdC {
                    arg_name: "argument".to_string(),
                    arg_type: Type::NumT,
                    ret_type: Type::NumT,
                    body: Box::new(AST::IdC("argument".to_string())),
                }),
                func_use: Box::new(AST::NumC(-3)),
            }),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_no_recursion() {
        assert_eq!(
            interpret(AST::RecC {
                func_name: "recursive_fn".to_string(),
                arg_name: "argument".to_string(),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::IdC("argument".to_string())),
                func_use: Box::new(AST::AppC {
                    func: Box::new(AST::IdC("recursive_fn".to_string())),
                    arg: Box::new(AST::NumC(-3))
                }),
            }),
            Data::Number(-3)
        )
    }

    #[test]
    fn interpret_recc_w_recursion() {
        assert_eq!(
            interpret(AST::RecC {
                func_name: "recursive_fn".to_string(),
                arg_name: "argument".to_string(),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::IfC {
                    cnd: Box::new(AST::EqC(
                        Box::new(AST::IdC("argument".to_string())),
                        Box::new(AST::NumC(1))
                    )),
                    then: Box::new(AST::NumC(1)),
                    els: Box::new(AST::AppC {
                        func: Box::new(AST::IdC("recursive_fn".to_string())),
                        arg: Box::new(AST::PlusC(
                            Box::new(AST::IdC("argument".to_string())),
                            Box::new(AST::NumC(-1))
                        ))
                    })
                }),
                func_use: Box::new(AST::AppC {
                    func: Box::new(AST::IdC("recursive_fn".to_string())),
                    arg: Box::new(AST::NumC(3))
                }),
            }),
            Data::Number(1)
        )
    }

    #[test]
    fn interpret_recc_w_recursion_hard() {
        assert_eq!(
            interpret(AST::RecC {
                func_name: "recursive_fn".to_string(),
                arg_name: "argument".to_string(),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::IfC {
                    cnd: Box::new(AST::EqC(
                        Box::new(AST::IdC("argument".to_string())),
                        Box::new(AST::NumC(1))
                    )),
                    then: Box::new(AST::NumC(1)),
                    els: Box::new(AST::MultC(
                        Box::new(AST::IdC("argument".to_string())),
                        Box::new(AST::AppC {
                            func: Box::new(AST::IdC("recursive_fn".to_string())),
                            arg: Box::new(AST::PlusC(
                                Box::new(AST::IdC("argument".to_string())),
                                Box::new(AST::NumC(-1))
                            ))
                        })
                    ))
                }),
                func_use: Box::new(AST::AppC {
                    func: Box::new(AST::IdC("recursive_fn".to_string())),
                    arg: Box::new(AST::NumC(3))
                }),
            }),
            Data::Number(6)
        )
    }
}
