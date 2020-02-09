use std::env;
use tc200;

fn main() {
    let input_file = env::args().nth(1).expect("Usage: & tc <input file>");

    println!("The type is {:?}", tc200::type_check(&input_file));
}
