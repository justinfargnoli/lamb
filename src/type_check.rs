use crate::parse::AST;
use std::{collections::HashMap, fmt, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,
    Boolean,
    Function { argument: Box<Type>, ret: Box<Type> },
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "NumberType"),
            Type::Boolean => write!(f, "BooleanType"),
            Type::Function { argument: arg, ret } => write!(f, "FunctionType({}, {})", arg, ret),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TypedASTEnum {
    NumberLiteral(i64),
    Plus(TypedAST, TypedAST),
    Multiply(TypedAST, TypedAST),
    TrueLiteral,
    FalseLiteral,
    Equals(TypedAST, TypedAST),
    If(TypedIf),
    Identifier(String),
    FunctionApplication(TypedFunctionApplication),
    FunctionDefinition(TypedFunctionDefinition),
    RecursiveFunction(TypedRecursiveFunction),
}

#[derive(Debug, PartialEq)]
pub struct TypedIf {
    pub condition: TypedAST,
    pub then: TypedAST,
    pub els: TypedAST,
}

#[derive(Debug, PartialEq)]
pub struct TypedFunctionApplication {
    pub function: TypedAST,
    pub argument: TypedAST,
}

#[derive(Debug, PartialEq)]
pub struct TypedFunctionDefinition {
    pub argument_name: String,
    pub argument_type: Type,
    pub return_type: Type,
    pub body: TypedAST,
}

#[derive(Debug, PartialEq)]
pub struct TypedRecursiveFunction {
    pub function_name: String,
    pub argument_name: String,
    pub argument_type: Type,
    pub return_type: Type,
    pub body: TypedAST,
    pub function_use: TypedAST,
}

#[derive(Debug, PartialEq)]
pub struct TypedAST {
    pub ty: Type,
    pub ast: Box<TypedASTEnum>,
}

impl TypedAST {
    pub fn new(ast: &AST) -> TypedAST {
        TypedAST::typer(ast, &mut HashMap::new())
    }

    fn typer(ast: &AST, tenv: &mut HashMap<String, Type>) -> TypedAST {
        match ast {
            AST::TrueLiteral => TypedAST {
                ty: Type::Boolean,
                ast: Box::new(TypedASTEnum::TrueLiteral),
            },
            AST::FalseLiteral => TypedAST {
                ty: Type::Boolean,
                ast: Box::new(TypedASTEnum::TrueLiteral),
            },
            AST::NumberLiteral(number) => TypedAST {
                ty: Type::Number,
                ast: Box::new(TypedASTEnum::NumberLiteral(*number)),
            },
            AST::Plus(operand1, operand2) => {
                let typed_ast1 = TypedAST::typer(operand1, tenv);
                let typed_ast2 = TypedAST::typer(operand2, tenv);

                if typed_ast1.ty != Type::Number || typed_ast2.ty != Type::Number {
                    panic!("Types differ in PlusC!")
                }

                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::Plus(typed_ast1, typed_ast2)),
                }
            }
            AST::Multiply(operand1, operand2) => {
                let typed_ast1 = TypedAST::typer(operand1, tenv);
                let typed_ast2 = TypedAST::typer(operand2, tenv);

                if typed_ast1.ty != Type::Number || typed_ast2.ty != Type::Number {
                    panic!("Types differ in MultC!")
                }

                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::Multiply(typed_ast1, typed_ast2)),
                }
            }
            AST::Equals(operand1, operand2) => {
                let typed_ast1 = TypedAST::typer(operand1, tenv);
                let typed_ast2 = TypedAST::typer(operand2, tenv);

                if let Type::Function { .. } = typed_ast1.ty {
                    panic!("EqC cannot compare type FunT")
                } else if let Type::Function { .. } = typed_ast2.ty {
                    panic!("EqC cannot compare type FunT")
                } else if typed_ast1.ty != typed_ast2.ty {
                    panic!("Types differ in MultC!")
                }

                TypedAST {
                    ty: Type::Boolean,
                    ast: Box::new(TypedASTEnum::Equals(typed_ast1, typed_ast2)),
                }
            }
            AST::If(if_struct) => {
                let condition = TypedAST::typer(&if_struct.condition, tenv);
                if condition.ty != Type::Boolean {
                    panic!("Condition in an if statement is not boolean!")
                }

                let then = TypedAST::typer(&if_struct.then, tenv);
                let els = TypedAST::typer(&if_struct.els, tenv);
                if then.ty != els.ty {
                    panic!("Types differ in then and else part of an if statement!")
                }

                TypedAST {
                    ty: then.ty.clone(),
                    ast: Box::new(TypedASTEnum::If(TypedIf {
                        condition,
                        then,
                        els,
                    })),
                }
            }
            AST::Identifier(identifier) => {
                if !tenv.contains_key(identifier) {
                    panic!("Variable not saved in type environment")
                }

                TypedAST {
                    ty: tenv[identifier].clone(),
                    ast: Box::new(TypedASTEnum::Identifier(identifier.clone())),
                }
            }
            AST::FunctionApplication(function_application_struct) => {
                let function = TypedAST::typer(&function_application_struct.function, tenv);
                match &function.ty {
                    Type::Function {
                        argument: function_argument_type,
                        ret,
                    } => {
                        let argument = TypedAST::typer(&function_application_struct.argument, tenv);
                        if **function_argument_type != argument.ty {
                            panic!("Argument type doesn't match declared type")
                        }

                        TypedAST {
                            ty: (**ret).clone(),
                            ast: Box::new(TypedASTEnum::FunctionApplication(
                                TypedFunctionApplication { function, argument },
                            )),
                        }
                    }
                    _ => panic!("Not a function in appC"),
                }
            }
            AST::FunctionDefinition(function_definition_struct) => {
                tenv.insert(
                    function_definition_struct.argument_name.clone(),
                    function_definition_struct.argument_type.clone(),
                );

                let body = TypedAST::typer(&function_definition_struct.body, tenv);
                if body.ty != function_definition_struct.return_type {
                    panic!("Body type doesn't match declared type")
                }

                /*
                 * Since the body has type checked we can remove the variable name form the scope to
                 * preserve a common understanding of scope. This allows us ot avoid cloning the HashMap.
                 */
                tenv.remove(&function_definition_struct.argument_name);

                TypedAST {
                    ty: Type::Function {
                        argument: Box::new(function_definition_struct.argument_type.clone()),
                        ret: Box::new(function_definition_struct.return_type.clone()),
                    },
                    ast: Box::new(TypedASTEnum::FunctionDefinition(TypedFunctionDefinition {
                        argument_name: function_definition_struct.argument_name.clone(),
                        argument_type: function_definition_struct.argument_type.clone(),
                        return_type: function_definition_struct.return_type.clone(),
                        body,
                    })),
                }
            }
            AST::RecursiveFunction(recursive_function_struct) => {
                tenv.insert(
                    recursive_function_struct.function_name.clone(),
                    Type::Function {
                        argument: Box::new(recursive_function_struct.argument_type.clone()),
                        ret: Box::new(recursive_function_struct.return_type.clone()),
                    },
                );
                tenv.insert(
                    recursive_function_struct.argument_name.clone(),
                    recursive_function_struct.argument_type.clone(),
                );

                let body = TypedAST::typer(&recursive_function_struct.body, tenv);
                if recursive_function_struct.return_type != body.ty {
                    panic!(
                        "Return type of recursive function does not match return type of the body!"
                    );
                }

                let function_use = TypedAST::typer(&recursive_function_struct.function_use, tenv);

                tenv.remove(&recursive_function_struct.function_name);
                tenv.remove(&recursive_function_struct.argument_name);

                TypedAST {
                    ty: function_use.ty.clone(),
                    ast: Box::new(TypedASTEnum::RecursiveFunction(TypedRecursiveFunction {
                        function_name: recursive_function_struct.function_name.clone(),
                        argument_name: recursive_function_struct.argument_name.clone(),
                        argument_type: recursive_function_struct.argument_type.clone(),
                        return_type: recursive_function_struct.return_type.clone(),
                        body,
                        function_use,
                    })),
                }
            }
        }
    }
}

pub fn type_of(ast: &AST) -> Type {
    TypedAST::new(ast).ty
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{FunctionApplication, FunctionDefinition, RecursiveFunction};

    #[test]
    fn eq_c() {
        let input = Box::new(AST::Equals(
            Box::new(AST::NumberLiteral(0)),
            Box::new(AST::NumberLiteral(-5)),
        ));
        assert_eq!(type_of(&input), Type::Boolean);
    }

    #[test]
    #[should_panic]
    fn eq_c_fail_incompatible_type() {
        let input = Box::new(AST::Equals(
            Box::new(AST::TrueLiteral),
            Box::new(AST::NumberLiteral(-984)),
        ));
        type_of(&input);
    }

    #[test]
    #[should_panic]
    fn eq_c_fail_comparing_functions() {
        let input = Box::new(AST::Equals(
            Box::new(AST::FunctionDefinition(FunctionDefinition {
                argument_name: String::from("a"),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::Identifier(String::from("a"))),
            })),
            Box::new(AST::FunctionDefinition(FunctionDefinition {
                argument_name: String::from("a"),
                argument_type: Type::Number,
                return_type: Type::Number,
                body: Box::new(AST::Identifier(String::from("a"))),
            })),
        ));
        type_of(&input);
    }

    #[test]
    fn ec_c_ret_type() {
        let input = Box::new(AST::RecursiveFunction(RecursiveFunction {
            function_name: String::from("func"),
            argument_name: String::from("arg"),
            argument_type: Type::Number,
            return_type: Type::Number,
            body: Box::new(AST::Identifier(String::from("arg"))),
            function_use: Box::new(AST::Equals(
                Box::new(AST::NumberLiteral(1)),
                Box::new(AST::FunctionApplication(FunctionApplication {
                    function: Box::new(AST::Identifier(String::from("func"))),
                    argument: Box::new(AST::NumberLiteral(1)),
                })),
            )),
        }));
        assert_eq!(type_of(&input), Type::Boolean);
    }
}
