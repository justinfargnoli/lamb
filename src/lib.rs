mod lex;
mod parse;
mod read;
mod type_check;

pub fn type_check(input_file: &str) {
    let characters = read::build_reader(input_file);
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn type_check_1() {
        // define types first then write the tests for type_check
    }
}
