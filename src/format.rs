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
        AST::TrueLiteral => write_line(output, "trueC", tab_count),
        AST::FalseLiteral => write_line(output, "falseC", tab_count),
        AST::NumberLiteral(number) => {
            write_line(output, format!("numC({})", number).as_str(), tab_count)
        }
        AST::Plus(lhs, rhs) => format_binary(output, "plusC", lhs, rhs, tab_count),
        AST::Multiply(lhs, rhs) => format_binary(output, "multC", lhs, rhs, tab_count),
        AST::Equals(lhs, rhs) => format_binary(output, "eqC", lhs, rhs, tab_count),
        AST::Identifier(id) => write_line(output, format!("numC({})", id).as_str(), tab_count),
        AST::FunctionApplication(function_application) => format_binary(
            output,
            "appC",
            &*function_application.function,
            &*function_application.argument,
            tab_count,
        ),
        AST::If(if_struct) => {
            write_line(output, format!("{}(", "ifC").as_str(), tab_count);
            format_ast(output, &*if_struct.condition, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, &*if_struct.then, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, &*if_struct.els, tab_count + 1);
            write_line(output, ")", tab_count);
        }
        AST::RecursiveFunction(recursive_function) => {
            write_line(output, format!("{}(", "recC").as_str(), tab_count);
            write_line(
                output,
                format!("\"{}\",", recursive_function.function_name).as_str(),
                tab_count + 1,
            );
            write_line(output, ",", tab_count + 1);
            write_line(
                output,
                format!("\"{}\",", recursive_function.argument_name).as_str(),
                tab_count,
            );
            write_line(output, ",", tab_count + 1);
            format_type(output, &recursive_function.argument_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, &recursive_function.return_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, &*recursive_function.body, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, &*recursive_function.function_use, tab_count + 1);
            write_line(output, ")", tab_count);
        }
        AST::FunctionDefinition(function_definition) => {
            write_line(output, format!("{}(", "fdC").as_str(), tab_count);
            write_line(
                output,
                format!("\"{}\",", function_definition.argument_name).as_str(),
                tab_count + 1,
            );
            write_line(output, ",", tab_count + 1);
            format_type(output, &function_definition.argument_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, &function_definition.return_type, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_ast(output, &*function_definition.body, tab_count + 1);
            write_line(output, ")", tab_count);
        }
    }
}

fn format_type(output: &mut String, ty: &Type, tab_count: u32) {
    match ty {
        Type::Boolean => write_line(output, "boolT", tab_count),
        Type::Number => write_line(output, "numT", tab_count),
        Type::Function { argument, ret } => {
            write_line(output, format!("{}(", "funT").as_str(), tab_count);
            format_type(output, argument, tab_count + 1);
            write_line(output, ",", tab_count + 1);
            format_type(output, ret, tab_count + 1);
            write_line(output, ")", tab_count);
        }
    }
}
