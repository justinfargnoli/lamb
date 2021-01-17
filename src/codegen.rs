use crate::{
    parse::{AppCStruct, FdCStruct, RecCStruct, AST},
    type_check::Type,
};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    support::LLVMString,
    types::BasicTypeEnum,
    values::{BasicValueEnum, CallSiteValue, FunctionValue, IntValue},
    AddressSpace,
};

pub fn run(ast: &AST) -> Result<(), LLVMString> {
    CodeGen::run(ast)
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn run(ast: &AST) -> Result<(), LLVMString> {
        let context = Context::create();
        let mut codegen = CodeGen {
            context: &context,
            module: context.create_module("main"),
            builder: context.create_builder(),
        };

        codegen.module(ast);

        codegen.module.verify()
    }

    pub fn module(&mut self, ast: &AST) {
        self.module
            .add_function("main", self.context.i64_type().fn_type(&[], false), None);

        match ast {
            AST::NumC(num) => {
                if *num < 0 {
                    panic!("Cannot codegen negative number: {}, num");
                } else {
                    self.context.i64_type().const_int(*num as u64, false);
                }
            }
            AST::IdC(variable_name) => panic!("IdC cannot find variable {}", variable_name),
            AST::AppC(app_fn) => {
                self.function_application(app_fn);
            }
            AST::FdC(def_fn) => {
                self.function_definition(def_fn);
            }
            AST::RecC(rec_fn) => {
                self.recursive_function(rec_fn);
            }
            _ => unimplemented!(),
        }
    }

    pub fn function(&mut self, _ast: &AST) -> FunctionValue<'ctx> {
        unimplemented!()
    }

    pub fn number(&mut self, _ast: &AST) -> IntValue<'ctx> {
        unimplemented!()
    }

    pub fn identifier(&mut self, _ast: &AST) -> IntValue<'ctx> {
        unimplemented!()
    }

    pub fn function_definition(&mut self, def_fn: &FdCStruct) -> FunctionValue<'ctx> {
        let llvm_ret_type = match def_fn.ret_type {
            Type::NumT => self.context.i64_type(),
            Type::BoolT => self.context.i8_type(),
            Type::FunT { .. } => unimplemented!(),
        };
        let llvm_arg_type = match def_fn.arg_type {
            Type::NumT => BasicTypeEnum::IntType(self.context.i64_type()),
            Type::BoolT => BasicTypeEnum::IntType(self.context.i8_type()),
            Type::FunT { .. } => {
                BasicTypeEnum::PointerType(self.context.i64_type().ptr_type(AddressSpace::Generic))
            }
        };
        let _function_value =
            self.module
                .add_function("", llvm_ret_type.fn_type(&[llvm_arg_type], false), None);
        unimplemented!()
    }

    pub fn function_application(&mut self, app_fn: &AppCStruct) -> CallSiteValue<'ctx> {
        let function_value = self.function(&*app_fn.func);
        let argument = self.number(&*app_fn.arg);
        self.builder.build_call(
            function_value,
            &[BasicValueEnum::IntValue(argument)],
            "call",
        )
    }

    pub fn recursive_function<'a>(&self, _rec_fn: &RecCStruct) -> FunctionValue<'ctx> {
        unimplemented!()
    }

    pub fn ret_type_to_llvm_type(&self, t: Type) -> BasicTypeEnum {
        match t {
            Type::NumT => BasicTypeEnum::IntType(self.context.i64_type()),
            Type::BoolT => BasicTypeEnum::IntType(self.context.i8_type()),
            Type::FunT { arg: _, ret } => self.ret_type_to_llvm_type(*ret),
        }
    }
}

// pub fn module<'ctx: 'a, 'a>(codegen: &'a mut CodeGen<'ctx>, ast: &AST) {
//     codegen
//         .module
//         .add_function("main", codegen.context.i64_type().fn_type(&[], false), None);

//     match ast {
//         AST::NumC(num) => {
//             if *num < 0 {
//                 panic!("Cannot codegen negative number: {}, num");
//             } else {
//                 codegen.context.i64_type().const_int(*num as u64, false);
//             }
//         }
//         AST::IdC(variable_name) => panic!("IdC cannot find variable {}", variable_name),
//         AST::AppC(app_fn) => {
//             function_application(codegen, app_fn);
//         }
//         AST::FdC(def_fn) => {
//             function_definition(codegen, def_fn);
//         }
//         AST::RecC(rec_fn) => {
//             recursive_function(codegen, rec_fn);
//         }
//         _ => unimplemented!(),
//     }
// }

// pub fn function<'ctx: 'a, 'a>(_codegen: &'a mut CodeGen<'ctx>, _ast: &AST) -> FunctionValue<'ctx> {
//     unimplemented!()
// }

// pub fn number<'ctx: 'a, 'a>(_codegen: &'a mut CodeGen<'ctx>, _ast: &AST) -> IntValue<'ctx> {
//     unimplemented!()
// }

// pub fn identifier<'ctx: 'a, 'a>(_codegen: &'a mut CodeGen<'ctx>, _ast: &AST) -> IntValue<'ctx> {
//     unimplemented!()
// }

// pub fn function_definition<'ctx: 'a, 'a>(
//     codegen: &'a mut CodeGen<'ctx>,
//     def_fn: &FdCStruct,
// ) -> FunctionValue<'ctx> {
//     let llvm_ret_type = match def_fn.ret_type {
//         Type::NumT => codegen.context.i64_type(),
//         Type::BoolT => codegen.context.i8_type(),
//         Type::FunT { .. } => unimplemented!(),
//     };
//     let llvm_arg_type = match def_fn.arg_type {
//         Type::NumT => BasicTypeEnum::IntType(codegen.context.i64_type()),
//         Type::BoolT => BasicTypeEnum::IntType(codegen.context.i8_type()),
//         Type::FunT { .. } => {
//             BasicTypeEnum::PointerType(codegen.context.i64_type().ptr_type(AddressSpace::Generic))
//         }
//     };
//     let function_value =
//         codegen
//             .module
//             .add_function("", llvm_ret_type.fn_type(&[llvm_arg_type], false), None);
//     unimplemented!()
// }

// pub fn function_application<'ctx: 'a, 'a>(
//     codegen: &'a mut CodeGen<'ctx>,
//     app_fn: &AppCStruct,
// ) -> CallSiteValue<'ctx> {
//     let function_value = function(codegen, &*app_fn.func);
//     let argument = number(codegen, &*app_fn.arg);
//     codegen.builder.build_call(
//         function_value,
//         &[BasicValueEnum::IntValue(argument)],
//         "call",
//     )
// }

// pub fn recursive_function<'ctx: 'a, 'a>(
//     _codegen: &'a mut CodeGen<'ctx>,
//     _rec_fn: &RecCStruct,
// ) -> FunctionValue<'ctx> {
//     unimplemented!()
// }
