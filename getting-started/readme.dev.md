## Getting Started Spinning Square
### A spinning square

[![Build Status](https://travis-ci.org/PistonDevelopers/Piston-Tutorials.svg?branch=master)](https://travis-ci.org/PistonDevelopers/Piston-Tutorials)

**Notice! Use [Rust Beta (1.8)](https://www.rust-lang.org/downloads.html) or newer, see [#1050](https://github.com/PistonDevelopers/piston/issues/1050)**.

In this tutorial, I hope to get you from an empty Cargo project to having a
window with a rotating square in it.
This tutorial does ___not___ explain concepts used in the game, as those
will be covered by other tutorials.
This tutorial only covers project setup and contains a sample "game" simply
to test the build environment.


I assume that you have installed Rust and Cargo, and have already built a
hello-world project with Cargo.
If you haven't met these criteria, please read the first few chapters of
[The Rust Book](http://doc.rust-lang.org/book/) and come back once
you've finished.

![Result](./out.gif)

#### At this stage

* You should be able to run the command `rustc -V`
* You should be able to run the command `cargo -V`

If you have failed either of these, please review the getting started
guide and make sure that you have the latest versions of `rustc` and `cargo`.

## Installing Dependencies

Parts of the Piston project depend on native C libraries. For example, in
order to display a window and hook it up to an OpenGL context, we can use
either Glutin, GLFW or SDL2 as the implementation of the windowing system.

The rest of this tutorial uses Glutin for windowing, so we won't need to
directly install any additional libraries for that purpose.

## Setting Up The Project

If everything is set up correctly, it's time to create a Cargo project
and specify dependencies.


```bash
cargo new --bin getting-started
cd getting-started
```

Now in your favorite editor, add project settings and dependencies to
`Cargo.toml`.

^code(./Cargo.toml)

You might be thinking that this is a lot of dependencies for such a simple
example application.
This is because of how the Piston Projects are organized.
The `piston` and `graphics` libraries are able to do a lot of work by
themselves, but they are made to be completely independent of a
backing implementation.
For example, when it comes to displaying a window and getting keyboard events
in a cross-platform manner, you can use either Glutin, GLFW or SDL2.
GLFW and SDL2 are both C and C++ cross-platform libraries for creating windows
with an OpenGL context. Glutin - pure Rust alternative.
In this tutorial I chose Glutin, so you will notice that in the cargo file, we
imported `glutin_window`.
`opengl_graphics` is another backend that implements the interface defined in
`graphics`.
`graphics` is a 2d graphics API that doesn't care about how things are
*actually* drawn to the screen.
If you implement the `graphics` interface yourself, you could route it
through directx, or render straight to a png.
In this tutorial, we are rendering using OpenGL, so we'll use `opengl_graphics`.

The pattern of "interface" and "backend" is very common with Piston Projects.
While other game engines might encompass lots of functionality, we prefer to have
many libraries that are separate and extendable, but also work well when
combined.


## Writing Some Code

Ok, time for some game logic. Edit `src/main.rs` in your favorite editor:

^code(./src/main.rs)

## Compiling And Running

Awesome! Now that we have the game code, let's get it running!
With Cargo, downloading dependencies and building the application is as
simple as running `cargo build` from the main project directory.

If all goes well, you should have the binary `spinning-square` inside the `target/debug`
directory.

Run it by executing `cargo run`.

On your screen you should have a rotating square that looks like this:

![Result](./out.gif)
