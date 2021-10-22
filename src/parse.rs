use super::type_check::Type;
use crate::tokenize::{Token, TokenStream};

#[derive(Debug, PartialEq)]
pub enum AST {
    NumberLiteral(i64),
    Plus(Box<AST>, Box<AST>),
    Multiply(Box<AST>, Box<AST>),
    TrueLiteral,
    FalseLiteral,
    Equals(Box<AST>, Box<AST>),
    If(If),
    Identifier(String),
    FunctionApplication(FunctionApplication),
    FunctionDefinition(FunctionDefinition),
    RecursiveFunction(RecursiveFunction),
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Box<AST>,
    pub then: Box<AST>,
    pub els: Box<AST>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionApplication {
    pub function: Box<AST>,
    pub argument: Box<AST>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDefinition {
    pub argument_name: String,
    pub argument_type: Type,
    pub return_type: Type,
    pub body: Box<AST>,
}

#[derive(Debug, PartialEq)]
pub struct RecursiveFunction {
    pub function_name: String,
    pub argument_name: String,
    pub argument_type: Type,
    pub return_type: Type,
    pub body: Box<AST>,
    pub function_use: Box<AST>,
}

impl AST {
    pub fn build(token_stream: &mut TokenStream) -> AST {
        match token_stream.next() {
            Some(token) => {
                match token {
                    Token::TrueLiteral => AST::TrueLiteral,
                    Token::FalseLiteral => AST::FalseLiteral,
                    Token::NumLiteral => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        if let Token::NumberLiteral(number) = token_stream.next().unwrap() {
                            assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                            AST::NumberLiteral(number)
                        } else {
                            panic!("Number not found in NumC")
                        }
                    }
                    Token::Plus => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::Plus(Box::new(ast1), Box::new(ast2))
                    }
                    Token::Multiply => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::Multiply(Box::new(ast1), Box::new(ast2))
                    }
                    Token::If => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast3 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::If(If {
                            condition: Box::new(ast1),
                            then: Box::new(ast2),
                            els: Box::new(ast3),
                        })
                    }
                    Token::Identifier => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let string_ast;
                        match token_stream.next().unwrap() {
                            Token::QuotedString(id) => {
                                string_ast = AST::Identifier(id);
                            }
                            _ => panic!("String not found!"),
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        string_ast
                    }
                    Token::FunctionApplication => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::FunctionApplication(FunctionApplication {
                            function: Box::new(ast1),
                            argument: Box::new(ast2),
                        })
                    }
                    Token::FunctionDefinition => {
                        //THE ARGUMENT NAME
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let string_ast;
                        match token_stream.next().unwrap() {
                            Token::QuotedString(val) => {
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

                        AST::FunctionDefinition(FunctionDefinition {
                            argument_name: string_ast,
                            argument_type: arg_type,
                            return_type: ret_type,
                            body: Box::new(ast_body),
                        })
                    }
                    Token::Equals => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        let ast1 = AST::build(token_stream);
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        let ast2 = AST::build(token_stream);
                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                        AST::Equals(Box::new(ast1), Box::new(ast2))
                    }
                    Token::RecursiveFunction => {
                        assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                        // 1st parameter
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let rec_func_name;
                        match token_stream.next().unwrap() {
                            Token::QuotedString(val) => {
                                //Token::ID, not to be confused with idC
                                rec_func_name = val;
                            }
                            _ => panic!("String not found!"),
                        }
                        if rec_func_name == "main" {
                            panic!("'main' is a reserved function name");
                        }
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        // 2nd parameter
                        assert_eq!(Token::Quote, token_stream.next().unwrap());
                        let rec_arg_name;
                        match token_stream.next().unwrap() {
                            Token::QuotedString(val) => {
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
                        AST::RecursiveFunction(RecursiveFunction {
                            function_name: rec_func_name,
                            argument_name: rec_arg_name,
                            argument_type: rec_arg_type,
                            return_type: rec_ret_type,
                            body: Box::new(rec_body_ast),
                            function_use: Box::new(rec_func_use_ast),
                        })
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
                Token::NumberType => Type::Number,
                Token::BooleanType => Type::Boolean,
                Token::FunctionType => {
                    assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
                    let box1 = AST::parse_type(token_stream);
                    assert_eq!(Token::Comma, token_stream.next().unwrap());
                    let box2 = AST::parse_type(token_stream);
                    assert_eq!(Token::ParenRight, token_stream.next().unwrap());
                    Type::Function {
                        argument: Box::new(box1),
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
    fn num_c_1() {
        let tokens = VecDeque::from(vec![
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(AST::build(&mut token_stream), AST::NumberLiteral(1));
    }

    #[test]
    fn plus_c_1_2() {
        let tokens = VecDeque::from(vec![
            Token::Plus,
            Token::ParenLeft,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::Plus(
                Box::new(AST::NumberLiteral(1)),
                Box::new(AST::NumberLiteral(2))
            )
        );
    }

    #[test]
    #[should_panic]
    fn plus_c_1_num_c_2() {
        let tokens = VecDeque::from(vec![
            Token::Plus,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::Comma,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    fn mult_c_1_2() {
        let tokens = VecDeque::from(vec![
            Token::Multiply,
            Token::ParenLeft,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::Multiply(
                Box::new(AST::NumberLiteral(1)),
                Box::new(AST::NumberLiteral(2))
            )
        );
    }

    #[test]
    #[should_panic]
    fn plus_c() {
        //testing plusC(numC(1), numC(2) -> this should panic (missing right parenthesis)
        let tokens = VecDeque::from(vec![
            Token::Plus,
            Token::ParenLeft,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            // Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    fn if_c_1() {
        //testing if(true, true, false)
        let tokens = VecDeque::from(vec![
            Token::If,
            Token::ParenLeft,
            Token::TrueLiteral,
            Token::Comma,
            Token::TrueLiteral,
            Token::Comma,
            Token::FalseLiteral,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::If(If {
                condition: Box::new(AST::TrueLiteral),
                then: Box::new(AST::TrueLiteral),
                els: Box::new(AST::FalseLiteral)
            })
        );
    }
    #[test]
    #[should_panic]
    fn if_c_2() {
        //testing if(true, true false)
        let tokens = VecDeque::from(vec![
            Token::If,
            Token::ParenLeft,
            Token::TrueLiteral,
            Token::Comma,
            Token::TrueLiteral,
            Token::FalseLiteral,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
    #[test]
    fn id_c() {
        let tokens = VecDeque::from(vec![
            Token::Identifier,
            Token::ParenLeft,
            Token::Quote,
            Token::QuotedString("x".to_string()),
            Token::Quote,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        assert_eq!(
            AST::build(&mut token_stream),
            AST::Identifier("x".to_string())
        );
    }
    #[test]
    #[should_panic]
    fn id_c_missing_quote() {
        //testing id("x)
        let tokens = VecDeque::from(vec![
            Token::Identifier,
            Token::ParenLeft,
            Token::Quote,
            Token::QuotedString("x".to_string()),
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
    //Tests fir appC and fdC omitted here, done externally.

    #[test]
    fn eq_c() {
        let tokens = VecDeque::from(vec![
            Token::Equals,
            Token::ParenLeft,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
            Token::Comma,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }

    #[test]
    #[should_panic]
    fn eq_c_fail() {
        let tokens = VecDeque::from(vec![
            Token::Equals,
            Token::ParenLeft,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(1),
            Token::ParenRight,
            Token::NumLiteral,
            Token::ParenLeft,
            Token::NumberLiteral(2),
            Token::ParenRight,
            Token::ParenRight,
        ]);
        let mut token_stream = TokenStream::build_test(tokens, 0);
        AST::build(&mut token_stream);
    }
}
