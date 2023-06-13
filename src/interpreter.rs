use crate::functions::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Str,
    Char,
    Bool,
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

            "exit" => silm_exit(tokens[1..].to_vec(), input_name, line_number),

            "#" => {}

            "" => {}

            _ => {
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
