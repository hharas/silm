use std::{
    env::args,
    fs::read_to_string,
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

#[derive(Clone, Copy, PartialEq)]
enum DataType {
    Int,
    Float,
    Str,
    Char,
    Bool,
}

struct Variable {
    datatype: DataType,
    identifier: String,
    value: String,
}

fn main() {
    let args: Vec<String> = args().collect();
    let mut variables: Vec<Variable> = Vec::new();

    if args.len() < 3 {
        if args.len() == 1 {
            println!("Silm 0.1.0 Interpreter");
            println!("Enter \"exit ();\" to quit");
            loop {
                print!("s>>> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();

                let trimmed_input = input.trim();

                interpret(
                    trimmed_input.to_string(),
                    "stdin".to_string(),
                    0,
                    &mut variables,
                );
            }
        } else {
            match args[1].as_str() {
                "--version" => {
                    println!("silm interpreter version 0.1.0");
                }

                _ => {
                    let filepath = PathBuf::from(&args[1]);
                    match read_to_string(&filepath) {
                        Ok(source) => {
                            let mut line_number = 0;
                            for line in source.lines() {
                                line_number += 1;

                                interpret(
                                    line.to_string(),
                                    filepath.file_name().unwrap().to_str().unwrap().to_string(),
                                    line_number,
                                    &mut variables,
                                );
                            }
                        }

                        Err(error) => {
                            println!("ERROR[0]: {error}");
                            exit(1);
                        }
                    }
                }
            }
        }
    }
}

fn interpret(line: String, input_name: String, line_number: i32, variables: &mut Vec<Variable>) {
    let tokens: Vec<&str> = line.trim().split(' ').collect();
    if !tokens.is_empty() {
        match tokens[0] {
            "int" => {
                // int x = 10;
                if tokens.len() == 4 {
                    if tokens[2] == "=" {
                        let datatype = DataType::Int;
                        let identifier = tokens[1].to_string();

                        let value = tokens[3].trim_end_matches(';').to_string();

                        match value.parse::<i32>() {
                            Ok(_) => assign(
                                Variable {
                                    datatype,
                                    identifier,
                                    value,
                                },
                                variables,
                            ),

                            Err(_) => {
                                throw_error(
                                    format!("'{}' is not an integer", value).as_str(),
                                    "int",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "assignment operator (=) not found",
                            "int",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 4)",
                        "int",
                        input_name,
                        line_number,
                    );
                }
            }

            "float" => {
                // float y = 20.5;
                if tokens.len() == 4 {
                    if tokens[2] == "=" {
                        let datatype = DataType::Float;
                        let identifier = tokens[1].to_string();
                        let value = tokens[3].trim_end_matches(';').to_string();

                        match value.parse::<f32>() {
                            Ok(_) => {
                                if value.contains('.') {
                                    assign(
                                        Variable {
                                            datatype,
                                            identifier,
                                            value,
                                        },
                                        variables,
                                    )
                                } else {
                                    throw_error(
                                        format!(
                                            "'{}' does not contain a floating point",
                                            identifier
                                        )
                                        .as_str(),
                                        "float",
                                        input_name,
                                        line_number,
                                    );
                                }
                            }

                            Err(_) => {
                                throw_error(
                                    format!("'{}' is not a float", value).as_str(),
                                    "float",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "assignment operator (=) not found",
                            "float",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 4)",
                        "float",
                        input_name,
                        line_number,
                    );
                }
            }

            "str" => {
                // str message = "Salam brother";
                if tokens.len() >= 4 {
                    if tokens[2] == "=" {
                        let datatype = DataType::Str;
                        let identifier = tokens[1].to_string();

                        let supposed_value = &tokens[3..].join(" ");

                        let value = supposed_value
                            .trim_start_matches('"')
                            .trim_end_matches("\";")
                            .to_string();

                        assign(
                            Variable {
                                datatype,
                                identifier,
                                value,
                            },
                            variables,
                        )
                    } else {
                        throw_error(
                            "assignment operator (=) not found",
                            "str",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!>= 4)",
                        "str",
                        input_name,
                        line_number,
                    );
                }
            }

            "char" => {
                // char grade = 'A';
                if tokens.len() == 4 {
                    if tokens[2] == "=" {
                        let datatype = DataType::Char;
                        let identifier = tokens[1].to_string();

                        let value = tokens[3]
                            .trim_start_matches('\'')
                            .trim_end_matches("\';")
                            .to_string();

                        match value.parse::<char>() {
                            Ok(_) => assign(
                                Variable {
                                    datatype,
                                    identifier,
                                    value,
                                },
                                variables,
                            ),

                            Err(_) => {
                                throw_error(
                                    format!("'{}' is not a char", value).as_str(),
                                    "char",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "assignment operator (=) not found",
                            "char",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 4)",
                        "char",
                        input_name,
                        line_number,
                    );
                }
            }

            "bool" => {
                // bool realest = true;
                if tokens.len() == 4 {
                    if tokens[2] == "=" {
                        let datatype = DataType::Bool;
                        let identifier = tokens[1].to_string();
                        let value = tokens[3].trim_end_matches(';').to_string();

                        match value.parse::<bool>() {
                            Ok(_) => assign(
                                Variable {
                                    datatype,
                                    identifier,
                                    value,
                                },
                                variables,
                            ),

                            Err(_) => {
                                throw_error(
                                    format!("'{}' is not a boolean", value).as_str(),
                                    "bool",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "assignment operator (=) not found",
                            "bool",
                            input_name,
                            line_number,
                        );
                    }
                } else {
                    throw_error(
                        "improper amount of tokens (!= 4)",
                        "bool",
                        input_name,
                        line_number,
                    );
                }
            }

            "println" => {
                // println (message);
                if tokens.len() == 2 {
                    let argument = tokens[1].trim_start_matches('(').trim_end_matches(");");
                    if !argument.is_empty() {
                        match variables
                            .iter()
                            .find(|variable| variable.identifier == argument)
                        {
                            Some(variable) => {
                                println!("{}", variable.value);
                            }

                            None => {
                                throw_error(
                                    format!("variable '{}' unrecognised", argument).as_str(),
                                    "println",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        println!();
                    }
                } else {
                    throw_error("no argument provided", "println", input_name, line_number);
                }
            }

            "formatln" => {
                // formatln ("Salam {name}");
                if tokens.len() >= 2 {
                    let argument_str = &tokens[1..].join(" ");
                    let argument_str = argument_str
                        .trim_start_matches("(\"")
                        .trim_end_matches("\");");

                    let mut result = String::from(argument_str);

                    // Can't loop over `variables` twice or else the borrow checker will go bananas
                    for index in 0..variables.len() {
                        let variable = &variables[index];
                        let placeholder = format!("{{{}}}", variable.identifier);

                        if let Some(found_variable) = find_var(&variable.identifier, variables) {
                            let value = found_variable.value;
                            result = result.replace(&placeholder, &value);
                        }
                    }

                    println!("{}", result);
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
                    let argument = tokens[1].trim_start_matches('(').trim_end_matches(");");
                    if !argument.is_empty() {
                        match variables
                            .iter()
                            .find(|variable| variable.identifier == argument)
                        {
                            Some(variable) => {
                                println!("{}", represent_datatype(variable.datatype));
                            }

                            None => {
                                throw_error(
                                    format!("variable '{}' unrecognised", argument).as_str(),
                                    "typeof",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "improper amount of tokens (!= 2)",
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
                // readline (message, input);
                if tokens.len() == 3 {
                    let arguments = &tokens[1..];

                    if arguments.len() == 2 {
                        let first_argument_identifier =
                            arguments[0].trim_start_matches('(').trim_end_matches(',');
                        match find_var(first_argument_identifier, variables) {
                            Some(first_argument) => {
                                if first_argument.datatype == DataType::Str {
                                    let second_argument_identifier =
                                        arguments[1].trim_end_matches(");");
                                    match find_var(second_argument_identifier, variables) {
                                        Some(second_argument) => {
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
                                                        second_argument_identifier
                                                    )
                                                    .as_str(),
                                                    "readline",
                                                    input_name,
                                                    line_number,
                                                );
                                            }
                                        }

                                        None => {
                                            throw_error(
                                                format!(
                                                    "variable '{}' unrecognised",
                                                    first_argument_identifier
                                                )
                                                .as_str(),
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
                                            first_argument_identifier
                                        )
                                        .as_str(),
                                        "readline",
                                        input_name,
                                        line_number,
                                    );
                                }
                            }

                            None => {
                                throw_error(
                                    format!(
                                        "variable '{}' unrecognised",
                                        first_argument_identifier
                                    )
                                    .as_str(),
                                    "readline",
                                    input_name,
                                    line_number,
                                );
                            }
                        }
                    } else {
                        throw_error(
                            "invalid amount of arguments (!= 2)",
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

            "" => {}

            "#" => {}

            _ => {
                println!(
                    "{}:{}: unknown function: {}",
                    input_name,
                    line_number,
                    tokens.join(" ")
                );
            }
        }
    }
}

fn represent_datatype(datatype: DataType) -> &'static str {
    match datatype {
        DataType::Bool => "bool",

        DataType::Float => "float",

        DataType::Int => "int",

        DataType::Str => "str",

        DataType::Char => "char",
    }
}

fn assign(variable: Variable, variables: &mut Vec<Variable>) {
    match variables
        .iter_mut()
        .find(|var| var.identifier == variable.identifier)
    {
        Some(existing_variable) => {
            existing_variable.value = variable.value;
        }

        None => {
            variables.push(variable);
        }
    }
}

fn find_var(identifier: &str, variables: &[Variable]) -> Option<Variable> {
    variables
        .iter()
        .find(|var| var.identifier == identifier)
        .map(|found_variable| Variable {
            datatype: found_variable.datatype,
            identifier: found_variable.identifier.clone(),
            value: found_variable.value.clone(),
        })
}

fn throw_error(message: &str, current_function: &str, input_name: String, line_number: i32) {
    println!(
        "{}:{}: {}: {}",
        input_name, line_number, current_function, message
    );
}
