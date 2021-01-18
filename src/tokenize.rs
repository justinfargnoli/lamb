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
    Number(i64),
    EqC,
    TrueC,
    FalseC,
    NumC,
    PlusC,
    MultC,
    IfC,
    IdC,
    AppC,
    FdC,
    RecC,
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
            curr_index,
        }
    }

    fn tokenize(mut char_stream: VecDeque<char>) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        while !char_stream.is_empty() {
            match char_stream.pop_front().unwrap() {
                '(' => {
                    tokens.push_back(Token::ParenLeft);
                    let mut num_str = String::new();
                    let mut negative = false;
                    if *char_stream.front().unwrap() == '-' {
                        negative = true;
                        char_stream.pop_front().unwrap();
                    }
                    loop {
                        if char_stream.front().unwrap().is_digit(10) {
                            num_str.push(char_stream.pop_front().unwrap());
                        } else if !num_str.is_empty() {
                            let mut num = num_str.parse::<i64>().unwrap();
                            if negative {
                                num *= -1;
                            }
                            tokens.push_back(Token::Number(num));
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
                        tokens.push_back(Token::NumC);
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
                        tokens.push_back(Token::FalseC);
                    } else if next_char == 'd' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::FdC);
                    } else {
                        panic!()
                    }
                }
                'p' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'l');
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 's');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::PlusC);
                }
                'm' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 'l');
                    assert_eq!(char_stream.pop_front().unwrap(), 't');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::MultC);
                }
                'a' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'p');
                    assert_eq!(char_stream.pop_front().unwrap(), 'p');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::AppC);
                }
                'i' => {
                    let next_char = char_stream.pop_front().unwrap();
                    if next_char == 'f' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::IfC);
                    } else if next_char == 'd' {
                        assert_eq!(char_stream.pop_front().unwrap(), 'C');
                        tokens.push_back(Token::IdC);
                    } else {
                        panic!()
                    }
                }
                't' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'r');
                    assert_eq!(char_stream.pop_front().unwrap(), 'u');
                    assert_eq!(char_stream.pop_front().unwrap(), 'e');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::TrueC);
                }
                'e' => {
                    assert_eq!(char_stream.pop_front().unwrap(), 'q');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::EqC);
                }
                'r' => {
                    // todo: write tests for this
                    assert_eq!(char_stream.pop_front().unwrap(), 'e');
                    assert_eq!(char_stream.pop_front().unwrap(), 'c');
                    assert_eq!(char_stream.pop_front().unwrap(), 'C');
                    tokens.push_back(Token::RecC);
                }
                ' ' | '\t' | '\n' | '\r' => continue,
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
    fn num_c() {
        let characters = VecDeque::from(vec!['n', 'u', 'm', 'C', '(', '2', ')']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn num_c_negative() {
        let characters = VecDeque::from(vec!['n', 'u', 'm', 'C', '(', '-', '2', ')']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(-2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn id_c() {
        let characters = VecDeque::from(vec!['i', 'd', 'C', '(', '\"', 'a', 'b', '\"', ')']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::IdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("ab"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn mult_c() {
        let characters = VecDeque::from(vec![
            'm', 'u', 'l', 't', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ]);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::MultC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn plus_c() {
        let characters = VecDeque::from(vec![
            'p', 'l', 'u', 's', 'C', '(', 'n', 'u', 'm', 'C', '(', '2', ')', ',', 'n', 'u', 'm',
            'C', '(', '2', ')', ')',
        ]);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::PlusC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(2)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn true_c() {
        let characters = VecDeque::from(vec!['t', 'r', 'u', 'e', 'C']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::TrueC));
    }

    #[test]
    fn false_c() {
        let characters = VecDeque::from(vec!['f', 'a', 'l', 's', 'e', 'C']);
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::FalseC));
    }

    #[test]
    fn eq_c() {
        let characters = String::from("eqC(numC(1), numC(3))").chars().collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::EqC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(1)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(3)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn if_c() {
        let characters = String::from("ifC(falseC, numC(1), numC(3))")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::IfC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::FalseC));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(1)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::NumC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Number(3)));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn fd_c() {
        let characters = String::from("fdC(\"x\", boolT, boolT, idC(\"x\"))")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::FdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::IdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }

    #[test]
    fn app_c() {
        let characters = String::from("appC(fdC(\"x\", boolT, boolT, idC(\"x\")), falseC)")
            .chars()
            .collect();
        let mut token_stream = TokenStream::build(characters);

        assert_eq!(token_stream.next(), Some(Token::AppC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::FdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::BoolT));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::IdC));
        assert_eq!(token_stream.next(), Some(Token::ParenLeft));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ID(String::from("x"))));
        assert_eq!(token_stream.next(), Some(Token::Quote));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
        assert_eq!(token_stream.next(), Some(Token::Comma));
        assert_eq!(token_stream.next(), Some(Token::FalseC));
        assert_eq!(token_stream.next(), Some(Token::ParenRight));
    }
}
