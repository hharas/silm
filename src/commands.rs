use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

use crate::{
    helper::{assign, extract_data, get_variable, throw_error},
    interpreter::{interpret, DataType, Variable},
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
                "command call does not contain two parantheses",
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
                            "command requires one argument",
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
                "command call does not contain two parantheses",
                "formatln",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error(
            "command requires one argument",
            "formatln",
            input_name,
            line_number,
        )
    }
}

pub fn silm_readln(
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
                                                            "readln",
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
                                                    "readln",
                                                    input_name,
                                                    line_number,
                                                );
                                            }
                                        } else {
                                            throw_error(
                                                "second argument not given",
                                                "readln",
                                                input_name,
                                                line_number,
                                            )
                                        }
                                    }

                                    Err(error) => {
                                        throw_error(&error, "readln", input_name, line_number)
                                    }
                                }
                            } else {
                                throw_error(
                                    "first argument not given",
                                    "readln",
                                    input_name,
                                    line_number,
                                )
                            }
                        }

                        Err(error) => throw_error(&error, "readln", input_name, line_number),
                    }
                }

                None => throw_error(
                    "command requires two arguments",
                    "readln",
                    input_name,
                    line_number,
                ),
            }
        } else {
            throw_error(
                "command call does not contain two parantheses",
                "readln",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error(
            "command requires at least two tokens",
            "readln",
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
            throw_error("invalid command call", "exit", input_name, line_number);
        }
    } else {
        throw_error(
            "command call requires two parantheses",
            "exit",
            input_name,
            line_number,
        )
    }
}

pub fn silm_block(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if !tokens.is_empty() {
        let name = tokens[0];

        if tokens[1] == "()" {
            let block_code = &tokens[2..].join(" ");

            let sections: Vec<&str> = block_code.split("\\;").collect();

            assign(
                Variable {
                    identifier: name.to_string(),
                    datatype: DataType::Block,
                    value: sections.join("\n"),
                },
                variables,
            )
        } else {
            throw_error(
                "block must have two parantheses",
                "block",
                input_name,
                line_number,
            )
        }
    } else {
        throw_error(
            "block can not be nameless",
            "block",
            input_name,
            line_number,
        )
    }
}

pub fn silm_interpret(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if !tokens.is_empty() {
        let argument = &mut tokens[0..].join(" ");

        if argument.starts_with('(') && argument.ends_with(')') {
            argument.remove(0);
            argument.pop();

            match extract_data(argument, variables) {
                Ok(variable_option) => {
                    if let Some(variable) = variable_option {
                        if variable.datatype == DataType::Str {
                            let filename = variable.value;
                            match read_to_string(filename.clone()) {
                                Ok(code) => {
                                    let mut current_line = 0;
                                    for line in code.lines() {
                                        current_line += 1;
                                        interpret(
                                            line.to_string(),
                                            filename.clone(),
                                            current_line,
                                            variables,
                                        );
                                    }
                                }

                                Err(error) => {
                                    throw_error(
                                        &format!("{}", error),
                                        "interpret",
                                        input_name,
                                        line_number,
                                    );
                                }
                            }
                        } else {
                            throw_error(
                                "argument must be a str",
                                "interpret",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        println!();
                    }
                }
                Err(error) => {
                    throw_error(&error, "interpret", input_name, line_number);
                }
            }
        } else {
            throw_error(
                "command call does not contain two parantheses",
                "interpret",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error("no argument provided", "interpret", input_name, line_number);
    }
}

pub fn silm_eval(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if !tokens.is_empty() {
        let argument = &mut tokens[0..].join(" ");

        if argument.starts_with('(') && argument.ends_with(')') {
            argument.remove(0);
            argument.pop();

            match extract_data(argument, variables) {
                Ok(variable_option) => {
                    if let Some(variable) = variable_option {
                        if variable.datatype == DataType::Str {
                            interpret(variable.value, input_name, line_number, variables);
                        } else {
                            throw_error("argument must be a str", "eval", input_name, line_number);
                        }
                    } else {
                        throw_error(
                            "command requires one argument",
                            "eval",
                            input_name,
                            line_number,
                        );
                    }
                }
                Err(error) => {
                    throw_error(&error, "eval", input_name, line_number);
                }
            }
        } else {
            throw_error(
                "command call does not contain two parantheses",
                "eval",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error("no argument provided", "eval", input_name, line_number);
    }
}

pub fn silm_import(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if !tokens.is_empty() {
        let argument = &mut tokens[0..].join(" ");

        if argument.starts_with('(') && argument.ends_with(')') {
            argument.remove(0);
            argument.pop();

            match extract_data(argument, variables) {
                Ok(variable_option) => {
                    if let Some(variable) = variable_option {
                        if variable.datatype == DataType::Str {
                            let filename = variable.value;
                            match read_to_string(filename.clone()) {
                                Ok(code) => {
                                    let mut current_line = 0;
                                    for line in code.lines() {
                                        current_line += 1;
                                        let code_tokens: Vec<&str> =
                                            line.split_whitespace().collect();
                                        if !code_tokens.is_empty() {
                                            if code_tokens[0] == "let" {
                                                silm_let(
                                                    code_tokens[1..].to_vec(),
                                                    filename.clone(),
                                                    current_line,
                                                    variables,
                                                )
                                            } else if code_tokens[0] == "block" {
                                                silm_block(
                                                    code_tokens[1..].to_vec(),
                                                    filename.clone(),
                                                    current_line,
                                                    variables,
                                                )
                                            }
                                        }
                                    }
                                }

                                Err(error) => {
                                    throw_error(
                                        &format!("{}", error),
                                        "import",
                                        input_name,
                                        line_number,
                                    );
                                }
                            }
                        } else {
                            throw_error(
                                "argument must be a str",
                                "import",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        throw_error(
                            "command requires one argument",
                            "current_function",
                            input_name,
                            line_number,
                        );
                    }
                }
                Err(error) => {
                    throw_error(&error, "import", input_name, line_number);
                }
            }
        } else {
            throw_error(
                "command call does not contain two parantheses",
                "import",
                input_name,
                line_number,
            );
        }
    } else {
        throw_error("no argument provided", "import", input_name, line_number);
    }
}

pub fn silm_if(
    tokens: Vec<&str>,
    input_name: String,
    line_number: i32,
    variables: &mut Vec<Variable>,
) {
    if !tokens.is_empty() {
        let arguments = &tokens[0..].join(" ");
        match arguments.split_once(" :: ") {
            Some((condition_str, then_str)) => {
                let mut condition = String::from(condition_str);

                if condition.starts_with('(') && condition.ends_with(')') {
                    condition.remove(0);
                    condition.pop();

                    match extract_data(&condition, variables) {
                        Ok(result_option) => {
                            if let Some(result) = result_option {
                                if result.datatype == DataType::Bool {
                                    if result.value == "true" {
                                        let then: Vec<&str> = then_str.split("\\;").collect();

                                        for line in then {
                                            interpret(
                                                line.into(),
                                                input_name.clone(),
                                                line_number,
                                                variables,
                                            );
                                        }
                                    }
                                } else {
                                    throw_error(
                                        "condition must be of type bool",
                                        "if",
                                        input_name,
                                        line_number,
                                    );
                                }
                            }
                        }

                        Err(error) => {
                            throw_error(&error, "if", input_name, line_number);
                        }
                    }
                } else {
                    throw_error(
                        "command condition must contain two parantheses",
                        "if",
                        input_name,
                        line_number,
                    );
                }
            }
            None => {
                throw_error(
                    "no condition-bound code provided",
                    "if",
                    input_name,
                    line_number,
                );
            }
        }
    } else {
        throw_error("no condition provided", "if", input_name, line_number);
    }
}
