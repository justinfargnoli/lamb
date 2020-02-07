use std::env;
use type_checker;

fn main() {
    let input_file = env::args().nth(1).expect("Usage: & tc <input file>");

    type_checker::type_check(&input_file);
}
