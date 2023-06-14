use crate::{
    functions::*,
    helper::{get_variable, throw_error},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Str,
    Char,
    Bool,
    Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub datatype: DataType,
    pub identifier: String,
    pub value: String,
}

pub fn interpret(
    line: String,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if !tokens.is_empty() {
        match tokens[0] {
            "let" => silm_let(tokens[1..].to_vec(), input_name, line_number, variables),

            "println" => silm_println(tokens[1..].to_vec(), input_name, line_number, variables),

            "formatln" => silm_formatln(tokens[1..].to_vec(), input_name, line_number, variables),

            "readline" => silm_readline(tokens[1..].to_vec(), input_name, line_number, variables),

            "block" => silm_block(tokens[1..].to_vec(), input_name, line_number, variables),

            "exit" => silm_exit(tokens[1..].to_vec(), input_name, line_number),

            "#" => {}

            "" => {}

            _ => {
                if let Some(variable) = get_variable(tokens[0], variables) {
                    if variable.datatype == DataType::Block {
                        if tokens[1] == "()" {
                            interpret(
                                variable.value,
                                format!("<block {}>", variable.identifier),
                                0,
                                variables,
                            )
                        } else {
                            throw_error(
                                "invalid block call",
                                &variable.identifier,
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        println!(
                            "{}:{}: unrecognised function: {}",
                            input_name,
                            line_number,
                            tokens.join(" ")
                        );
                    }
                } else {
                    println!(
                        "{}:{}: unrecognised function: {}",
                        input_name,
                        line_number,
                        tokens.join(" ")
                    );
                }
            }
        }
    }
}
