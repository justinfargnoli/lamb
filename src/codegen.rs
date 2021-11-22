use crate::{
    type_check::Type,
    type_check::{TypedAST, TypedASTEnum},
};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    support::LLVMString,
    types::{BasicType, BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, CallableValue, FunctionValue},
    AddressSpace, IntPredicate,
};

use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
};

pub fn run(ast: &TypedAST) -> Result<u64, LLVMString> {
    CodeGen::run(ast)
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn run(typed_ast: &TypedAST) -> Result<u64, LLVMString> {
        let context = Context::create();
        let mut codegen = CodeGen {
            context: &context,
            module: context.create_module("lamb_module"),
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
            _ => panic!("Cannot compile a function that returns a function"),
        };
        let main_function = self.module.add_function(
            "lamb_main_function",
            main_return_type.fn_type(&[], false),
            None,
        );

        let main_basic_block = self
            .context
            .append_basic_block(main_function, "lamb_main_entry");
        self.builder.position_at_end(main_basic_block);

        let return_value = self.codegen(typed_ast).into_int_value();
        self.builder.build_return(Some(&return_value));

        main_function.verify(false);

        main_function
    }

    fn codegen_helper(
        &mut self,
        typed_ast: &TypedAST,
        argument_values: &mut HashMap<String, BasicValueEnum<'ctx>>,
    ) -> BasicValueEnum<'ctx> {
        match &*typed_ast.ast {
            TypedASTEnum::NumberLiteral(num) => self
                .context
                .i64_type()
                .const_int((*num).try_into().unwrap(), false)
                .into(),
            TypedASTEnum::Plus(op1, op2) => {
                let lhs = self.codegen_helper(op1, argument_values).into_int_value();
                let rhs = self.codegen_helper(op2, argument_values).into_int_value();
                self.builder.build_int_add(lhs, rhs, "lamb_plus").into()
            }
            TypedASTEnum::Multiply(op1, op2) => {
                let lhs = self.codegen_helper(op1, argument_values).into_int_value();
                let rhs = self.codegen_helper(op2, argument_values).into_int_value();
                self.builder.build_int_mul(lhs, rhs, "lamb_multiply").into()
            }
            TypedASTEnum::TrueLiteral => self.context.bool_type().const_int(1, false).into(),
            TypedASTEnum::FalseLiteral => self.context.bool_type().const_int(0, false).into(),
            TypedASTEnum::Equals(op1, op2) => {
                let lhs = self.codegen_helper(op1, argument_values).into_int_value();
                let rhs = self.codegen_helper(op2, argument_values).into_int_value();
                let comparison =
                    self.builder
                        .build_int_compare(IntPredicate::EQ, lhs, rhs, "lamb_equals");
                self.builder
                    .build_int_cast(
                        comparison,
                        self.context.bool_type(),
                        "lamb_equals_i64_to_i1_cast",
                    )
                    .into()
            }
            TypedASTEnum::If(if_struct) => {
                let condition = self.codegen_helper(&if_struct.condition, argument_values);
                let then_block = self.context.insert_basic_block_after(
                    self.builder.get_insert_block().unwrap(),
                    "lamb_then_block",
                );
                let else_block = self
                    .context
                    .insert_basic_block_after(then_block, "lamb_else_block");
                self.builder.build_conditional_branch(
                    condition.into_int_value(),
                    then_block,
                    else_block,
                );

                let post_dominator_block = self
                    .context
                    .insert_basic_block_after(else_block, "lamb_post_dominator_block");

                self.builder.position_at_end(then_block);
                let then_value = self.codegen_helper(&if_struct.then, argument_values);
                self.builder
                    .build_unconditional_branch(post_dominator_block);
                let then_post_dominator_block = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(else_block);
                let else_value = self.codegen_helper(&if_struct.els, argument_values);
                self.builder
                    .build_unconditional_branch(post_dominator_block);
                let else_post_dominator_block = self.builder.get_insert_block().unwrap();

                self.builder.position_at_end(post_dominator_block);

                let phi_value = match typed_ast.ty {
                    Type::Boolean => self
                        .builder
                        .build_phi(self.context.bool_type(), "lamb_phi_bool"),
                    Type::Number => self
                        .builder
                        .build_phi(self.context.i64_type(), "lamb_phi_int"),
                    Type::Function { .. } => self
                        .builder
                        .build_phi(self.llvm_basic_type(&typed_ast.ty), "lamb_hi_bool"),
                };
                phi_value.add_incoming(&[
                    (&then_value, then_post_dominator_block),
                    (&else_value, else_post_dominator_block),
                ]);
                phi_value.as_basic_value()
            }
            TypedASTEnum::Identifier(identifier) => match argument_values.get(identifier) {
                Some(basic_value_enum) => *basic_value_enum,
                None => {
                    self.module.print_to_stderr();
                    panic!("identifier not found: ({})", identifier)
                }
            },
            TypedASTEnum::FunctionApplication(function_application) => {
                let function_pointer = self
                    .codegen_helper(&function_application.function, argument_values)
                    .into_pointer_value();
                let argument = self.codegen_helper(&function_application.argument, argument_values);
                self.builder
                    .build_call(
                        CallableValue::try_from(function_pointer).unwrap(),
                        &[argument.into()],
                        "lamb_function_call",
                    )
                    .try_as_basic_value()
                    .unwrap_left()
            }
            TypedASTEnum::FunctionDefinition(function_definition) => {
                let previous_basic_block = self.builder.get_insert_block().unwrap();

                let function_type = self.function_prototype(
                    &function_definition.argument_type,
                    &function_definition.return_type,
                );
                let function_value = self
                    .module
                    .add_function("lamb_function", function_type, None);

                let function_entry_basic_block = self
                    .context
                    .append_basic_block(function_value, "lamb_function_entry");
                self.builder.position_at_end(function_entry_basic_block);

                argument_values.insert(
                    function_definition.argument_name.clone(),
                    function_value.get_first_param().unwrap(),
                );
                let return_value = self.codegen_helper(&function_definition.body, argument_values);
                self.builder.build_return(Some(&return_value));
                argument_values
                    .remove(&function_definition.argument_name)
                    .unwrap();

                self.builder.position_at_end(previous_basic_block);

                function_value.verify(false);

                function_value.as_global_value().as_pointer_value().into()
            }
            TypedASTEnum::RecursiveFunction(recursive_function) => {
                let function_type = self.function_prototype(
                    &recursive_function.argument_type,
                    &recursive_function.return_type,
                );
                let function_value = self.module.add_function(
                    &recursive_function.function_name,
                    function_type,
                    None,
                );

                let function_entry_basic_block = self
                    .context
                    .append_basic_block(function_value, "lamb_recursive_function_entry");
                self.builder.position_at_end(function_entry_basic_block);

                argument_values.insert(
                    recursive_function.function_name.clone(),
                    function_value.as_global_value().as_pointer_value().into(),
                );
                argument_values.insert(
                    recursive_function.argument_name.clone(),
                    function_value.get_first_param().unwrap(),
                );
                let return_value = self.codegen_helper(&recursive_function.body, argument_values);
                self.builder.build_return(Some(&return_value));
                argument_values
                    .remove(&recursive_function.argument_name)
                    .unwrap();

                function_value.verify(false);

                self.builder.position_at_end(
                    function_value
                        .get_previous_function()
                        .unwrap()
                        .get_last_basic_block()
                        .unwrap(),
                );
                let function_use_result_value = self.codegen(&recursive_function.function_use);
                argument_values
                    .remove(&recursive_function.function_name)
                    .unwrap();

                function_use_result_value
            }
        }
    }

    fn codegen(&mut self, typed_ast: &TypedAST) -> BasicValueEnum<'ctx> {
        self.codegen_helper(typed_ast, &mut HashMap::new())
    }

    fn llvm_basic_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Number => self.context.i64_type().into(),
            Type::Boolean => self.context.bool_type().into(),
            Type::Function { argument, ret } => self
                .llvm_basic_type(ret)
                .fn_type(&[self.llvm_basic_type(argument).into()], false)
                .ptr_type(AddressSpace::Global)
                .into(),
        }
    }

    fn function_prototype(&mut self, argument: &Type, ret: &Type) -> FunctionType<'ctx> {
        let argument_type = self.llvm_basic_type(argument);
        let return_type = self.llvm_basic_type(ret);

        return_type.fn_type(&[argument_type.into()], false)
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
        let _ = CodeGen::run(&input).unwrap();
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
