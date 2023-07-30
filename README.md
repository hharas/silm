<img align="left" style="width: 277px" src="./silm.png" width="277" />

**Silm is a line-by-line-interpreted programming language written in Rust.**

At the essence of it, Silm is merely an `interpret()` function that accepts four arguments:  
a line of code,  
a filename or input name (e.g. `main.slm` or `<stdin>`),  
current line number in the file (e.g. `1` for `<stdin>`),  
and a mutable reference to a vector of `Variable` structs.

Read the [Wiki](https://github.com/hharas/silm/wiki) for more information on Silm.

---

## What does Silm mean?

Silm (سِلم) is an Arabic word that means "Serenity".

## What file is what?

### `src/main.rs`

This is the CLI front-end of Silm, or specifically the `src/interpreter.rs` module. It can run on interactive mode, interpret a file by going through it line by line, execute commands from the command line, etc.

### `src/interpreter.rs`

Now that's the whole wrapper of the language, it contains the `interpret()` function which is ran on files line-by-line or through an interactive mode.

### `src/commands.rs`

That's where global commands are defined, which are the commands `interpreter.rs` will detect and execute. It contains commands like `println`, `let`, `readln` and more.

### `src/functions.rs`

That's Silm's Standard Library of functions that can't be used globally as commands but rather inside commands. It contains functions like `eq`, `ne`, `typeof` and more.

### `src/helper.rs`

These are helper functions that are needed for the commands & functions in the standard library to work properly. It contains some algorithms that make up the essence of the language.

### `src/version.rs`

Just a file that reminds Silm what version it is right now.

## Contribution

This is a small fun project for me but if you have any ideas for improving it then feel free to make a pull request! I'm interested in seeing Silm get bigger and bigger.

## License

Silm is licensed under the GNU General Public License version 3.
