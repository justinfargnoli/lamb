// use crate::Input;
use std::fs::File;
use std::io::{Bytes, Read, Result};
use std::iter::Peekable;

// pub struct Reader {
//     characters: Peekable<Bytes<File>>,
// }

pub fn build_reader(input_file: &str) -> Result<Peekable<Bytes<File>>> {
    File::open(input_file)?.bytes().peekable()
}

// impl Reader {
//     pub fn build(input_file: &str) -> Result<Reader> {
//         Result::Ok(Reader {
//             characters: File::open(input_file)?.bytes().peekable(),
//         })
//     }
// }

// impl Input<u8> for Reader {
//     fn peek(&self) -> u8 {
//         self.characters.peek()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
}
