use std::io::{Bytes, Read};
use std::iter::Peekable;

#[derive(Debug)]
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
        TokenStream { character_stream }
    }
}

impl<T: Read> Iterator for TokenStream<T> {
    type Item = TokenStream<T>;

    fn next(&mut self) -> Option<TokenStream<T>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read;

    #[test]
    fn tokenize_1() {
        let mut characters = read::build("input1.txt").expect("Unable to open file");
        let token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }
}
