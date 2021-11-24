use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Typed Lambda Calculus")
        .author("Justin Fargnoli <justinfargnoli@gmail.com>")
        .version("0.1")
        .about("A CLI for working with my implementation of the Typed Lambda Calculus.")
        .subcommand(
            SubCommand::with_name("check")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .help("Lex, parse, and type check 'file'."),
        )
        .subcommand(
            SubCommand::with_name("compile")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .help("Lex, parse, type check, and compile (with LLVM) 'file'."),
        )
        .subcommand(
            SubCommand::with_name("interpret")
                .arg(
                    Arg::with_name("file")
                        .required(true)
                        .takes_value(true)
                        .index(1),
                )
                .help("Lex, parse, type check, and interpret 'file'."),
        )
        .get_matches();

    match matches.subcommand() {
        ("check", Some(arg_matches)) => {
            let file = arg_matches
                .value_of("file")
                .expect("Argument <file> not found.");
            println!("'{}' returns the type '{}'.", file, lamb::check(file));
        }
        ("compile", Some(arg_matches)) => {
            let file = arg_matches
                .value_of("file")
                .expect("Argument <file> not found.");
            match lamb::compile(file) {
                Ok(result) => println!("{}", result),
                Err(error_string) => println!("{}", error_string),
            }
        }
        ("interpret", Some(_)) => unimplemented!(),
        _ => panic!("Unable to parse command line arguments."),
    };
}
