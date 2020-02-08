use crate::tokenizer::TokenStream;
use crate::Type;
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
    pub fn build(token_stream: TokenStream<impl Read>) -> AST {
        // match token_stream {
        // 	// Token::ParenLeft =>
        // 	_ =>
        // }
        unimplemented!()
    }
}
