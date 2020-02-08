use std::fs::File;
use std::io::{Read, Result};

pub fn build(input_file: &str) -> Result<Vec<char>> {
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
    fn read_1() {
        let mut character_iter = build("input1.txt").expect("Unable to open file").into_iter();
        assert_eq!(character_iter.next().unwrap(), 'n');
        assert_eq!(character_iter.next().unwrap(), 'u');
        assert_eq!(character_iter.next().unwrap(), 'm');
        assert_eq!(character_iter.next().unwrap(), 'C');
        assert_eq!(character_iter.next().unwrap(), '(');
        assert_eq!(character_iter.next().unwrap(), '2');
        assert_eq!(character_iter.next().unwrap(), ')');
    }
}
