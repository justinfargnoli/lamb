pub mod codegen;
pub mod parse;
pub mod read;
pub mod tokenize;
pub mod type_check;

use parse::AST;
use std::{collections::VecDeque, io::Result};
use tokenize::TokenStream;
use type_check::Type;

pub fn read(input_file: &str) -> Result<VecDeque<char>> {
    read::build(input_file)
}

pub fn tokenize(input_file: &str) -> TokenStream {
    let characters = read(input_file).unwrap();
    TokenStream::build(characters)
}

pub fn parse(input_file: &str) -> AST {
    let mut tokenizer = tokenize(input_file);
    AST::build(&mut tokenizer)
}

pub fn type_check(input_file: &str) -> Type {
    let ast = parse(input_file);
    type_check::tc(&ast)
}

pub fn check(input_file: &str) -> Type {
    let characters = read::build(input_file).unwrap();
    let mut tokenizer = TokenStream::build(characters);
    let ast = AST::build(&mut tokenizer);
    type_check::tc(&ast)
}

pub fn compile(input_file: &str) {
    let characters = read::build(input_file).unwrap();
    let mut tokenizer = TokenStream::build(characters);
    let ast = AST::build(&mut tokenizer);
    type_check::tc(&ast);
}
