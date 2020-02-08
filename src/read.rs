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
        let mut characters = build("input1.txt").expect("Unable to open file");
        assert_eq!(characters.next().unwrap().unwrap(), 'n' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'u' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'm' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'C' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), '(' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), '2' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), ')' as u8);
    }
}
