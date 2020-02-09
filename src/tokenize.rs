#[derive(Debug, PartialEq)]
pub enum Token {
    ParenLeft,
    ParenRight,
    Comma,
    Quote,
    NumT,
    BoolT,
    FunT,
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

#[derive(Debug, PartialEq)]
pub struct TokenStream {
    stream: Vec<Token>,
    curr_index: usize,
}

impl TokenStream {
    pub fn build(character_stream: Vec<char>) -> TokenStream {
        TokenStream {
            stream: TokenStream::tokenize(character_stream),
            curr_index: 0,
        }
    }

    pub fn build_test(token_stream: Vec<Token>, curr_index: usize) -> TokenStream {
        TokenStream {
            stream: token_stream,
            curr_index: curr_index,
        }
    }

    pub fn stream(&self) -> &Vec<Token> {
        &self.stream
    }

    pub fn stream_mut(&mut self) -> &mut Vec<Token> {
        &mut self.stream
    }

    fn tokenize(char_stream: Vec<char>) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut curr_index: usize = 0;
        match TokenStream::next_char(&char_stream, &mut curr_index) {
            '(' => tokens.push(Token::ParenLeft),
            ')' => tokens.push(Token::ParenRight),
            ',' => tokens.push(Token::Comma),
            '\"' => tokens.push(Token::Quote),
            'n' => {
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'u');
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'm');
                let next_char = TokenStream::next_char(&char_stream, &mut curr_index);
                if next_char == 'T' {
                    tokens.push(Token::NumT);
                } else if next_char == 'C' {
                    tokens.push(Token::TNumC);
                }
                panic!();
            }
            'b' => {
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'o');
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'o');
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'l');
                assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'T');
                tokens.push(Token::BoolT);
            }
            'f' => {
                let next_char = TokenStream::next_char(&char_stream, &mut curr_index);
                if next_char == 'u' {
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'n');
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'T');
                    tokens.push(Token::FunT);
                } else if next_char == 'a' {
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'l');
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 's');
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'e');
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'C');
                    tokens.push(Token::TFalseC);
                } else if next_char == 'd' {
                    assert_eq!(TokenStream::next_char(&char_stream, &mut curr_index), 'C');
                    tokens.push(Token::TFdC);
                }
                panic!()
            }
            _ => panic!("Your input wasn't able to be converted into a token stream."),
        }
        tokens
    }

    fn next_char(stream: &[char], curr_index: &mut usize) -> char {
        let character = stream[*curr_index];
        *curr_index += 1;
        character
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_numC() {
        let characters = vec!['n', 'u', 'm', 'C', '(', '2', ')'];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_idC() {
        let characters = vec!['i', 'd', 'C', '\"', 'a', 'b', '\"'];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TIdC));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("ab"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
    }

    #[test]
    fn tokenize_multC() {
        let characters = vec![
            'm', 'u', 'l', 't', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TMultC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_plusC() {
        let characters = vec![
            'p', 'l', 'u', 's', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TPlusC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_trueC() {
        let characters = vec!['t', 'r', 'u', 'e', 'C'];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TTrueC));
    }

    #[test]
    fn tokenize_falseC() {
        let characters = vec!['f', 'a', 'l', 's', 'e', 'C'];
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TFalseC));
    }

    #[test]
    fn tokenize_ifC() {
        let characters = String::from("ifC(falseC, numC(1), numC(3))")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TFalseC));
    }

    #[test]
    fn tokenize_fdC() {
        let characters = String::from("fdC(\"x\", boolT, boolT, idC(\"x\"))")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TFdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TIdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_appC() {
        let characters = String::from("appC(fdC(\"x\", boolT, boolT, idC(\"x\")), falseC)")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TAppC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::TFdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TIdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TFalseC));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }
}
