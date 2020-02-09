use crate::parse::AST;
use crate::Type;
use std::collections::HashMap;

pub fn tc(ast: Box<AST>, tenv: HashMap<String, Type>) -> Type {
	match *ast {
		AST::Anumc => Type::NumT,
		_ => Type::NumT,
	}

    // unimplemented!()
}
