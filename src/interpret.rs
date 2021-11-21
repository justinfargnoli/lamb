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

    fn into_function(self) -> Function {
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

fn interpreter(ast: AST, map: &mut HashMap<String, Data>) -> Data {
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
        AST::EqC(op1, op2) => Data::Boolean(interpreter(*op1, map) == interpreter(*op2, map)),
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
        .clone(),
        AST::AppC { func, arg } => {
            let function = interpreter(*func, map).into_function();
            let argument = interpreter(*arg, map);
            map.insert(function.arg_name.clone(), argument);
            let func_ret = interpreter(function.body, map);
            println!("appC: {:#?}", function.arg_name);
            map.remove(&function.arg_name).unwrap();
            func_ret
        }
        AST::FdC { arg_name, body, .. } => Data::Function(Function {
            arg_name,
            body: *body,
        }),
        AST::RecC {
            func_name,
            arg_name,
            body,
            func_use,
            ..
        } => {
            map.insert(
                func_name,
                Data::Function(Function {
                    arg_name,
                    body: *body,
                }),
            ); // add the function to the current scope
            interpreter(*func_use, map)
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
    fn interpret_recc() {
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
                    els: Box::new(AST::MultC(
                        Box::new(AST::IdC("argument".to_string())),
                        Box::new(AST::AppC {
                            func: Box::new(AST::IdC("function".to_string())),
                            arg: Box::new(AST::PlusC(
                                Box::new(AST::IdC("function".to_string())),
                                Box::new(AST::PlusC(
                                    Box::new(AST::IdC("argument".to_string())),
                                    Box::new(AST::NumC(-1))
                                ))
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
