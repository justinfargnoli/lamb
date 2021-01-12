use std::env;

fn main() {
    let input_file = env::args().nth(1).expect("Usage: tlc <input file>");

    println!("The type is {:?}", tlc::check(&input_file));
}
