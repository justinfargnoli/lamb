#[derive(Debug, PartialEq)]
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
pub struct TokenStream {
    character_stream: std::vec::IntoIter<char>,
}

impl TokenStream {
    pub fn build(character_stream: Vec<char>) -> TokenStream {
        TokenStream {
            character_stream: character_stream.into_iter(),
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.character_stream.next() {
            Some(character) => match character {
                '(' => Some(Token::ParenLeft),
                ')' => Some(Token::ParenRight),
                ',' => Some(Token::Comma),
                _ => Some(Token::TFdC),
            },
            None => None,
        }
        // unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read;

    #[test]
    fn tokenize_1() {
        let characters = read::build("input1.txt").expect("Unable to open file");
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }
}
