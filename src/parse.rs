use crate::Type;
use crate::tokenizer::TokenStream;
use crate::tokenizer::Token;
use std::io::Read;

pub enum AST {
	Anumc,
	AplusC(Box<AST>, Box<AST>),
	AmultC(Box<AST>, Box<AST>),
	AtrueC,
	AfalseC,
	AifC {cnd: Box<AST>, then: Box<AST>, els: Box<AST>},
	AidC(String),
	AappC {func: Box<AST>, arg: Box<AST>},
	AfdC {arg_name: String, arg_type: Type, ret_type: Type, body: Box<AST>},
}

impl AST {
	pub fn build(token_stream:&mut TokenStream<impl Read>) -> Box<AST> {
		match token_stream.next() {
			Some(token) => {
				match token {
					Token::TNumC => Box::new(AST::Anumc),
					Token::TTrueC => Box::new(AST::AtrueC),
					Token::TFalseC => Box::new(AST::AfalseC),
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
					Token::TIdC => {	//TODO: WHAT SHOULD THIS BE
						assert_eq!(Token::Quotes, token_stream.next().unwrap());
						let mut string_ast;
						match token_stream.next().unwrap() {
							Token::ID(val) => {string_ast = Box::new(AST::AidC(val));},
							_ => panic!("String not found!"),
						}
						assert_eq!(Token::Quotes, token_stream.next().unwrap());
						string_ast
					}

					_ => Box::new(AST::AfalseC),
				}
			}
			None => panic!("IAUFHI")
		}
		// unimplemented!()
	}
}