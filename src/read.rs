use std::fs::File;
use std::io::{Bytes, Read, Result};
use std::iter::Peekable;

pub fn build_reader(input_file: &str) -> Result<Peekable<Bytes<File>>> {
    Result::Ok(File::open(input_file)?.bytes().peekable())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        let mut characters = build_reader("input1.txt").expect("Unable to open file");
        assert_eq!(characters.next().unwrap().unwrap(), 'n' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'u' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'm' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), 'C' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), '(' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), '2' as u8);
        assert_eq!(characters.next().unwrap().unwrap(), ')' as u8);
    }
}
