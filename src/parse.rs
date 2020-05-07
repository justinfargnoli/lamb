use super::type_check::Type;
use crate::tokenize::{Token, TokenStream};

#[derive(Debug, PartialEq)]
pub enum AST {
    NumC,
    PlusC(Box<AST>, Box<AST>),
    MultC(Box<AST>, Box<AST>),
    TrueC,
    FalseC,
    EqC(Box<AST>, Box<AST>),
    IfC {
        cnd: Box<AST>,
        then: Box<AST>,
        els: Box<AST>,
    },
    IdC(String),
    AppC {
        func: Box<AST>,
        arg: Box<AST>,
    },
    FdC {
        arg_name: String,
        arg_type: Type,
        ret_type: Type,
        body: Box<AST>,
    },
    RecC {
        func_name: String,
        arg_name: String,
        arg_type: Type,
        ret_type: Type,
        body: Box<AST>,
        func_use: Box<AST>,
    },
}

impl AST {
    pub fn build(token_stream: &mut TokenStream) -> AST {
        match token_stream.next() {
            Some(token) => {
                match token {
                    Token::TrueC => AST::TrueC,
                    Token::FalseC => AST::FalseC,
                    Token::NumC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(
                            std::mem::discriminant(&Token::Number(0 /* value doesn't matter */)),
                            std::mem::discriminant(&token_stream.next().unwrap())
                        );
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::NumC
                    }
                    Token::PlusC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::PlusC(Box::new(ast1), Box::new(ast2))
                    }
                    Token::MultC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::MultC(Box::new(ast1), Box::new(ast2))
                    }
                    Token::IfC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast3 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::IfC {
                            cnd: Box::new(ast1),
                            then: Box::new(ast2),
                            els: Box::new(ast3),
                        }
                    }
                    Token::IdC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let string_ast;
                        match token_stream.next().unwrap() {
                            Token::ID(id) => {
                                string_ast = AST::IdC(id);
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        string_ast
                    }
                    Token::AppC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::AppC {
                            func: Box::new(ast1),
                            arg: Box::new(ast2),
                        }
                    }
                    Token::FdC => {
                        //THE ARGUMENT NAME
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let string_ast;
                        match token_stream.next().unwrap() {
                            Token::ID(val) => {
                                string_ast = val;
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        //THE ARGUMENT TYPE
                        let arg_type = AST::parse_type(token_stream);

                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        //THE RETURN TYPE
                        let ret_type = AST::parse_type(token_stream);

                        assert_eq!(Token::Comma, token_stream.next().unwrap());

                        //THE BODY
                        let ast_body = AST::build(token_stream);

                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());

                        AST::FdC {
                            arg_name: string_ast,
                            arg_type,
                            ret_type,
                            body: Box::new(ast_body),
                        }
                    }
                    Token::EqC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::EqC(Box::new(ast1), Box::new(ast2))
                    }
                    Token::RecC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        // 1st parameter
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let rec_func_name;
                        match token_stream.next().unwrap() {
                            Token::ID(val) => {
                                //Token::ID, not to be confused with idC
                                rec_func_name = val;
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 2nd parameter
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let rec_arg_name;
                        match token_stream.next().unwrap() {
                            Token::ID(val) => {
                                //Token::ID, not to be confused with idC
                                rec_arg_name = val;
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 3rd parameter
                        let rec_arg_type = AST::parse_type(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 4th parameter
                        let rec_ret_type = AST::parse_type(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 5th parameter
                        let rec_body_ast = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 6th parameter
                        let rec_func_use_ast = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::RecC {
                            func_name: rec_func_name,
                            arg_name: rec_arg_name,
                            arg_type: rec_arg_type,
                            ret_type: rec_ret_type,
                            body: Box::new(rec_body_ast),
                            func_use: Box::new(rec_func_use_ast),
                        }
                    }
                    _ => panic!("Parsing error"), ////TODO: THIS should never happen
                }
            }
            None => panic!("No token found"),
        }
    }

    fn parse_type(token_stream: &mut TokenStream) -> Type {
        match token_stream.next() {
            Some(token) => match token {
                Token::NumT => Type::NumT,
                Token::BoolT => Type::BoolT,
                Token::FunT => {
                    assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                    let box1 = AST::parse_type(token_stream);
                    assert_eq!(Token::Comma, token_stream.next().unwrap());
                    let box2 = AST::parse_type(token_stream);
                    assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                    Type::FunT {
                        arg: Box::new(box1),
                        ret: Box::new(box2),
                    }
                }
                _ => panic!("Argument type not found!"),
            },
            None => panic!("No token found when parsing type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn parse_1() {
        //testing numC(1)
        let tokens = VecDeque::from(vec![
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(AST::build(&mut token_stream), AST::NumC);
    }

    #[test]
    fn parse_2() {
        //testing plusC(numC(1), numC(2))
        let tokens = VecDeque::from(vec![
            Token::PlusC,
            Token::ParenLeft,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::PlusC(Box::new(AST::NumC), Box::new(AST::NumC))
        );
    }

    #[test]
    #[should_panic]
    fn parse_3() {
        //testing plusC(1, numC(2)), this should panic
        let tokens = VecDeque::from(vec![
            Token::PlusC,
            Token::ParenLeft,
            Token::Number(1),
            Token::Comma,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    fn parse_4() {
        //testing multC(numC(1), numC(2))
        let tokens = VecDeque::from(vec![
            Token::MultC,
            Token::ParenLeft,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::MultC(Box::new(AST::NumC), Box::new(AST::NumC))
        );
    }

    #[test]
    #[should_panic]
    fn parse_plus_c() {
        //testing plusC(numC(1), numC(2) -> this should panic (missing right parenthesis)
        let tokens = VecDeque::from(vec![
            Token::PlusC,
            Token::ParenLeft,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            // Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    fn parse_if_c_1() {
        //testing if(true, true, false)
        let tokens = VecDeque::from(vec![
            Token::IfC,
            Token::ParenLeft,
            Token::TrueC,
            Token::Comma,
            Token::TrueC,
            Token::Comma,
            Token::FalseC,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::IfC {
                cnd: Box::new(AST::TrueC),
                then: Box::new(AST::TrueC),
                els: Box::new(AST::FalseC)
            }
        );
    }
    #[test]
    #[should_panic]
    fn parse_if_c_2() {
        //testing if(true, true false)
        let tokens = VecDeque::from(vec![
            Token::IfC,
            Token::ParenLeft,
            Token::TrueC,
            Token::Comma,
            Token::TrueC,
            Token::FalseC,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
    #[test]
    fn parse_8() {
        //testing id("x")
        let tokens = VecDeque::from(vec![
            Token::IdC,
            Token::ParenLeft,
            Token::Quote,
            Token::ID("x".to_string()),
            Token::Quote,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(AST::build(&mut token_stream), AST::IdC("x".to_string()));
    }
    #[test]
    #[should_panic]
    fn parse_9() {
        //testing id("x)
        let tokens = VecDeque::from(vec![
            Token::IdC,
            Token::ParenLeft,
            Token::Quote,
            Token::ID("x".to_string()),
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
    //Tests fir appC and fdC omitted here, done externally.

    #[test]
    fn parse_eq_c() {
        let tokens = VecDeque::from(vec![
            Token::EqC,
            Token::ParenLeft,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    #[should_panic]
    fn parse_eq_c_fail() {
        let tokens = VecDeque::from(vec![
            Token::EqC,
            Token::ParenLeft,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::NumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
}
