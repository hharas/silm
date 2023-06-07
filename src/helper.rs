use crate::interpreter::{DataType, Variable};

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
