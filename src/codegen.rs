use crate::{
    type_check::Type,
    type_check::{
        TypedAST, TypedASTEnum, TypedFunctionApplication, TypedFunctionDefinition,
        TypedRecursiveFunction,
    },
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    support::LLVMString,
    types::{BasicType, BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, CallSiteValue, FunctionValue, IntValue},
    AddressSpace, IntPredicate,
};

pub fn run(ast: &TypedAST) -> Result<u64, LLVMString> {
    CodeGen::run(ast)
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn run(typed_ast: &TypedAST) -> Result<u64, LLVMString> {
        let context = Context::create();
        let mut codegen = CodeGen {
            context: &context,
            module: context.create_module("tlc_module"),
            builder: context.create_builder(),
        };

        let main = codegen.module(typed_ast);

        codegen.module.verify()?;

        let execution_engine = codegen.module.create_execution_engine()?;

        let run_result;
        unsafe {
            run_result = execution_engine.run_function(main, &[]).as_int(false);
        }

        Ok(run_result)
    }

    fn module(&mut self, typed_ast: &TypedAST) -> FunctionValue<'ctx> {
        let main_return_type = match typed_ast.ty {
            Type::Number => self.context.i64_type(),
            Type::Boolean => self.context.bool_type(),
            _ => unreachable!(),
        };
        let main_function = self.module.add_function(
            "tlc_main_function",
            main_return_type.fn_type(&[], false),
            None,
        );

        let main_basic_block = self.context.append_basic_block(main_function, "entry");
        self.builder.position_at_end(main_basic_block);

        match typed_ast.ty {
            Type::Number | Type::Boolean => {
                let return_value = self.codegen(typed_ast).into_int_value();
                self.builder.build_return(Some(&return_value))
            }
            _ => panic!("Cannot compile a function that returns a function"),
        };

        main_function.verify(false);

        main_function
    }

    fn function(&mut self, typed_ast: &TypedAST) -> FunctionValue<'ctx> {
        match &*typed_ast.ast {
            TypedASTEnum::If(_if_struct) => {
                unimplemented!()
            }
            TypedASTEnum::Identifier(_) => unimplemented!(),
            TypedASTEnum::FunctionDefinition(_def_fn) => {
                // self.function_definition(def_fn)
                unimplemented!()
            }
            TypedASTEnum::FunctionApplication(_app_fn) => {
                // self.function_application(&app_fn)
                unimplemented!()
            }
            TypedASTEnum::RecursiveFunction(_rec_fn) => {
                // self.recursive_function(&rec_fn)
                unimplemented!()
            }
            _ => unreachable!(),
        }
    }

    fn codegen(&mut self, typed_ast: &TypedAST) -> BasicValueEnum<'ctx> {
        match &*typed_ast.ast {
            TypedASTEnum::NumberLiteral(num) => {
                if *num < 0 {
                    panic!("Cannot codegen negative number: {}, num", num);
                }
                self.context.i64_type().const_int(*num as u64, false).into()
            }
            TypedASTEnum::Plus(op1, op2) => {
                let lhs = self.codegen(op1).into_int_value();
                let rhs = self.codegen(op2).into_int_value();
                self.builder.build_int_add(lhs, rhs, "tlc_plus").into()
            }
            TypedASTEnum::Multiply(op1, op2) => {
                let lhs = self.codegen(op1).into_int_value();
                let rhs = self.codegen(op2).into_int_value();
                self.builder.build_int_mul(lhs, rhs, "tlc_multiply").into()
            }
            TypedASTEnum::TrueLiteral => self.context.bool_type().const_int(1, false).into(),
            TypedASTEnum::FalseLiteral => self.context.bool_type().const_int(0, false).into(),
            TypedASTEnum::Equals(op1, op2) => {
                let lhs = self.codegen(op1).into_int_value();
                let rhs = self.codegen(op2).into_int_value();
                let comparison =
                    self.builder
                        .build_int_compare(IntPredicate::EQ, lhs, rhs, "tlc_equals");
                self.builder
                    .build_int_cast(
                        comparison,
                        self.context.bool_type(),
                        "tlc_equals_i64_to_i1_cast",
                    )
                    .into()
            }
            TypedASTEnum::If(_if_struct) => {
                unimplemented!()
            }
            TypedASTEnum::Identifier(_) => unimplemented!(),
            TypedASTEnum::FunctionDefinition(_def_fn) => {
                // self.function_definition(def_fn)
                unimplemented!()
            }
            TypedASTEnum::FunctionApplication(_app_fn) => {
                // self.function_application(&app_fn);
                unimplemented!()
            }
            TypedASTEnum::RecursiveFunction(_rec_fn) => {
                // self.recursive_function(&rec_fn);
                unimplemented!()
            }
        }
    }

    fn llvm_basic_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Number => self.context.i64_type().into(),
            Type::Boolean => self.context.i8_type().into(),
            Type::Function { argument, ret } => self
                .llvm_basic_type(ret)
                .fn_type(&[self.llvm_basic_type(argument)], false)
                .ptr_type(AddressSpace::Global)
                .into(),
        }
    }

    fn function_prototype(&mut self, argument: &Type, ret: &Type) -> FunctionType<'ctx> {
        let argument_type = self.llvm_basic_type(argument);
        let return_type = self.llvm_basic_type(ret);

        return_type.fn_type(&[argument_type], false)
    }

    fn function_definition(&mut self, def_fn: &TypedFunctionDefinition) -> FunctionValue<'ctx> {
        let function_type = self.function_prototype(&def_fn.return_type, &def_fn.argument_type);
        let function_value = self
            .module
            .add_function("tlc_function", function_type, None);

        self.context.append_basic_block(function_value, "entry");

        match def_fn.return_type {
            Type::Number => {
                let return_value = self.codegen(&def_fn.body).into_int_value();
                self.builder.build_return(Some(&return_value));
            }
            Type::Boolean => {
                let return_value = self.codegen(&def_fn.body).into_int_value();
                self.builder.build_return(Some(&return_value));
            }
            Type::Function { .. } => {
                let return_value = self
                    .function(&def_fn.body)
                    .as_global_value()
                    .as_pointer_value();
                self.builder.build_return(Some(&return_value));
            }
        }

        function_value
    }

    fn function_application(&mut self, _app_fn: &TypedFunctionApplication) -> CallSiteValue<'ctx> {
        // let function_value = self.function(&*app_fn.func);
        // let argument = self.number(&*app_fn.arg);
        // self.builder.build_call(
        //     function_value,
        //     &[BasicValueEnum::IntValue(argument)],
        //     "call",
        // )
        unimplemented!()
    }

    fn recursive_function(&mut self, rec_fn: &TypedRecursiveFunction) {
        let function_type = self.function_prototype(&rec_fn.return_type, &rec_fn.argument_type);
        let function_value = self
            .module
            .add_function(&rec_fn.function_name, function_type, None);

        self.context.append_basic_block(function_value, "entry");

        match rec_fn.return_type {
            Type::Number => {
                let return_value = self.codegen(&rec_fn.body).into_int_value();
                self.builder.build_return(Some(&return_value));
            }
            Type::Boolean => {
                let return_value = self.codegen(&rec_fn.body).into_int_value();
                self.builder.build_return(Some(&return_value));
            }
            Type::Function { .. } => {
                let return_value = self
                    .function(&rec_fn.body)
                    .as_global_value()
                    .as_pointer_value();
                self.builder.build_return(Some(&return_value));
            }
        }

        // todo: compile rec_fn.func_use
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codegen_true_literal() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::TrueLiteral),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 1)
    }

    #[test]
    fn codegen_false_literal() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::FalseLiteral),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 0)
    }

    #[test]
    fn codegen_number_literal_0() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::NumberLiteral(0)),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 0)
    }

    #[test]
    #[should_panic]
    fn codegen_number_literal_negative() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::NumberLiteral(-1)),
        };
        let _ = CodeGen::run(&input);
    }

    #[test]
    fn codegen_number_literal_1() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::NumberLiteral(1)),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 1)
    }

    #[test]
    fn codegen_number_literal_99() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::NumberLiteral(99)),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 99)
    }

    #[test]
    fn codegen_plus_1() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::Plus(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(0)),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(0)),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 0)
    }

    #[test]
    fn codegen_plus_2() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::Plus(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(3)),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(2)),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 5)
    }

    #[test]
    fn codegen_multiply_1() {
        let input = TypedAST {
            ty: Type::Number,
            ast: Box::new(TypedASTEnum::Multiply(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(3)),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(2)),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 6)
    }

    #[test]
    fn codegen_equals_number() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::Equals(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(0)),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::NumberLiteral(0)),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 1)
    }

    #[test]
    fn codegen_equals_boolean_1() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::Equals(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::TrueLiteral),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::TrueLiteral),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 1)
    }

    #[test]
    fn codegen_equals_boolean_2() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::Equals(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::FalseLiteral),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::TrueLiteral),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 0)
    }

    #[test]
    fn codegen_equals_boolean_3() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::Equals(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::TrueLiteral),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::FalseLiteral),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 0)
    }

    #[test]
    fn codegen_equals_boolean_4() {
        let input = TypedAST {
            ty: Type::Boolean,
            ast: Box::new(TypedASTEnum::Equals(
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::FalseLiteral),
                },
                TypedAST {
                    ty: Type::Number,
                    ast: Box::new(TypedASTEnum::FalseLiteral),
                },
            )),
        };
        assert_eq!(CodeGen::run(&input).unwrap(), 1)
    }
}
