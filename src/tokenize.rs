use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
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
    TEqC,
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
    stream: VecDeque<Token>,
    curr_index: usize,
}

impl TokenStream {
    pub fn build(character_stream: VecDeque<char>) -> TokenStream {
        TokenStream {
            stream: TokenStream::tokenize(character_stream),
            curr_index: 0,
        }
    }

    #[allow(dead_code)] // used for tests in 'parse.rs'
    pub fn build_test(token_stream: VecDeque<Token>, curr_index: usize) -> TokenStream {
        TokenStream {
            stream: token_stream,
            curr_index: curr_index,
        }
    }

    fn tokenize(mut char_stream: VecDeque<char>) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        while !char_stream.is_empty() {
            match char_stream.pop_front().unwrap() {
                '(' => {
                    tokens.push_back(Token::ParenLeft);
                    let mut num = String::new();
                    loop {
                        if char_stream.front().unwrap().to_digit(10).is_some() {
                            num.push(char_stream.pop_front().unwrap());
                        } else if !num.is_empty() {
                            tokens.push_back(Token::Number(num.parse::<u32>().unwrap()));
                            break;
                        } else {
                            break;
                        }
                    }
                }
                ')' => tokens.push_back(Token::ParenRight),
                ',' => tokens.push_back(Token::Comma),
                '\"' => {
                    tokens.push_back(Token::Quote);
                    let mut id = String::new();
                    loop {
                        let next_char = char_stream.pop_front().unwrap();
                        if next_char == '\"' {
                            tokens.push_back(Token::ID(id));
                            tokens.push_back(Token::Quote);
                            break;
                        }
                        id.push(next_char);
                    }
                }
                'n' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 'm');
                    let next_char = char_stream.pop_front().unwrap();
                    if next_char == 'T' {
                        tokens.push_back(Token::NumT);
                    } else if next_char == 'C' {
                        tokens.push_back(Token::TNumC);
                    }
                }
                'b' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'o');
                    assert_eq!(char_stream.pop_front().unwrap(), 'o');
                    assert_eq!(char_stream.pop_front().unwrap(), 'l');
                    assert_eq!(char_stream.pop_front().unwrap(), 'T');
                    tokens.push_back(Token::BoolT);
                }
                'f' => {
                    let next_char = char_stream.pop_front().unwrap();
                    if next_char == 'u' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'n');
                        assert_eq!(char_stream.pop_front().unwrap(), 'T');
                        tokens.push_back(Token::FunT);
                    } else if next_char == 'a' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'l');
                        assert_eq!(char_stream.pop_front().unwrap(), 's');
                        assert_eq!(char_stream.pop_front().unwrap(), 'e');
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::TFalseC);
                    } else if next_char == 'd' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::TFdC);
                    } else {
                        panic!()
                    }
                }
                'p' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'l');
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 's');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TPlusC);
                }
                'm' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 'l');
                    assert_eq!(char_stream.pop_front().unwrap(), 't');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TMultC);
                }
                'a' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'p');
                    assert_eq!(char_stream.pop_front().unwrap(), 'p');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TAppC);
                }
                'i' => {
                    let next_char = char_stream.pop_front().unwrap();
                    if next_char == 'f' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::TIfC);
                    } else if next_char == 'd' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::TIdC);
                    } else {
                        panic!()
                    }
                }
                't' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'r');
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 'e');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TTrueC);
                }
                'e' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'q');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TEqC);
                }
                ' ' => continue,
                _ => panic!("Your input wasn't able to be converted into a token stream."),
            }
        }
        tokens
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.stream.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_num_c() {
        let characters = VecDeque::from(vec!['n', 'u', 'm', 'C', '(', '2', ')']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_id_c() {
        let characters = VecDeque::from(vec!['i', 'd', 'C', '(', '\"', 'a', 'b', '\"', ')']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TIdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("ab"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_mult_c() {
        let characters = VecDeque::from(vec![
            'm', 'u', 'l', 't', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ]);
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
    fn tokenize_plus_c() {
        let characters = VecDeque::from(vec![
            'p', 'l', 'u', 's', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ]);
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
    fn tokenize_true_c() {
        let characters = VecDeque::from(vec!['t', 'r', 'u', 'e', 'C']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TTrueC));
    }

    #[test]
    fn tokenize_false_c() {
        let characters = VecDeque::from(vec!['f', 'a', 'l', 's', 'e', 'C']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TFalseC));
    }

    #[test]
    fn tokenize_eq_c() {
        let characters = String::from("eqC(numC(1), numC(3))").chars().collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TEqC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(1)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(3)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_if_c() {
        let characters = String::from("ifC(falseC, numC(1), numC(3))")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TIfC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::TFalseC));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(1)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TNumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(3)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_fd_c() {
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
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn tokenize_app_c() {
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
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::TFalseC));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }
}
