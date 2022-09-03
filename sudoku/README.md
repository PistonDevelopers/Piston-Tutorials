# Welcome to the Sudoku tutorial!
by Sven Nilsen, 2017
updated by Brent Westbrook, 2022

In this tutorial, you will learn how to create a Sudoku game with Piston. This
tutorial is a bit long, because it goes step by step through the thinking
process when developing a game. Expect to spend a day or two getting through
this!

I assume you have gotten through the [getting-started](../getting-started)
tutorial and know how to set up a new Rust project. Some of these steps will be
repeated briefly in the first chapter.

## Chapter 1

Type the following command in the Terminal window:

```
cargo new sudoku
cd sudoku
```

In the file "src/main.rs", you should see the following:

```rust
fn main() {
    println!("Hello, world!");
}
```

To test that it runs, type this in the Terminal window:

```
cargo run
```

You should see something like:

```
$ cargo run
   Compiling sudoku v0.1.0 (file:///Users/sven/rust/Piston-Tutorials/sudoku)
    Finished dev [unoptimized + debuginfo] target(s) in 2.42 secs
     Running `target/debug/sudoku`
Hello, world!
```

Now, we need to add a few libraries to the Cargo.toml. To make this more
efficient, we will use the `cargo add` subcommand.

The first library we will add is `piston`. This is the core library of the
Piston game engine.

Type the following in the Terminal window:

```
cargo add piston
```

When you open up "Cargo.toml", you should see something like:

```
[dependencies]
piston = "0.53.1"
```

Now you can updated your `main.rs` file:

```rust
use piston::WindowSettings;

fn main() {
    let settings = WindowSettings::new("Sudoku", (640, 480)).exit_on_esc(true);

    println!("{}", settings.get_exit_on_esc());
}
```

When typing `cargo run` in the Terminal window, you should see the program
printing out `true`.

The first line `use piston::WindowSettings;` imports the `WindowSettings` struct
from the `piston` crate. We use the associated function `new` to create one of
these structs, passing two parameters: the title of the window ("Sudoku" in our
case), and the window size (640x480 pixels in our case). Then we call
`.exit_on_esc(true);` to indicate that we want the window to exit when we press
ESC on the keyboard.

The `WindowSettings` struct is used to tell the window backend how to load the
window. It reads out the settings by calling the corresponding getter methods
like the `get_exit_on_esc` method we used in our final `println`.

Not every window backend in Piston is guaranteed to respect the settings you
choose. For example, if the window runs on a platform where all applications run
fullscreen, it will ignore whether you set `.fullscreen(true)` or
`.fullscreen(false)`, but the Piston documentation shows all of the settings
available within a `WindowSettings`.

To view the docs, type the following in the Terminal window:

```
cargo doc --open
```

This can take quite a while with all of the dependencies of Piston, so you can
also select just the Piston docs with the command

```
cargo doc --open -p piston
```

This will open up Piston's documentation in the default browser, giving you
something like:

![docs](./images/docs.png)

[Goto Chapter 2](chp-02.md)
