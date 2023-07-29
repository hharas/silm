use crate::{
    commands::*,
    helper::{assign, get_variable, throw_error},
    version::VERSION,
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

    assign(
        Variable {
            identifier: "$version$".into(),
            datatype: DataType::Str,
            value: VERSION.into(),
        },
        variables,
    );

    assign(
        Variable {
            identifier: "$line$".into(),
            datatype: DataType::Str,
            value: line.clone(),
        },
        variables,
    );

    assign(
        Variable {
            identifier: "$input_name$".into(),
            datatype: DataType::Str,
            value: input_name.clone(),
        },
        variables,
    );

    assign(
        Variable {
            identifier: "$line_number$".into(),
            datatype: DataType::Int,
            value: line_number.to_string(),
        },
        variables,
    );

    if !line.trim().starts_with('#') && !tokens.is_empty() {
        match tokens[0] {
            "let" => silm_let(tokens[1..].to_vec(), input_name, line_number, variables),

            "println" => silm_println(tokens[1..].to_vec(), input_name, line_number, variables),

            "formatln" => silm_formatln(tokens[1..].to_vec(), input_name, line_number, variables),

            "readln" => silm_readln(tokens[1..].to_vec(), input_name, line_number, variables),

            "block" => silm_block(tokens[1..].to_vec(), input_name, line_number, variables),

            "interpret" => silm_interpret(tokens[1..].to_vec(), input_name, line_number, variables),

            "eval" => silm_eval(tokens[1..].to_vec(), input_name, line_number, variables),

            "import" => silm_import(tokens[1..].to_vec(), input_name, line_number, variables),

            "if" => silm_if(tokens[1..].to_vec(), input_name, line_number, variables),

            "exit" => silm_exit(tokens[1..].to_vec(), input_name, line_number),

            "" => {}

            _ => {
                if let Some(variable) = get_variable(tokens[0], variables) {
                    if variable.datatype == DataType::Block {
                        if tokens.len() >= 2 {
                            if tokens[1] == "()" {
                                let mut block_variables: Vec<Variable> = Vec::new();
                                let mut current_line = 0;

                                for line in variable.value.lines() {
                                    current_line += 1;

                                    interpret(
                                        line.to_string(),
                                        format!("<block {}>", variable.identifier),
                                        current_line,
                                        &mut block_variables,
                                    )
                                }
                            } else {
                                throw_error(
                                    "block call must contain two parantheses",
                                    &variable.identifier,
                                    input_name,
                                    line_number,
                                );
                            }
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
                            "{}:{}: unrecognised command: {}",
                            input_name,
                            line_number,
                            tokens.join(" ")
                        );
                    }
                } else {
                    println!(
                        "{}:{}: unrecognised command: {}",
                        input_name,
                        line_number,
                        tokens.join(" ")
                    );
                }
            }
        }
    }
}
