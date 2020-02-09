use crate::tokenize::{Token, TokenStream};
use crate::Type;

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
	pub fn build(token_stream:&mut TokenStream) -> Box<AST> {
		match token_stream.next() {
			Some(token) => {
				match token {
					Token::TTrue => Box::new(AST::AtrueC),
					Token::TFalse => Box::new(AST::AfalseC),
					Token::TNumC => {
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						let mut ast = AST::Anumc;
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						Box::new(ast)
					}
					Token::TPlusC => {
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						let mut ast1 = AST::build(token_stream);
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						let mut ast2 = AST::build(token_stream);
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						Box::new(AST::AplusC(ast1, ast2))
					}
					Token::TMultC => {
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						let mut ast1 = AST::build(token_stream);
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						let mut ast2 = AST::build(token_stream);
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						Box::new(AST::AmultC(ast1, ast2))
					}
					Token::TIfC => {
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						let mut ast1 = AST::build(token_stream);
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						let mut ast2 = AST::build(token_stream);
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						let mut ast3 = AST::build(token_stream);
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						Box::new(AST::AifC {cnd: ast1, then: ast2, els: ast3}) 
					}
					Token::TIdC => {	//TODO: CHECK!!!!
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						assert_eq!(Token::Quote, token_stream.next().unwrap());
						let mut string_ast;
						match token_stream.next().unwrap() {
							Token::ID(val) => {string_ast = Box::new(AST::AidC(val));},
							_ => panic!("String not found!"),
						}
						assert_eq!(Token::Quote, token_stream.next().unwrap());
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						string_ast
					}
					Token::TAppC => {
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						let mut ast1 = AST::build(token_stream);
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						let mut ast2 = AST::build(token_stream);
						assert_eq!(Token::ParenRight, token_stream.next().unwrap());
						Box::new(AST::AappC {func: ast1, arg: ast2})
					}
					Token::TFdC => {	//TODO: CHECK!!!!
						assert_eq!(Token::ParenLeft, token_stream.next().unwrap());
						assert_eq!(Token::Quote, token_stream.next().unwrap());
						let mut string_ast;
						match token_stream.next().unwrap() {
							Token::ID(val) => {string_ast = Box::new(AST::AidC(val));},
							_ => panic!("String not found!"),
						}
						assert_eq!(Token::Comma, token_stream.next().unwrap());
						//TODO: DO THE ARGUMENT TYPE

						assert_eq!(Token::Comma, token_stream.next().unwrap());
						//TODO: DO THE RETURN TYPE
						
						let mut ast_body = AST::build(token_stream);

						assert_eq!(Token::ParenRight, token_stream.next().unwrap());


                        assert_eq!(Token::Comma, token_stream.next().unwrap());
                        //TODO: DO THE RETURN TYPE

                        let mut ast_body = AST::build(token_stream);

                        assert_eq!(Token::ParenRight, token_stream.next().unwrap());

                        Box::new(AST::AfalseC) //TODO: NOT ACTUALLY THIS
                    }
					_ => Box::new(AST::AfalseC)	////TODO: THIS should never happen
				}
			}
			None => panic!("No token found")
		}
		// unimplemented!()
	}
}