use crate::tokenize::{Token, TokenStream};
use crate::Type;

#[derive(Debug, PartialEq)]
pub enum AST {
    Anumc,
    AplusC(Box<AST>, Box<AST>),
    AmultC(Box<AST>, Box<AST>),
    AtrueC,
    AfalseC,
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
}

impl AST {
    pub fn build(token_stream: &mut TokenStream) -> Box<AST> {
        match token_stream.next() {
            Some(token) => {
                match token {
                    Token::TTrueC => Box::new(AST::AtrueC),
                    Token::TFalseC => Box::new(AST::AfalseC),
                    Token::TNumC => {
                        assert_eq!(Token::TNumC, token_stream.next().unwrap());
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
                        //TODO: CHECK!!!!
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
                        //TODO: CHECK!!!!
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

                        let ast_body = AST::build(token_stream);

                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());

                        Box::new(AST::AfdC {
                            arg_name: string_ast,
                            arg_type: arg_type,
                            ret_type: ret_type,
                            body: ast_body,
                        }) //TODO: NOT ACTUALLY THIS
                    }
                    _ => Box::new(AST::AfalseC), ////TODO: THIS should never happen
                }
            }
            None => panic!("No token found"),
        }
        // unimplemented!()
    }
    pub fn parse_type(token_stream: &mut TokenStream) -> Box<Type> {
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
            None => panic!("No token found in parsing type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // #[test]
    // fn parse_1() {
    //     //testing numC(1)
    //     let tokens = VecDeque::from(vec![
    //         Token::TNumC,
    //         Token::ParenLeft,
    //         Token::Number(1),
    //         Token::ParenRight,
    //     ]);
    //     let mut token_stream = TokenStream::build_test(tokens, 0);
    //     assert_eq!(AST::build(&mut token_stream), Box::new(AST::Anumc));
    // }
}
