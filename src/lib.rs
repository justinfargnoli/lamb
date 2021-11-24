pub mod codegen;
pub mod format;
pub mod interpret;
pub mod parse;
pub mod read;
pub mod tokenize;
pub mod type_check;

use inkwell::support::LLVMString;
use interpret::Data;
use parse::AST;
use tokenize::TokenStream;
use type_check::{Type, TypedAST};

pub fn tokenize(code: &str) -> TokenStream {
    TokenStream::build(code.chars().collect())
}

pub fn parse(code: &str) -> AST {
    let mut tokenizer = tokenize(code);
    AST::build(&mut tokenizer)
}

pub fn type_check(code: &str) -> Type {
    let ast = parse(code);
    type_check::type_of(&ast)
}

pub fn check(code: &str) -> Type {
    type_check(code)
}

fn typed_ast(code: &str) -> TypedAST {
    let ast = parse(code);
    TypedAST::new(&ast)
}

pub fn compile(code: &str) -> Result<u64, LLVMString> {
    let typed_ast = typed_ast(code);
    codegen::run(&typed_ast)
}

pub fn interpret(code: &str) -> Data {
    let ast = parse(code);
    interpret::interpret(&ast)
}

pub fn format(code: &str) {
    let ast = parse(code);
    format::format(&ast);
}
