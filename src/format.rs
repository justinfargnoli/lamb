use crate::{parse::AST, type_check::Type};

use std::fmt::Write;

pub fn format(ast: &AST) -> String {
    let mut output = String::new();
    format_ast(&mut output, ast, 0);
    output
}

fn write_line(output: &mut String, string: &str, tab_count: u32) {
    let mut tabs = (0..tab_count).map(|_| "\t").collect::<String>();
    tabs.push_str(string);
    writeln!(output, "{}\n", tabs).unwrap();
}

fn format_binary(output: &mut String, name: &str, lhs: &AST, rhs: &AST, tab_count: u32) {
    write_line(output, format!("{}(", name).as_str(), tab_count);
    format_ast(output, lhs, tab_count + 1);
    write_line(output, ",", tab_count + 1);
    format_ast(output, rhs, tab_count + 1);
    write_line(output, ")", tab_count);
}

fn format_ast(output: &mut String, ast: &AST, tab_count: u32) {
    match ast {
        AST::TrueC => write_line(output, "trueC", tab_count),
        AST::FalseC => write_line(output, "falseC", tab_count),
        AST::NumC(number) => write_line(output, format!("numC({})", number).as_str(), tab_count),
        AST::PlusC(lhs, rhs) => format_binary(output, "plusC", lhs, rhs, tab_count),
        AST::MultC(lhs, rhs) => format_binary(output, "multC", lhs, rhs, tab_count),
        AST::EqC(lhs, rhs) => format_binary(output, "eqC", lhs, rhs, tab_count),
        AST::IdC(id) => write_line(output, format!("numC({})", id).as_str(), tab_count),
        AST::AppC { func, arg } => format_binary(output, "appC", func, arg, tab_count),
        AST::IfC { cnd, then, els } => {
            write_line(output, format!("{}(", "ifC").as_str(), tab_count);
            format_ast(output, cnd, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, then, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, els, tab_count + 1);
            write_line(output, ")", tab_count);
        }
        AST::RecC {
            func_name,
            arg_name,
            arg_type,
            ret_type,
            body,
            func_use,
        } => {
            write_line(output, format!("{}(", "recC").as_str(), tab_count);
            write_line(
                output,
                format!("\"{}\",", func_name).as_str(),
                tab_count + 1,
            );
            write_line(output, ",", tab_count + 1);
            write_line(output, format!("\"{}\",", arg_name).as_str(), tab_count);
            write_line(output, ",", tab_count + 1);
            format_type(output, arg_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, ret_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, body, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, func_use, tab_count + 1);
            write_line(output, ")", tab_count);
        }
        AST::FdC {
            arg_name,
            arg_type,
            ret_type,
            body,
        } => {
            write_line(output, format!("{}(", "fdC").as_str(), tab_count);
            write_line(output, format!("\"{}\",", arg_name).as_str(), tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, arg_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, ret_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, body, tab_count + 1);
            write_line(output, ")", tab_count);
        }
    }
}

fn format_type(output: &mut String, ty: &Type, tab_count: u32) {
    match ty {
        Type::BoolT => write_line(output, "boolT", tab_count),
        Type::NumT => write_line(output, "numT", tab_count),
        Type::FunT { arg, ret } => {
            write_line(output, format!("{}(", "funT").as_str(), tab_count);
            format_type(output, arg, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, ret, tab_count + 1);
            write_line(output, ")", tab_count);
        }
    }
}
