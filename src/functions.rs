use crate::{
    helper::{extract_data, get_variable, represent_datatype},
    interpreter::{DataType, Variable},
};

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

#[test]
fn test_silm_typeof() {
    assert_eq!(silm_typeof(vec!["('A')"], &[]).unwrap().value, "char");
    assert!(silm_typeof(vec!["(a)"], &[]).is_err());
}

pub fn silm_nameof(tokens: Vec<&str>, variables: &[Variable]) -> Result<Variable, String> {
    if !tokens.is_empty() {
        let argument_str = &mut tokens[0..].join(" ");

        if argument_str.starts_with('(') && argument_str.ends_with(')') {
            argument_str.remove(0);
            argument_str.pop();

            match extract_data(argument_str, variables) {
                Ok(argument_option) => {
                    if let Some(argument) = argument_option {
                        Ok(Variable {
                            identifier: "$returned$".into(),
                            datatype: DataType::Str,
                            value: argument.identifier,
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

#[test]
fn test_silm_nameof() {
    assert_eq!(
        silm_nameof(vec!["(\"Test\")"], &[]).unwrap().value,
        "$uninitialised$"
    );
    assert_eq!(
        silm_nameof(
            vec!["(b)"],
            &[Variable {
                identifier: "b".into(),
                datatype: DataType::Bool,
                value: "false".into()
            }]
        )
        .unwrap()
        .value,
        "b"
    );
    assert!(silm_nameof(vec!["(a)"], &[]).is_err());
}

pub fn silm_eq(tokens: Vec<&str>, variables: &[Variable]) -> Result<Variable, String> {
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
                                            if first_argument.datatype == second_argument.datatype {
                                                if first_argument.value == second_argument.value {
                                                    Ok(Variable {
                                                        datatype: DataType::Bool,
                                                        identifier: "$returned$".into(),
                                                        value: "true".into(),
                                                    })
                                                } else {
                                                    Ok(Variable {
                                                        datatype: DataType::Bool,
                                                        identifier: "$returned$".into(),
                                                        value: "false".into(),
                                                    })
                                                }
                                            } else {
                                                Ok(Variable {
                                                    datatype: DataType::Bool,
                                                    identifier: "$returned$".into(),
                                                    value: "false".into(),
                                                })
                                            }
                                        } else {
                                            Err("second argument not given".into())
                                        }
                                    }

                                    Err(error) => Err(error),
                                }
                            } else {
                                Err("first argument not given".into())
                            }
                        }

                        Err(error) => Err(error),
                    }
                }

                None => Err("function requires two arguments".into()),
            }
        } else {
            Err("function call does not contain two parantheses".into())
        }
    } else {
        Err("function requires at least two tokens".into())
    }
}

#[test]
fn test_silm_eq() {
    assert_eq!(silm_eq(vec!["(1,", "1)"], &[]).unwrap().value, "true");
    assert_eq!(
        silm_eq(vec!["(\"same", "phrase\",", "\"same", "phrase\")"], &[])
            .unwrap()
            .value,
        "true"
    );
    assert_eq!(
        silm_eq(vec!["(false,", "true)"], &[]).unwrap().value,
        "false"
    );
}

pub fn silm_ne(tokens: Vec<&str>, variables: &[Variable]) -> Result<Variable, String> {
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
                                            if first_argument.datatype == second_argument.datatype {
                                                if first_argument.value == second_argument.value {
                                                    Ok(Variable {
                                                        datatype: DataType::Bool,
                                                        identifier: "$returned$".into(),
                                                        value: "false".into(),
                                                    })
                                                } else {
                                                    Ok(Variable {
                                                        datatype: DataType::Bool,
                                                        identifier: "$returned$".into(),
                                                        value: "true".into(),
                                                    })
                                                }
                                            } else {
                                                Ok(Variable {
                                                    datatype: DataType::Bool,
                                                    identifier: "$returned$".into(),
                                                    value: "true".into(),
                                                })
                                            }
                                        } else {
                                            Err("second argument not given".into())
                                        }
                                    }

                                    Err(error) => Err(error),
                                }
                            } else {
                                Err("first argument not given".into())
                            }
                        }

                        Err(error) => Err(error),
                    }
                }

                None => Err("function requires two arguments".into()),
            }
        } else {
            Err("function call does not contain two parantheses".into())
        }
    } else {
        Err("function requires at least two tokens".into())
    }
}

#[test]
fn test_silm_ne() {
    assert_eq!(silm_ne(vec!["(1,", "1)"], &[]).unwrap().value, "false");
    assert_eq!(
        silm_ne(vec!["(\"same", "phrase\",", "\"same", "phrase\")"], &[])
            .unwrap()
            .value,
        "false"
    );
    assert_eq!(
        silm_ne(vec!["(false,", "true)"], &[]).unwrap().value,
        "true"
    );
}

pub fn silm_format(tokens: Vec<&str>, variables: &[Variable]) -> Result<Variable, String> {
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

                        Ok(Variable {
                            identifier: "$returned$".into(),
                            datatype: DataType::Str,
                            value: result,
                        })
                    } else {
                        Err("command requires one argument".into())
                    }
                }

                Err(error) => Err(error),
            }
        } else {
            Err("command call does not contain two parantheses".into())
        }
    } else {
        Err("command requires one argument".into())
    }
}
