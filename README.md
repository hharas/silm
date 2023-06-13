# Silm

Experimental, line-by-line-interpreted programming language written in Rust.

## Why?

I wondered how hard will an interpreter be to write, but it turns out it's pretty easy.

Seriously though: I thought of importing silm (or probably some of its code) to [Aati](https://github.com/hharas/aati) in order to add support for PKGINSTALL files (coming along the way).

## What does Silm mean?

Silm (سِلم) is an Arabic word that means "Peace".

## How does it work?

At the essence of it, Silm is merely an `interpret()` function that accepts four arguments:
1. a line of code,
2. filename or input name (e.g. `main.slm` or `<stdin>`),
3. current line number in the file (e.g. `0` for `<stdin>`),
4. and a mutable reference to a vector of `Variable` structs.

Which makes it very portable and easy to implement, and also easy to develop further.

## What is what?

### `src/main.rs`

This is the CLI front-end of Silm, or specifically the `src/interpreter.rs` module. It can run on interactive mode, interpret a file by going through it line by line, execute commands from the command line, etc.

### `src/interpreter.rs`

Now that's the whole wrapper of the language, it contains the `interpret()` function which is ran on files line-by-line or through an interactive mode.

### `src/functions.rs`

That's the Standard Library of Silm, since it contains the functions `interpreter.rs` will use and execute based on user input. It contains functions like `println`, `let`, `readline` and more.

### `src/helpelr.rs`

These are helper functions that are needed for the functions in the standard library to work. It contains some algorithms that make up the essence of the language.

#### `src/version.rs`

Just a file that reminds the Silm what version it is at right now.

## How does the code work?

`main.slm`:
```bash
# We can initialise variables like this:
let x = 10;
let y = 1.5;
let word = "real";
let silm_is_a = 'W';
let real = true;

# Variables' values and types are also mutable by default

# Function calls look a little weird but they still work
# Most of them are I/O functions like `println`
# which prints out a variable's value
println (x);

# or it takes nothing and prints an empty newline
println ();

# We also got `formatln` which receives a string value with placeholders for variables
formatln ("Silm is {word}");

# and `typeof` which receives a variable and prints out its datatype
typeof (y);

# and we have `readline`! the most complicated function of them all
# it receives two strings, it prints out the first one to the user,
# it receives user input from stdin, then it stores it in the second string
let name = "";
readline ("What's your name? ", name);

formatln ("Nice to meet you, {name}!");

# at this point the program will exit, but you can also explicltly exit using:
exit ();
# this is helpful when you're using the interactive mode
```

You can run that code using:
```bash
silm main.slm
```

or you can enter the interactive mode and write everything yourself:
```bash
silm
```

Silm has pretty nice error handling too (besides the `formatln` function, as it returns the same string if a variable is not found due to its poor implementation).

## Contribution

This is a small fun project for me but if you have any ideas for improving it then feel free to make a pull request! I'm interested in seeing Silm get bigger and bigger.

## License

Silm is licensed under the GNU General Public License version 3.
