#![allow(non_snake_case)]

use crate::parse::AST;
use crate::Type;
use std::collections::HashMap;

pub fn tc(ast: Box<AST>, tenv: &mut HashMap<String, Type>) -> Type {
    match *ast {
        AST::AtrueC => Type::BoolT,
        AST::AfalseC => Type::BoolT,
        AST::Anumc => Type::NumT,
        AST::AplusC(op1, op2) => {
            if tc(op1, tenv) == Type::NumT && tc(op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in AplusC!")
            }
        }
        AST::AmultC(op1, op2) => {
            if tc(op1, tenv) == Type::NumT && tc(op2, tenv) == Type::NumT {
                Type::NumT
            } else {
                panic!("Types differ in AmultC!")
            }
        }
        AST::AifC { cnd, then, els } => {
            if tc(cnd, tenv) != Type::BoolT {
                panic!("Condition in an if statement is not boolean!")
            }
            let then_type = tc(then, tenv);
            let else_type = tc(els, tenv);
            if then_type == else_type {
                then_type
            } else {
                panic!("Types differ in then and else part of an if statement!")
            }
        }
        AST::AidC(id) => {
            if tenv.contains_key(&id) {
                tenv[&id].clone()
            } else {
                panic!("Variable not saved in type environment")
            }
        }
        AST::AappC { func, arg } => {
            let fun_type = tc(func, tenv);
            let arg_type = tc(arg, tenv);
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
        AST::AfdC {
            arg_name,
            arg_type,
            ret_type,
            body,
        } => {
            // let ext_tenv = tenv/*.clone()*/;
            tenv.insert(arg_name.clone(), *arg_type.clone());
            let body_ret = tc(body, tenv);
            if body_ret == *ret_type {
                /*
                 * Since the body has type checked we can remove the varaible name form the scope to preseve a common understanding of scope.
                 * This allows us ot avoid cloning the HashMap
                 */
                tenv.remove(&arg_name);
                Type::FunT {
                    arg: arg_type,
                    ret: ret_type,
                }
            } else {
                panic!("Body type doesn't match declared type")
            }
        }
    }
}
