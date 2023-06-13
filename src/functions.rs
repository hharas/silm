use std::{
    io::{self, Write},
    process::exit,
};

use crate::{
    helper::{assign, extract_data, get_variable, represent_datatype, throw_error},
    interpreter::{DataType, Variable},
};

pub fn silm_let(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if tokens.len() >= 3 {
        if tokens[1] == "=" {
            let identifier = tokens[0].to_string();
            let supposed_value = &mut tokens[2..].join(" ");

            match extract_data(supposed_value, variables) {
                Ok(returned_variable_option) => {
                    if let Some(returned_variable) = returned_variable_option {
                        let datatype = returned_variable.datatype;
                        let value = returned_variable.value;

                        assign(
                            Variable {
                                identifier,
                                datatype,
                                value,
                            },
                            variables,
                        )
                    } else {
                        throw_error("empty value given", "let", input_name, line_number)
                    }
                }
                Err(error) => throw_error(&error, "let", input_name, line_number),
            }
        } else {
            throw_error(
                "assignment operator (=) not found",
                "let",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error(
            "improper amount of tokens (!>= 4)",
            "let",
            input_name,
            line_number,
        );
    }
}

pub fn silm_println(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &[Variable],
) {
    if !tokens.is_empty() {
        let argument = &mut tokens[0..].join(" ");

        if argument.starts_with('(') && argument.ends_with(')') {
            argument.remove(0);
            argument.pop();

            match extract_data(argument, variables) {
                Ok(variable_option) => {
                    if let Some(variable) = variable_option {
                        println!("{}", variable.value);
                    } else {
                        println!();
                    }
                }
                Err(error) => {
                    throw_error(&error, "println", input_name, line_number);
                }
            }
        } else {
            throw_error(
                "function call does not contain two parantheses",
                "println",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error("no argument provided", "println", input_name, line_number);
    }
}

pub fn silm_formatln(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &[Variable],
) {
    if !tokens.is_empty() {
        let argument_str = &mut tokens[0..].join(" ");

        if argument_str.starts_with('(') && argument_str.ends_with(')') {
            argument_str.remove(0);
            argument_str.pop();

            match extract_data(argument_str, variables) {
                Ok(argument_option) => {
                    if let Some(argument) = argument_option {
                        let mut result = argument.value;

                        // Can't loop over `variables` twice or else the borrow checker will go bananas
                        for index in 0..variables.len() {
                            let variable = &variables[index];
                            let placeholder = format!("{{{}}}", variable.identifier);

                            if let Some(found_variable) =
                                get_variable(&variable.identifier, variables)
                            {
                                let value = found_variable.value;
                                result = result.replace(&placeholder, &value);
                            }
                        }

                        println!("{}", result);
                    } else {
                        throw_error(
                            "function requires one argument",
                            "formatln",
                            input_name,
                            line_number,
                        )
                    }
                }

                Err(error) => throw_error(&error, "formatln", input_name, line_number),
            }
        } else {
            throw_error(
                "function call does not contain two parantheses",
                "formatln",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error(
            "function requires one argument",
            "formatln",
            input_name,
            line_number,
        )
    }
}

pub fn silm_typeof(tokens: Vec<&str>, variables: &[Variable]) -> Result<Variable, String> {
    if !tokens.is_empty() {
        let argument_str = &mut tokens[0..].join(" ");

        if argument_str.starts_with('(') && argument_str.ends_with(')') {
            argument_str.remove(0);
            argument_str.pop();

            match extract_data(argument_str, variables) {
                Ok(argument_option) => {
                    if let Some(argument) = argument_option {
                        Ok(Variable {
                            identifier: "$returned$".to_string(),
                            datatype: DataType::Str,
                            value: represent_datatype(argument.datatype).to_string(),
                        })
                    } else {
                        Err("function requires one argument".to_string())
                    }
                }

                Err(error) => Err(error),
            }
        } else {
            Err("function call does not contain two parantheses".to_string())
        }
    } else {
        Err("function requires one argument".to_string())
    }
}

pub fn silm_readline(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if tokens.len() >= 2 {
        let arguments = &mut tokens[0..].join(" ");

        if arguments.starts_with('(') && arguments.ends_with(')') {
            arguments.remove(0);
            arguments.pop();

            match arguments.rsplit_once(',') {
                Some((first_argument_str, second_argument_str)) => {
                    match extract_data(first_argument_str, variables) {
                        Ok(first_argument_option) => {
                            if let Some(first_argument) = first_argument_option {
                                match extract_data(second_argument_str, variables) {
                                    Ok(second_argument_option) => {
                                        if let Some(second_argument) = second_argument_option {
                                            if second_argument.datatype == DataType::Str {
                                                print!("{}", first_argument.value);
                                                io::stdout().flush().unwrap();

                                                let mut stdin_text = String::new();
                                                match io::stdin().read_line(&mut stdin_text) {
                                                    Ok(_) => assign(
                                                        Variable {
                                                            datatype: second_argument.datatype,
                                                            identifier: second_argument.identifier,
                                                            value: stdin_text
                                                                .trim_end()
                                                                .to_string(),
                                                        },
                                                        variables,
                                                    ),

                                                    Err(_) => {
                                                        throw_error(
                                                            "invalid input",
                                                            "readline",
                                                            input_name,
                                                            line_number,
                                                        );
                                                    }
                                                }
                                            } else {
                                                throw_error(
                                                    format!(
                                                        "invalid type: '{}' must be a str",
                                                        first_argument_str
                                                    )
                                                    .as_str(),
                                                    "readline",
                                                    input_name,
                                                    line_number,
                                                );
                                            }
                                        } else {
                                            throw_error(
                                                "second argument not given",
                                                "readline",
                                                input_name,
                                                line_number,
                                            )
                                        }
                                    }

                                    Err(error) => {
                                        throw_error(&error, "readline", input_name, line_number)
                                    }
                                }
                            } else {
                                throw_error(
                                    "first argument not given",
                                    "readline",
                                    input_name,
                                    line_number,
                                )
                            }
                        }

                        Err(error) => throw_error(&error, "readline", input_name, line_number),
                    }
                }

                None => throw_error(
                    "function requires two arguments",
                    "readline",
                    input_name,
                    line_number,
                ),
            }
        } else {
            throw_error(
                "function call does not contain two parantheses",
                "readline",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error(
            "function requires at least two tokens",
            "readline",
            input_name,
            line_number,
        );
    }
}

pub fn silm_exit(tokens: Vec<&str>, input_name: String, line_number: i32) {
    if !tokens.is_empty() {
        if tokens[0] == "()" {
            exit(0);
        } else {
            throw_error("invalid function call", "exit", input_name, line_number);
        }
    } else {
        throw_error(
            "function call requires parantheses",
            "exit",
            input_name,
            line_number,
        )
    }
}
