use std::fs::File;
use std::io::Read;

pub fn build(input_file: &str) -> String {
    let mut code = String::new();
    File::open(input_file)
        .unwrap()
        .read_to_string(&mut code)
        .unwrap();
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input_file: &str, expected: &str) {
        let code_string = build(input_file);
        assert!(code_string.chars().eq(expected.chars()))
    }

    #[test]
    fn read_input_1() {
        test("tests/inputs/number_literal.txt", "numC(2)");
    }
    #[test]
    fn read_input_2() {
        test("tests/inputs/plus.txt", "plusC(numC(1), numC(2))");
    }
}
