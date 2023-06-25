<img align="left" style="width: 277px" src="./silm.png" width="282" />

**Silm is a line-by-line-interpreted programming language written in Rust.**

At the essence of it, Silm is merely an `interpret()` function that accepts four arguments:  
a line of code,  
a filename or input name (e.g. `main.slm` or `<stdin>`),  
current line number in the file (e.g. `1` for `<stdin>`),  
and a mutable reference to a vector of `Variable` structs.

This makes Silm's syntax very simple and makes its development easier and more fun.

---

## What does Silm mean?

Silm (سِلم) is an Arabic word that means "Serenity".

## What file is what?

### `src/main.rs`

This is the CLI front-end of Silm, or specifically the `src/interpreter.rs` module. It can run on interactive mode, interpret a file by going through it line by line, execute commands from the command line, etc.

### `src/interpreter.rs`

Now that's the whole wrapper of the language, it contains the `interpret()` function which is ran on files line-by-line or through an interactive mode.

### `src/functions.rs`

That's the Standard Library of Silm, since it contains the functions `interpreter.rs` will use and execute based on user input. It contains functions like `println`, `let`, `readln` and more.

### `src/helper.rs`

These are helper functions that are needed for the functions in the standard library to work. It contains some algorithms that make up the essence of the language.

### `src/version.rs`

Just a file that reminds Silm what version it is right now.

## How does the code work?

`main.slm`:
```bash
# We can initialise variables like this:
let x = 10
let y = 1.5
let word = "real"
let silm_is_a = 'W'
let real = true

# Variables' values and types are also mutable by default
# When you want to assign a variable to another variable like this:
let z = y

# Silm is merely gonna copy y's datatype and value to z and not make a reference to it
# Which means when z changes, y isn't gonna change with it accordingly

# Function calls look a little weird but they still work
# Most of them are I/O functions like `println`
# which prints out a variable's value
println (x)

# or it takes nothing and prints an empty newline
println ()

# We also got `formatln` which receives a string value with placeholders for variables
formatln ("Silm is {word}")

# There's also `typeof`, which receives a variable and returns a str containing the variable's datatype
# functions like `typeof` are called inline functions, they're functions that:
# (1) return variables
# (2) can be called inside other functions, like how I'm calling `typeof` inside `println`:
println (typeof (y))

# and we have `readln`! the most complicated function of them all
# it receives two strings, it prints out the first one to the user,
# it receives user input from stdin, then it stores it in the second string
let name = ""
readln ("What's your name? ", name)

# `formatln` looks for variables between brackets and prints out their value
# if a variable is not found, it's not gonna throw an error or anything which is a problem
formatln ("Nice to meet you, {name}!")

# You can also put all that code in a block that you can always execute later
# It's something like a function, you can create them using the `block` function
# keep in mind that code blocks have their own scope of variables
# which can maybe be called "forcibly-pure" functions
# Silm translates the `\;` separator in blocks as a newline
block greet () let name = "" \; readln ("What's your name? ", name) \; formatln ("Nice to meet you, {name}!")

# blocks are called just like global functions get called, as you can see
greet ()

# If you want, you can also evaluate code and run it:
eval ("println ('W')")

# You may also import variables & blocks from other modules, say:
import ("version.slm")
println (version)

# Or interpret files using this file's current variables!
interpret ("welcome.slm")

# at this point the program will exit, but you can also explicltly exit using exit ()
# this is helpful when you're using the interactive mode
exit ()
```

`version.slm`:
```bash
let language_name = "Silm"
let version = "0.1.0"
```

`welcome.slm`:
```bash
println ("Welcome to {language_name}!")
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
