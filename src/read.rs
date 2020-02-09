use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Result};

pub fn build(input_file: &str) -> Result<VecDeque<char>> {
    Result::Ok(
        File::open(input_file)?
            .bytes()
            .map(|byte| byte.unwrap() as char)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_1() {
        let mut character_iter = build("input1.txt")
            .expect("Unable to open file")
            .into_iter();
        assert_eq!(character_iter.next().unwrap(), 'n');
        assert_eq!(character_iter.next().unwrap(), 'u');
        assert_eq!(character_iter.next().unwrap(), 'm');
        assert_eq!(character_iter.next().unwrap(), 'C');
        assert_eq!(character_iter.next().unwrap(), '(');
        assert_eq!(character_iter.next().unwrap(), '2');
        assert_eq!(character_iter.next().unwrap(), ')');
    }
    #[test]
    fn read_2() {
        let mut character_iter = build("input2.txt")
            .expect("Unable to open file")
            .into_iter();
        assert_eq!(character_iter.next().unwrap(), 'p');
        assert_eq!(character_iter.next().unwrap(), 'l');
        assert_eq!(character_iter.next().unwrap(), 'u');
        assert_eq!(character_iter.next().unwrap(), 's');
        assert_eq!(character_iter.next().unwrap(), 'C');
        assert_eq!(character_iter.next().unwrap(), '(');
        assert_eq!(character_iter.next().unwrap(), 'n');
        assert_eq!(character_iter.next().unwrap(), 'u');
        assert_eq!(character_iter.next().unwrap(), 'm');
        assert_eq!(character_iter.next().unwrap(), 'C');
        assert_eq!(character_iter.next().unwrap(), '(');
        assert_eq!(character_iter.next().unwrap(), '1');
        assert_eq!(character_iter.next().unwrap(), ')');
        assert_eq!(character_iter.next().unwrap(), ',');
        assert_eq!(character_iter.next().unwrap(), ' ');
        assert_eq!(character_iter.next().unwrap(), 'n');
        assert_eq!(character_iter.next().unwrap(), 'u');
        assert_eq!(character_iter.next().unwrap(), 'm');
        assert_eq!(character_iter.next().unwrap(), 'C');
        assert_eq!(character_iter.next().unwrap(), '(');
        assert_eq!(character_iter.next().unwrap(), '2');
        assert_eq!(character_iter.next().unwrap(), ')');
        assert_eq!(character_iter.next().unwrap(), ')');
    }
}
