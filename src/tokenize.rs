#[derive(Debug, PartialEq)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Comma,
    Quote,
    ID(String),
    Number(u32),
    TTrue,
    TFalse,
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
    character_stream: Vec<char>,
    current_index: usize,
}

impl TokenStream {
    pub fn build(character_stream: Vec<char>) -> TokenStream {
        TokenStream {
            character_stream,
            current_index: 0,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let token = match self.character_stream[self.current_index] {
            '(' => Some(Token::ParenLeft),
            ')' => Some(Token::ParenRight),
            ',' => Some(Token::Comma),
            '\"' => Some(Token::Quote),
            _ => panic!("Your input wasn't able to be converted into a token stream."),
        };
        self.current_index += 1;
        token
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
