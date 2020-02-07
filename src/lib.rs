mod lex;
mod parse;
mod type_check;

mod read {
    use std::fs::File;
    use std::io::{Bytes, Read, Result};
    use std::iter::Peekable;

    pub fn build_reader(input_file: &str) -> Result<Peekable<Bytes<File>>> {
        File::open(input_file)?.bytes().peekable()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
