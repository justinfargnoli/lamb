use crate::tokenize::TokenStream;
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
        arg_type: Type,
        ret_type: Type,
        body: Box<AST>,
    },
}

impl AST {
	pub fn build(mut token_stream: TokenStream<impl Read>) -> Box<AST> {
		match token_stream.next() {
			Some(token) => {
				match token {
					Token::TNumC => Box::new(AST::Anumc),
					_ => Box::new(AST::Anumc),
				}
			}
			None => panic!("IAUFHI")
		}
		// unimplemented!()
	}
}
