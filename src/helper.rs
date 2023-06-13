use crate::interpreter::{DataType, Variable};

// This function goes hard, feel free to copy & paste
pub fn extract_data(data: &str, variables: &[Variable]) -> Result<Option<Variable>, String> {
    let datatype;
    let value;

    let data = data.trim();
    let data_tokens: Vec<&str> = data.split_whitespace().collect();

    if !data.is_empty() {
        if data.starts_with('\"') && data.ends_with('\"') {
            datatype = DataType::Str;
            value = data
                .trim_start_matches('\"')
                .trim_end_matches('\"')
                .to_string();

            Ok(Some(Variable {
                identifier: "$uninitialised$".to_string(),
                datatype,
                value,
            }))
        } else if data.starts_with('\'') && data.ends_with('\'') {
            datatype = DataType::Char;
            value = data
                .trim_start_matches('\'')
                .trim_end_matches('\'')
                .to_string();

            if !value.is_empty() {
                match value.parse::<char>() {
                    Ok(_) => Ok(Some(Variable {
                        identifier: "$uninitialised$".to_string(),
                        datatype,
                        value,
                    })),

                    Err(_) => Err("invalid char assignment".to_string()),
                }
            } else {
                Ok(Some(Variable {
                    identifier: "$uninitialised$".to_string(),
                    datatype,
                    value: "".to_string(),
                }))
            }
        } else if data == "true" || data == "false" {
            datatype = DataType::Bool;
            value = data.to_string();

            Ok(Some(Variable {
                identifier: "$uninitialised$".to_string(),
                datatype,
                value,
            }))
        } else if data.chars().next().unwrap().is_ascii_digit() {
            if data_tokens.len() > 1 {
                match shunting_yard(data_tokens, variables) {
                    Ok(result) => Ok(Some(Variable {
                        identifier: "$uninitialised$".to_string(),
                        datatype: DataType::Float,
                        value: result.to_string(),
                    })),

                    Err(error) => Err(format!("shunting yard algorithm: {}", error)),
                }
            } else if data_tokens[0].contains('.') {
                match data_tokens[0].parse::<f32>() {
                    Ok(value) => Ok(Some(Variable {
                        datatype: DataType::Float,
                        identifier: "$uninitialised$".to_string(),
                        value: value.to_string(),
                    })),

                    Err(_) => Err("invalid float assignment".to_string()),
                }
            } else {
                match data_tokens[0].parse::<i32>() {
                    Ok(value) => Ok(Some(Variable {
                        datatype: DataType::Int,
                        identifier: "$uninitialised$".to_string(),
                        value: value.to_string(),
                    })),

                    Err(_) => Err("invalid int assignment".to_string()),
                }
            }
        } else if data_tokens.len() == 1 {
            match get_variable(data, variables) {
                Some(variable) => Ok(Some(variable)),

                None => Err(format!("variable '{}' unrecognised", data)),
            }
        } else {
            match shunting_yard(data_tokens, variables) {
                Ok(result) => Ok(Some(Variable {
                    identifier: "$uninitialised$".to_string(),
                    datatype: DataType::Float,
                    value: result.to_string(),
                })),

                Err(error) => Err(format!("shunting yard algorithm: {}", error)),
            }
        }
    } else {
        Ok(None)
    }
}

pub fn assign(variable: Variable, variables: &mut Vec<Variable>) {
    match variables
        .iter_mut()
        .find(|var| var.identifier == variable.identifier)
    {
        Some(existing_variable) => {
            existing_variable.value = variable.value;
            existing_variable.datatype = variable.datatype;
        }

        None => {
            variables.push(variable);
        }
    }
}

pub fn represent_datatype(datatype: DataType) -> &'static str {
    match datatype {
        DataType::Bool => "bool",

        DataType::Float => "float",

        DataType::Int => "int",

        DataType::Str => "str",

        DataType::Char => "char",
    }
}

pub fn get_variable(identifier: &str, variables: &[Variable]) -> Option<Variable> {
    variables
        .iter()
        .find(|var| var.identifier == identifier)
        .map(|found_variable| Variable {
            datatype: found_variable.datatype,
            identifier: found_variable.identifier.clone(),
            value: found_variable.value.clone(),
        })
}

pub fn throw_error(message: &str, current_function: &str, input_name: String, line_number: i32) {
    println!(
        "{}:{}: {}: {}",
        input_name, line_number, current_function, message
    );
}

// I'm planning to make this function also return a DataType
// an Int by default, or a Float if the result is so
pub fn shunting_yard(tokens: Vec<&str>, variables: &[Variable]) -> Result<f32, String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut output_stack: Vec<f32> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let operators = "+-*/^%";

    for token in tokens {
        if let Ok(number) = token.parse::<f32>() {
            output_queue.push(number.to_string());
            output_stack.push(number);
        } else if operators.contains(token) {
            while let Some(op) = operator_stack.clone().last() {
                if operators.contains(op)
                    && ((token == "+" || token == "-") && (op == "*" || op == "/"))
                {
                    output_queue.push(operator_stack.pop().unwrap());
                    let b = output_stack.pop().unwrap();
                    let a = output_stack.pop().unwrap();
                    let result = match op.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        "^" => (a * a) * b,
                        "%" => a % b,
                        _ => return Err(format!("invalid operator: {}", op)),
                    };
                    output_stack.push(result);
                } else {
                    break;
                }
            }
            operator_stack.push(token.to_string());
        } else if token == "(" {
            operator_stack.push(token.to_string());
        } else if token == ")" {
            while let Some(op) = operator_stack.clone().last() {
                if op == "(" {
                    operator_stack.pop();
                    break;
                } else {
                    output_queue.push(operator_stack.pop().unwrap());
                    let b = output_stack.pop().unwrap();
                    let a = output_stack.pop().unwrap();
                    let result = match op.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        "^" => (a * a) * b,
                        "%" => a % b,
                        _ => return Err(format!("invalid operator: {}", op)),
                    };
                    output_stack.push(result);
                }
            }
        } else if variables
            .iter()
            .any(|variable| variable.identifier == token)
        {
            match get_variable(token, variables) {
                Some(variable) => {
                    if variable.datatype == DataType::Int || variable.datatype == DataType::Float {
                        if let Ok(number) = variable.value.parse::<f32>() {
                            output_queue.push(number.to_string());
                            output_stack.push(number);
                        } else {
                            return Err(format!(
                                "variable '{}' can not be parsed into a float",
                                token
                            ));
                        }
                    } else {
                        return Err(format!(
                            "variable '{}' must be either an int or a float",
                            token
                        ));
                    }
                }

                None => return Err(format!("invalid variable: {}", token)),
            }
        } else {
            return Err(format!("invalid token: {}", token));
        }
    }

    while let Some(op) = operator_stack.pop() {
        if op == "(" {
            return Err("mismatched parentheses".to_string());
        }
        output_queue.push(op.clone());
        let b = output_stack.pop().unwrap();
        let a = output_stack.pop().unwrap();
        let result = match op.as_str() {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "^" => (a * a) * b,
            "%" => a % b,
            _ => return Err(format!("invalid operator: {}", op)),
        };
        output_stack.push(result);
    }

    if output_stack.len() != 1 {
        return Err("invalid expression".to_string());
    }

    Ok(output_stack[0])
}
