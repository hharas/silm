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
2. filename or input name (e.g. `main.slm` or `stdin`),
3. current line number in the file (e.g. `0` for `stdin`),
4. and a mutable reference to a vector of `Variable` structs.

Which makes it very portable and easy to implement, and also easy to develop further.

## How does the code work?

`main.slm`:
```bash
# We can initialise variables like this:
int x = 10;
float y = 1.5;
str word = "real";
char slim_is_a = 'W';
bool real = true;

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
str message = "What's your name? ";
str name = "";
readline (message, name)

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
