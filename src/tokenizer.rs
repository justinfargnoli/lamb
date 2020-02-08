use std::io::{Bytes, Read};
use std::iter::Peekable;

pub enum Token {
    ParenLeft,
    ParenRight,
    Comma,
    ID(String),
    Number(u32),
    TTrueC,
    TFalseC,
    TNumC,
    TPlusC,
    TMultC,
    TIfC,
    TIdC,
    TAppC,
    TFdC,
}

#[derive(Debug)]
pub struct TokenStream<T: Read> {
	character_stream: Peekable<Bytes<T>>, 
}

impl<T: Read> TokenStream<T> {
	pub fn build(character_stream: Peekable<Bytes<T>>) -> TokenStream<T> {
		TokenStream {
			character_stream,
		}
	}
}

impl<T: Read> Iterator for TokenStream<T> {
    type Item = TokenStream<T>;

    fn next(&mut self) -> Option<TokenStream<T>> {
        unimplemented!()
    }
}

