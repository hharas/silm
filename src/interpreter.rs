use std::{
    io::{self, Write},
    process::exit,
};

use crate::helper::{assign, extract_data, get_variable, represent_datatype, throw_error};

#[derive(Clone, Copy, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Str,
    Char,
    Bool,
}

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
            "let" => {
                // let x = 10;
                if tokens.len() >= 4 {
                    if tokens[2] == "=" {
                        let identifier = tokens[1].to_string();
                        let supposed_value = &mut tokens[3..].join(" ");

                        if supposed_value.ends_with(';') {
                            supposed_value.pop();

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
                                        throw_error(
                                            "empty value given",
                                            "let",
                                            input_name,
                                            line_number,
                                        )
                                    }
                                }
                                Err(error) => throw_error(&error, "let", input_name, line_number),
                            }
                        } else {
                            throw_error(
                                "statement does not end with a semicolon",
                                "let",
                                input_name,
                                line_number,
                            );
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

            "println" => {
                // println (message);
                if tokens.len() >= 2 {
                    let argument = &mut tokens[1..].join(" ");

                    if argument.ends_with(';') {
                        argument.pop();

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
                                "function call does not contain parantheses",
                                "println",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        throw_error(
                            "function call does not end with a semicolon",
                            "println",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error("no argument provided", "println", input_name, line_number);
                }
            }

            "formatln" => {
                // formatln ("Salam {name}");
                if tokens.len() >= 2 {
                    let argument_str = &mut tokens[1..].join(" ");

                    if argument_str.ends_with(';') {
                        argument_str.pop();

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
                                            let placeholder =
                                                format!("{{{}}}", variable.identifier);

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

                                Err(error) => {
                                    throw_error(&error, "formatln", input_name, line_number)
                                }
                            }
                        } else {
                            throw_error(
                                "function call does not contain parantheses",
                                "formatln",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        throw_error(
                            "function call does not end with a semicolon",
                            "formatln",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!>= 2)",
                        "formatln",
                        input_name,
                        line_number,
                    )
                }
            }

            "typeof" => {
                // typeof (x);
                if tokens.len() == 2 {
                    let argument_str = &mut tokens[1..].join(" ");

                    if argument_str.ends_with(';') {
                        argument_str.pop();

                        if argument_str.starts_with('(') && argument_str.ends_with(')') {
                            argument_str.remove(0);
                            argument_str.pop();

                            match extract_data(argument_str, variables) {
                                Ok(argument_option) => {
                                    if let Some(argument) = argument_option {
                                        println!("{}", represent_datatype(argument.datatype));
                                    } else {
                                        throw_error(
                                            "function requires one argument",
                                            "typeof",
                                            input_name,
                                            line_number,
                                        )
                                    }
                                }

                                Err(error) => {
                                    throw_error(&error, "typeof", input_name, line_number);
                                }
                            }
                        } else {
                            throw_error(
                                "function call does not contain parantheses",
                                "typeof",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        throw_error(
                            "function call does not end with a semicolon",
                            "typeof",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error("no argument provided", "typeof", input_name, line_number);
                }
            }

            "readline" => {
                // readline ("Enter your username: ", input);
                if tokens.len() >= 3 {
                    let arguments = &mut tokens[1..].join(" ");

                    if arguments.ends_with(';') {
                        arguments.pop();

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
                                                        if let Some(second_argument) =
                                                            second_argument_option
                                                        {
                                                            if second_argument.datatype
                                                                == DataType::Str
                                                            {
                                                                print!("{}", first_argument.value);
                                                                io::stdout().flush().unwrap();

                                                                let mut stdin_text = String::new();
                                                                match io::stdin()
                                                                    .read_line(&mut stdin_text)
                                                                {
                                                                    Ok(_) => assign(
                                                                        Variable {
                                                                            datatype:
                                                                                second_argument
                                                                                    .datatype,
                                                                            identifier:
                                                                                second_argument
                                                                                    .identifier,
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

                                                    Err(error) => throw_error(
                                                        &error,
                                                        "readline",
                                                        input_name,
                                                        line_number,
                                                    ),
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

                                        Err(error) => {
                                            throw_error(&error, "readline", input_name, line_number)
                                        }
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
                                "function call does not contain parantheses",
                                "readline",
                                input_name,
                                line_number,
                            );
                        }
                    } else {
                        throw_error(
                            "function call does not end with a semicolon",
                            "readline",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 3)",
                        "readline",
                        input_name,
                        line_number,
                    );
                }
            }

            "exit" => {
                // exit ();
                if tokens.len() == 2 {
                    if tokens[1] == "();" {
                        exit(0);
                    } else {
                        throw_error("invalid function call", "exit", input_name, line_number);
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 2)",
                        "exit",
                        input_name,
                        line_number,
                    )
                }
            }

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
