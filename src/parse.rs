use crate::tokenize::{Token, TokenStream};
use crate::Type;

#[derive(Debug, PartialEq)]
pub enum AST {
    Anumc,
    AplusC(Box<AST>, Box<AST>),
    AmultC(Box<AST>, Box<AST>),
    AtrueC,
    AfalseC,
    AeqC(Box<AST>, Box<AST>),
    AifC {
        cnd: Box<AST>,
        then: Box<AST>,
        els: Box<AST>,
    },
    AidC(String),
    AappC {
        func: Box<AST>,
        arg: Box<AST>,
    },
    AfdC {
        arg_name: String,
        arg_type: Box<Type>,
        ret_type: Box<Type>,
        body: Box<AST>,
    },
    ArecC {
        func_name: String,
        arg_name: String,
        arg_type: Box<Type>,
        ret_type: Box<Type>,
        body: Box<AST>,
        func_use: Box<AST>,
    },
}

impl AST {
    pub fn build(token_stream: &mut TokenStream) -> Box<AST> {
        match token_stream.next() {
            Some(token) => {
                match token {
                    Token::TTrueC => Box::new(AST::AtrueC),
                    Token::TFalseC => Box::new(AST::AfalseC),
                    Token::TNumC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(
                            std::mem::discriminant(&Token::Number(0 /* value doesn't matter */)),
                            std::mem::discriminant(&token_stream.next().unwrap())
                        );
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::Anumc)
                    }
                    Token::TPlusC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::AplusC(ast1, ast2))
                    }
                    Token::TMultC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::AmultC(ast1, ast2))
                    }
                    Token::TIfC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast3 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::AifC {
                            cnd: ast1,
                            then: ast2,
                            els: ast3,
                        })
                    }
                    Token::TIdC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let string_ast;
                        match token_stream.next().unwrap() {
                            Token::ID(id) => {
                                string_ast = Box::new(AST::AidC(id));
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        string_ast
                    }
                    Token::TAppC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::AappC {
                            func: ast1,
                            arg: ast2,
                        })
                    }
                    Token::TFdC => {
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

                        Box::new(AST::AfdC {
                            arg_name: string_ast,
                            arg_type,
                            ret_type,
                            body: ast_body,
                        })
                    }
                    Token::TEqC => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        Box::new(AST::AeqC(ast1, ast2))
                    }
                    Token::TRecC => {
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
                        Box::new(AST::ArecC {
                            func_name: rec_func_name,
                            arg_name: rec_arg_name,
                            arg_type: rec_arg_type,
                            ret_type: rec_ret_type,
                            body: rec_body_ast,
                            func_use: rec_func_use_ast,
                        })
                    }
                    _ => panic!("Parsing error"), ////TODO: THIS should never happen
                }
            }
            None => panic!("No token found"),
        }
    }

    fn parse_type(token_stream: &mut TokenStream) -> Box<Type> {
        match token_stream.next() {
            Some(token) => match token {
                Token::NumT => Box::new(Type::NumT),
                Token::BoolT => Box::new(Type::BoolT),
                Token::FunT => {
                    assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                    let box1 = AST::parse_type(token_stream);
                    assert_eq!(Token::Comma, token_stream.next().unwrap());
                    let box2 = AST::parse_type(token_stream);
                    assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                    Box::new(Type::FunT {
                        arg: box1,
                        ret: box2,
                    })
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
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(*AST::build(&mut token_stream), AST::Anumc);
    }

    #[test]
    fn parse_2() {
        //testing plusC(numC(1), numC(2))
        let tokens = VecDeque::from(vec![
            Token::TPlusC,
            Token::ParenLeft,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            *AST::build(&mut token_stream),
            AST::AplusC(Box::new(AST::Anumc), Box::new(AST::Anumc))
        );
    }

    #[test]
    #[should_panic]
    fn parse_3() {
        //testing plusC(1, numC(2)), this should panic
        let tokens = VecDeque::from(vec![
            Token::TPlusC,
            Token::ParenLeft,
            Token::Number(1),
            Token::Comma,
            Token::TNumC,
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
            Token::TMultC,
            Token::ParenLeft,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            *AST::build(&mut token_stream),
            AST::AmultC(Box::new(AST::Anumc), Box::new(AST::Anumc))
        );
    }

    #[test]
    #[should_panic]
    fn parse_plus_c() {
        //testing plusC(numC(1), numC(2) -> this should panic (missing right parenthesis)
        let tokens = VecDeque::from(vec![
            Token::TPlusC,
            Token::ParenLeft,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::TNumC,
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
            Token::TIfC,
            Token::ParenLeft,
            Token::TTrueC,
            Token::Comma,
            Token::TTrueC,
            Token::Comma,
            Token::TFalseC,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            *AST::build(&mut token_stream),
            AST::AifC {
                cnd: Box::new(AST::AtrueC),
                then: Box::new(AST::AtrueC),
                els: Box::new(AST::AfalseC)
            }
        );
    }
    #[test]
    #[should_panic]
    fn parse_if_c_2() {
        //testing if(true, true false)
        let tokens = VecDeque::from(vec![
            Token::TIfC,
            Token::ParenLeft,
            Token::TTrueC,
            Token::Comma,
            Token::TTrueC,
            Token::TFalseC,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
    #[test]
    fn parse_8() {
        //testing id("x")
        let tokens = VecDeque::from(vec![
            Token::TIdC,
            Token::ParenLeft,
            Token::Quote,
            Token::ID("x".to_string()),
            Token::Quote,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(*AST::build(&mut token_stream), AST::AidC("x".to_string()));
    }
    #[test]
    #[should_panic]
    fn parse_9() {
        //testing id("x)
        let tokens = VecDeque::from(vec![
            Token::TIdC,
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
            Token::TEqC,
            Token::ParenLeft,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::Comma,
            Token::TNumC,
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
            Token::TEqC,
            Token::ParenLeft,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(1),
            Token::ParenRight,
            Token::TNumC,
            Token::ParenLeft,
            Token::Number(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
}
