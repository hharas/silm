use std::{
    env::args,
    fs::read_to_string,
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

use crate::{
    interpreter::{interpret, Variable},
    version::VERSION,
};

mod helper;
mod interpreter;
mod version;

fn main() {
    let args: Vec<String> = args().collect();
    let mut variables: Vec<Variable> = Vec::new();

    if args.len() < 3 {
        if args.len() == 1 {
            println!("Silm {} Interpreter", VERSION);
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
                "--help" => {
                    println!(
                        r#"silm - experimental, line-by-line-intepreted programming language

USAGE: silm [OPTIONS]

OPTIONS:
    --help       Show this help message
    --version    Show interpreter version
    <filename>   Silm source code file

This program is free software: you can redistribute it and/or modify
it under the terms of version 3 of the GNU General Public License
as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details."#
                    );
                }

                "--version" => {
                    println!("silm interpreter version {}", VERSION);
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
    } else {
        println!("Too many arguments");
    }
}
