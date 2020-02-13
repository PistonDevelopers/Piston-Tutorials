# Welcome to the Roguelike Tutorial!#

In this tutorial you will learn how to create a simple Roguelike game,
comparable to the Pokemon Mystery Dungeon games.
This tutorial consists of multiple parts, each of which will focus on a
different aspect of the game.

This tutorial has been designed to make you think rather than just blindly
copying code segments, and so it will provide you with function templates for
you to fill out yourself, however, at the end of each chapter, there will be a
link to a working implementation of the code which can be used as a
reference.

# Chapter 0 - Getting Started #

In this first section we are going to set up our project structure so that
you will be able to gain the most benefit from this tutorial.

## Assumptions ##

This tutorial will assume you have some level of comfort in using a terminal
and general programming knowledge. If you have any questions however, feel
free to create an issue or message me @cm16161 about it.

## Installing Rust and Cargo ##

To start with, we need to make sure that we have both Rust and Cargo
installed on our system. We can do this by following the instructions at:

[https://doc.rust-lang.org/cargo/getting-started/installation.html](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Setting Up Project Structure ##

With `Cargo` installed we will now use it to initialise our project structure
by executing the following command in your terminal:

`cargo new roguelike --bin`

Which will automatically generate the following project structure:

``` shell
.
├── Cargo.toml
└── src
    └── main.rs

```
If you go into the directory and then execute the command:
`cargo run`
The following output (or similar) should be output:

``` shell
      Compiling roguelike v0.1.0 (/home/chetan/Documents/roguelike)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/roguelike`
Hello, world!
```

The main takeaways are that it compiles successfully, and outputs `Hello,
World!`.

## Cargo Docs ##

This tutorial will utilise Cargo's ability to automatically generate
documentation, and output the documentation. This can be achieved by using
the terminal command:

``` shell
cargo doc --open
```

It is important to keep the documentation updated as it will be useful for
yourself, the programmer, and any other person who wants to use and
understand your code.

## Chapter 1 - Graphics ##

Follow the link to get to Chapter 1, which will handle setting up the
Graphics of our game and creating a window which we can play our game on.

[Chapter 1 - Graphics](chapters/Chapter_1-Graphics.md)
