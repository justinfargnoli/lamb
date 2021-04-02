use crate::{
    parse::{AppCStruct, FdCStruct, RecCStruct, AST},
    type_check::Type,
};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    support::LLVMString,
    types::{BasicType, FunctionType},
    values::{CallSiteValue, FunctionValue, IntValue},
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

        let main = codegen.module(ast);

        codegen.module.verify()?;

        let execution_engine = codegen.module.create_execution_engine()?;

        unsafe {
            execution_engine.run_function_as_main(main, &[]);
        }

        Ok(())
    }

    pub fn module(&mut self, ast: &AST) -> FunctionValue<'ctx> {
        let main =
            self.module
                .add_function("main", self.context.i64_type().fn_type(&[], false), None);

        self.context.append_basic_block(main, "entry");

        match ast {
            AST::NumC(num) => {
                if *num < 0 {
                    panic!("Cannot codegen negative number: {}, num", num);
                }
                self.builder
                    .build_return(Some(&self.context.i64_type().const_int(*num as u64, false)));
            }
            AST::PlusC(op1, op2) => {
                let op1_llvm = self.number(op1);
                let op2_llvm = self.number(op2);
                self.builder.build_return(Some(
                    &self.builder.build_int_add(op1_llvm, op2_llvm, "tmpadd"),
                ));
            }
            AST::MultC(op1, op2) => {
                let op1_llvm = self.number(op1);
                let op2_llvm = self.number(op2);
                self.builder.build_return(Some(
                    &self.builder.build_int_mul(op1_llvm, op2_llvm, "tmpmul"),
                ));
            }
            AST::TrueC => {
                self.builder
                    .build_return(Some(&self.context.bool_type().const_int(1, false)));
            }
            AST::FalseC => {
                self.builder
                    .build_return(Some(&self.context.bool_type().const_int(0, false)));
            }
            AST::EqC(_op1, _op2) => {
                // todo: add ability to test boolean expressions for equality

                // let op1_llvm = self.number(op1);
                // let op2_llvm = self.number(op2);
                // self.builder
                //     .build_return(Some(&self.builder.build_int_compare(
                //         IntPredicate::EQ,
                //         op1_llvm,
                //         op2_llvm,
                //         "compare",
                //     )));

                unimplemented!()
            }
            AST::IfC(_if_struct) => {
                unimplemented!()
            }
            AST::IdC(_) => unreachable!(),
            AST::FdC(def_fn) => {
                self.function_definition(def_fn);
            }
            AST::AppC(app_fn) => {
                self.function_application(app_fn);
            }
            AST::RecC(rec_fn) => {
                self.recursive_function(rec_fn);
            }
        }

        main
    }

    pub fn function(&mut self, _ast: &AST) -> FunctionValue<'ctx> {
        unimplemented!()
    }

    pub fn number(&mut self, _ast: &AST) -> IntValue<'ctx> {
        unimplemented!()
    }

    pub fn boolean(&mut self, _ast: &AST) -> IntValue<'ctx> {
        unimplemented!()
    }

    pub fn type_to_llvm_basic_type(&self, t: &Type) -> Box<dyn BasicType<'ctx> + 'ctx> {
        match t {
            Type::NumT => Box::new(self.context.i64_type()),
            Type::BoolT => Box::new(self.context.bool_type()),
            Type::FunT { arg, ret } => Box::new(
                self.function_prototype(arg, ret)
                    .ptr_type(AddressSpace::Generic),
            ),
        }
    }

    pub fn function_prototype(&self, ret: &Type, arg: &Type) -> FunctionType<'ctx> {
        self.type_to_llvm_basic_type(ret).fn_type(
            &[self.type_to_llvm_basic_type(arg).as_basic_type_enum()],
            false,
        )
    }

    pub fn function_definition(&mut self, def_fn: &FdCStruct) -> FunctionValue<'ctx> {
        let function_value = self.module.add_function(
            "",
            self.function_prototype(&def_fn.ret_type, &def_fn.arg_type),
            None,
        );

        self.context.append_basic_block(function_value, "entry");

        match def_fn.ret_type {
            Type::NumT => {
                let return_value = self.number(&def_fn.body);
                self.builder.build_return(Some(&return_value));
            }
            Type::BoolT => {
                let return_value = self.boolean(&def_fn.body);
                self.builder.build_return(Some(&return_value));
            }
            Type::FunT { .. } => {
                let return_value = self
                    .function(&def_fn.body)
                    .as_global_value()
                    .as_pointer_value();
                self.builder.build_return(Some(&return_value));
            }
        }

        return function_value;
    }

    pub fn function_application(&mut self, _app_fn: &AppCStruct) -> CallSiteValue<'ctx> {
        // let function_value = self.function(&*app_fn.func);
        // let argument = self.number(&*app_fn.arg);
        // self.builder.build_call(
        //     function_value,
        //     &[BasicValueEnum::IntValue(argument)],
        //     "call",
        // )
        unimplemented!()
    }

    pub fn recursive_function<'a>(&mut self, rec_fn: &RecCStruct) {
        let function_value = self.module.add_function(
            &rec_fn.func_name,
            self.function_prototype(&rec_fn.ret_type, &rec_fn.arg_type),
            None,
        );

        self.context.append_basic_block(function_value, "entry");

        match rec_fn.ret_type {
            Type::NumT => {
                let return_value = self.number(&rec_fn.body);
                self.builder.build_return(Some(&return_value));
            }
            Type::BoolT => {
                let return_value = self.boolean(&rec_fn.body);
                self.builder.build_return(Some(&return_value));
            }
            Type::FunT { .. } => {
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
