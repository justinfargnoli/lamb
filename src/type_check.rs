#![allow(non_snake_case)]

use crate::parse::AST;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Type {
    NumT,
    BoolT,
    FunT { arg: Box<Type>, ret: Box<Type> },
}

pub fn tc(ast: AST, tenv: &mut HashMap<String, Type>) -> Type {
    match ast {
        AST::TrueC => Type::BoolT,
        AST::FalseC => Type::BoolT,
        AST::NumC(_) => Type::NumT,
        AST::PlusC(op1, op2) => {
            if tc(*op1, tenv) == Type::NumT && tc(*op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in PlusC!")
            }
        }
        AST::MultC(op1, op2) => {
            if tc(*op1, tenv) == Type::NumT && tc(*op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in MultC!")
            }
        }
        AST::EqC(operand1, operand2) => {
            if tc(*operand1, tenv) == tc(*operand2, tenv) {
                Type::BoolT
            } else {
                panic!("Types differ in MultC!")
            }
        }
        AST::IfC { cnd, then, els } => {
            if tc(*cnd, tenv) != Type::BoolT {
                panic!("Condition in an if statement is not boolean!")
            }
            let then_type = tc(*then, tenv);
            let else_type = tc(*els, tenv);
            if then_type == else_type {
                then_type
            } else {
                panic!("Types differ in then and else part of an if statement!")
            }
        }
        AST::IdC(id) => {
            if tenv.contains_key(&id) {
                tenv[&id].clone()
            } else {
                panic!("Variable not saved in type environment")
            }
        }
        AST::AppC { func, arg } => {
            let fun_type = tc(*func, tenv);
            let arg_type = tc(*arg, tenv);
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
        AST::RecC {
            func_name,
            arg_name,
            arg_type,
            ret_type,
            body,
            func_use,
        } => {
            tenv.insert(
                func_name.clone(),
                Type::FunT {
                    arg: Box::new(arg_type.clone()),
                    ret: Box::new(ret_type.clone()),
                },
            );
            tenv.insert(arg_name.clone(), arg_type);
            if ret_type == tc(*body, tenv) {
                tc(*func_use, tenv);
                tenv.remove(&func_name);
                tenv.remove(&arg_name);
                ret_type
            } else {
                panic!("Return type of recursive function does not match return type of the body!");
            }
        }
        AST::FdC {
            arg_name,
            arg_type,
            ret_type,
            body,
        } => {
            // let ext_tenv = tenv/*.clone()*/;
            tenv.insert(arg_name.clone(), arg_type.clone());
            let body_ret = tc(*body, tenv);
            if body_ret == ret_type {
                /*
                 * Since the body has type checked we can remove the varaible name form the scope to preseve a common understanding of scope.
                 * This allows us ot avoid cloning the HashMap
                 */
                tenv.remove(&arg_name);
                Type::FunT {
                    arg: Box::new(arg_type),
                    ret: Box::new(ret_type),
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

    #[test]
    fn tc_eq_c() {
        let input = Box::new(AST::EqC(Box::new(AST::NumC(0)), Box::new(AST::NumC(-5))));
        assert_eq!(tc(*input, &mut HashMap::new()), Type::BoolT);
    }

    #[test]
    #[should_panic]
    fn tc_eq_c_fail() {
        let input = Box::new(AST::EqC(Box::new(AST::TrueC), Box::new(AST::NumC(-984))));
        tc(*input, &mut HashMap::new());
    }
}
