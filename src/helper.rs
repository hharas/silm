use crate::{
    functions::silm_typeof,
    interpreter::{DataType, Variable},
};

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
        } else if data.chars().next().unwrap().is_ascii_digit() || data.starts_with('-') {
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
                match data_tokens[0].parse::<f64>() {
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
        } else if let Some(result) = call_function(
            data_tokens[0],
            data_tokens[1..].to_vec(),
            &mut variables.to_vec(),
        ) {
            match result {
                Ok(returned_value) => Ok(Some(returned_value)),

                Err(error) => Err(error),
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

#[test]
fn test_extract_data() {
    let mut variables: Vec<Variable> = Vec::new();

    variables.push(Variable {
        identifier: "x".to_string(),
        datatype: DataType::Int,
        value: "10".to_string(),
    });

    assert_eq!(
        extract_data("10", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Int,
            value: "10".to_string()
        }))
    );

    assert_eq!(
        extract_data("1.0", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Float,
            value: "1".to_string()
        }))
    );

    assert_eq!(
        extract_data("\"alhamdulillah\"", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Str,
            value: "alhamdulillah".to_string()
        }))
    );

    assert_eq!(
        extract_data("'W'", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Char,
            value: "W".to_string()
        }))
    );

    assert_eq!(
        extract_data("true", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Bool,
            value: "true".to_string()
        }))
    );

    assert_eq!(
        extract_data("10 + x", &variables),
        Ok(Some(Variable {
            identifier: "$uninitialised$".to_string(),
            datatype: DataType::Float,
            value: "20".to_string()
        }))
    );
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

#[test]
fn test_assign() {
    let mut variables: Vec<Variable> = Vec::new();

    variables.push(Variable {
        identifier: "already_there".to_string(),
        datatype: DataType::Bool,
        value: "false".to_string(),
    });

    let new_variable = Variable {
        identifier: "new_var".to_string(),
        datatype: DataType::Int,
        value: "10".to_string(),
    };

    assign(new_variable, &mut variables);

    assert!(variables
        .iter()
        .any(|variable| variable.identifier == "new_var".to_string()
            && variable.datatype == DataType::Int
            && variable.value == "10".to_string()));

    assign(
        Variable {
            identifier: "already_there".to_string(),
            datatype: DataType::Str,
            value: "changed now!".to_string(),
        },
        &mut variables,
    );

    assert!(variables.iter().any(
        |variable| variable.identifier == "already_there".to_string()
            && variable.datatype == DataType::Str
            && variable.value == "changed now!".to_string()
    ));
}

pub fn represent_datatype(datatype: DataType) -> &'static str {
    match datatype {
        DataType::Bool => "bool",

        DataType::Float => "float",

        DataType::Int => "int",

        DataType::Str => "str",

        DataType::Char => "char",

        DataType::Block => "block",
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

#[test]
fn test_get_variable() {
    let mut variables: Vec<Variable> = Vec::new();

    let new_variable = Variable {
        identifier: "name".to_string(),
        datatype: DataType::Str,
        value: "hasan".to_string(),
    };

    variables.push(new_variable.clone());

    assert_eq!(get_variable("name", &variables), Some(new_variable));
    assert_eq!(get_variable("$uninitialised$", &variables), None);
}

pub fn throw_error(message: &str, current_function: &str, input_name: String, line_number: i32) {
    println!(
        "{}:{}: {}: {}",
        input_name, line_number, current_function, message
    );
}

// I'm planning to make this function also return a DataType
// an Int by default, or a Float if the result is so
pub fn shunting_yard(tokens: Vec<&str>, variables: &[Variable]) -> Result<f64, String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut output_stack: Vec<f64> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let operators = "+-*/^%";

    for token in tokens {
        if let Ok(number) = token.parse::<f64>() {
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
                        if let Ok(number) = variable.value.parse::<f64>() {
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

#[test]
fn test_shunting_yard() {
    let mut variables: Vec<Variable> = Vec::new();
    variables.push(Variable {
        identifier: "x".to_string(),
        datatype: DataType::Int,
        value: "256".to_string(),
    });

    let tokens: Vec<&str> = "x + 1 + 2 - ( 3 * 4 ) / 5 ^ 6 % 7"
        .split_whitespace()
        .collect();

    assert_eq!(shunting_yard(tokens, &variables), Ok(258.92));
}

fn call_function(
    name: &str,
    tokens: Vec<&str>,
    variables: &mut [Variable],
) -> Option<Result<Variable, String>> {
    match name {
        "typeof" => match silm_typeof(tokens, variables) {
            Ok(returned_value) => Some(Ok(returned_value)),

            Err(error) => Some(Err(format!("{}: {}", name, error))),
        },

        _ => None,
    }
}
