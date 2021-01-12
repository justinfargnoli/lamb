use crate::parse::AST;
use inkwell::{builder::Builder, context::Context, module::Module};

pub struct CodeGen<'ctx> {
    _context: &'ctx Context,
    _module: Module<'ctx>,
    _builder: Builder<'ctx>,
}

pub fn run(ast: &AST) {
    let context = Context::create();
    let mut codegen = CodeGen {
        _context: &context,
        _module: context.create_module("main"),
        _builder: context.create_builder(),
    };

    module(&mut codegen, ast);
}

fn module(_codegen: &mut CodeGen, ast: &AST) {
    match ast {
        _ => unimplemented!(),
    }
}
