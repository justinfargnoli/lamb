#![allow(non_snake_case)]

use crate::parse::AST;
use std::{collections::HashMap, fmt, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    NumT,
    BoolT,
    FunT { arg: Box<Type>, ret: Box<Type> },
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::NumT => write!(f, "NumT"),
            Type::BoolT => write!(f, "BoolT"),
            Type::FunT { arg, ret } => write!(f, "FunT({}, {})", arg, ret),
        }
    }
}

pub fn run(ast: &AST) -> Type {
    tc_helper(ast, &mut HashMap::new())
}

fn tc_helper(ast: &AST, tenv: &mut HashMap<String, Type>) -> Type {
    match ast {
        AST::TrueC => Type::BoolT,
        AST::FalseC => Type::BoolT,
        AST::NumC(_) => Type::NumT,
        AST::PlusC(op1, op2) => {
            if tc_helper(&op1, tenv) == Type::NumT && tc_helper(&op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in PlusC!")
            }
        }
        AST::MultC(op1, op2) => {
            if tc_helper(&op1, tenv) == Type::NumT && tc_helper(&op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in MultC!")
            }
        }
        AST::EqC(op1, op2) => {
            let op1_type = tc_helper(&op1, tenv);
            let op2_type = tc_helper(&op2, tenv);

            if let Type::FunT { .. } = op1_type {
                panic!("EqC cannot comapre type FunT")
            } else if let Type::FunT { .. } = op2_type {
                panic!("EqC cannot comapre type FunT")
            } else if op1_type != op2_type {
                panic!("Types differ in EqC!")
            }

            Type::BoolT
        }
        AST::IfC(ifCStruct) => {
            if tc_helper(&ifCStruct.cnd, tenv) != Type::BoolT {
                panic!("Condition in an if statement is not boolean!")
            }
            let then_type = tc_helper(&ifCStruct.then, tenv);
            let else_type = tc_helper(&ifCStruct.els, tenv);
            if then_type == else_type {
                then_type
            } else {
                panic!("Types differ in then and else part of an if statement!")
            }
        }
        AST::IdC(id) => {
            if tenv.contains_key(id) {
                tenv[id].clone()
            } else {
                panic!("Variable not saved in type environment")
            }
        }
        AST::AppC(appCStruct) => {
            let fun_type = tc_helper(&appCStruct.func, tenv);
            let arg_type = tc_helper(&appCStruct.arg, tenv);
            match fun_type {
                Type::FunT {
                    arg: funT_arg,
                    ret: funT_ret,
                } => {
                    if arg_type == *funT_arg {
                        *funT_ret //dereferencing the box type
                    } else {
                        panic!("Argument type doesn't match declared type")
                    }
                }
                _ => panic!("Not a function in appC"),
            }
        }
        AST::RecC(recCStruct) => {
            tenv.insert(
                recCStruct.func_name.clone(),
                Type::FunT {
                    arg: Box::new(recCStruct.arg_type.clone()),
                    ret: Box::new(recCStruct.ret_type.clone()),
                },
            );
            tenv.insert(recCStruct.arg_name.clone(), recCStruct.arg_type.clone());
            if recCStruct.ret_type == tc_helper(&recCStruct.body, tenv) {
                let ret_type = tc_helper(&recCStruct.func_use, tenv);
                tenv.remove(&recCStruct.func_name);
                tenv.remove(&recCStruct.arg_name);
                ret_type
            } else {
                panic!("Return type of recursive function does not match return type of the body!");
            }
        }
        AST::FdC(fdCStruct) => {
            tenv.insert(fdCStruct.arg_name.clone(), fdCStruct.arg_type.clone());
            let body_ret = tc_helper(&fdCStruct.body, tenv);
            if body_ret == fdCStruct.ret_type {
                /*
                 * Since the body has type checked we can remove the varaible name form the scope to
                 * preseve a common understanding of scope. This allows us ot avoid cloning the HashMap.
                 */
                tenv.remove(&fdCStruct.arg_name);
                Type::FunT {
                    arg: Box::new(fdCStruct.arg_type.clone()),
                    ret: Box::new(fdCStruct.ret_type.clone()),
                }
            } else {
                panic!("Body type doesn't match declared type")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{AppCStruct, FdCStruct, RecCStruct};

    #[test]
    fn eq_c() {
        let input = Box::new(AST::EqC(Box::new(AST::NumC(0)), Box::new(AST::NumC(-5))));
        assert_eq!(run(&input), Type::BoolT);
    }

    #[test]
    #[should_panic]
    fn eq_c_fail_incompatible_type() {
        let input = Box::new(AST::EqC(Box::new(AST::TrueC), Box::new(AST::NumC(-984))));
        run(&input);
    }

    #[test]
    #[should_panic]
    fn eq_c_fail_comparing_functions() {
        let input = Box::new(AST::EqC(
            Box::new(AST::FdC(FdCStruct {
                arg_name: String::from("a"),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::IdC(String::from("a"))),
            })),
            Box::new(AST::FdC(FdCStruct {
                arg_name: String::from("a"),
                arg_type: Type::NumT,
                ret_type: Type::NumT,
                body: Box::new(AST::IdC(String::from("a"))),
            })),
        ));
        run(&input);
    }

    #[test]
    fn ec_c_ret_type() {
        let input = Box::new(AST::RecC(RecCStruct {
            func_name: String::from("func"),
            arg_name: String::from("arg"),
            arg_type: Type::NumT,
            ret_type: Type::NumT,
            body: Box::new(AST::IdC(String::from("arg"))),
            func_use: Box::new(AST::EqC(
                Box::new(AST::NumC(1)),
                Box::new(AST::AppC(AppCStruct {
                    func: Box::new(AST::IdC(String::from("func"))),
                    arg: Box::new(AST::NumC(1)),
                })),
            )),
        }));
        assert_eq!(run(&input), Type::BoolT);
    }
}
