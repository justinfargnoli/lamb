use crate::{
    parse::{FdCStruct, AST},
    type_check::Type,
};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    support::LLVMString,
    types::{BasicType, FunctionType},
    values::FunctionValue,
    AddressSpace,
};

pub fn run(ast: &AST) -> Result<(), LLVMString> {
    CodeGen::run(ast)
}

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    _builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn run(ast: &AST) -> Result<(), LLVMString> {
        let context = Context::create();
        let mut codegen = CodeGen {
            context: &context,
            module: context.create_module("main"),
            _builder: context.create_builder(),
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
            // AST::AppC(app_fn) => {
            //     self.function_application(app_fn);
            // }
            AST::FdC(def_fn) => {
                self.function_definition(def_fn);
            }
            // AST::RecC(rec_fn) => {
            //     self.recursive_function(rec_fn);
            // }
            _ => unimplemented!(),
        }
    }

    // pub fn function(&mut self, _ast: &AST) -> FunctionValue<'ctx> {
    //     unimplemented!()
    // }

    // pub fn number(&mut self, _ast: &AST) -> IntValue<'ctx> {
    //     unimplemented!()
    // }

    // pub fn identifier(&mut self, _ast: &AST) -> IntValue<'ctx> {
    //     unimplemented!()
    // }

    pub fn type_to_llvm_basic_type(&self, t: &Type) -> Box<dyn BasicType<'ctx> + 'ctx> {
        match t {
            Type::NumT => Box::new(self.context.i64_type()),
            Type::BoolT => Box::new(self.context.i8_type()),
            Type::FunT { arg, ret } => Box::new(
                self.function_prototype(arg, ret)
                    .ptr_type(AddressSpace::Generic),
            ),
        }
    }

    pub fn function_prototype(&self, ret: &Type, arg: &Type) -> FunctionType<'ctx> {
        match ret {
            Type::NumT => self.context.i64_type().fn_type(
                &[self.type_to_llvm_basic_type(arg).as_basic_type_enum()],
                false,
            ),
            Type::BoolT => self.context.i8_type().fn_type(
                &[self.type_to_llvm_basic_type(arg).as_basic_type_enum()],
                false,
            ),
            Type::FunT { .. } => self.type_to_llvm_basic_type(ret).fn_type(
                &[self.type_to_llvm_basic_type(arg).as_basic_type_enum()],
                false,
            ),
        }
    }

    pub fn function_definition(&mut self, def_fn: &FdCStruct) -> FunctionValue<'ctx> {
        let _function_value = self.module.add_function(
            "",
            self.function_prototype(&def_fn.ret_type, &def_fn.arg_type),
            None,
        );
        unimplemented!()
    }

    // pub fn function_application(&mut self, app_fn: &AppCStruct) -> CallSiteValue<'ctx> {
    //     let function_value = self.function(&*app_fn.func);
    //     let argument = self.number(&*app_fn.arg);
    //     self.builder.build_call(
    //         function_value,
    //         &[BasicValueEnum::IntValue(argument)],
    //         "call",
    //     )
    // }

    // pub fn recursive_function<'a>(&self, _rec_fn: &RecCStruct) -> FunctionValue<'ctx> {
    //     unimplemented!()
    // }
}
