## Getting Started: Spinning Square 🔲
### A spinning square

[![Build Status](https://travis-ci.org/PistonDevelopers/Piston-Tutorials.svg?branch=master)](https://travis-ci.org/PistonDevelopers/Piston-Tutorials)

This tutorial aims to take you from an empty Cargo project to having a
window with a rotating square in it.
We plan to cover the core concepts used in the game in other tutorials.
This tutorial only covers project setup and contains a sample game simply
to test the build environment.


#### Prerequisites 📑
1. Make sure you have installed [Rust](https://www.rust-lang.org/tools/install)
and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2. Build a [hello-world project](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
with Cargo.

> Note: You may refer to the first few chapters of
[The Rust Book](http://doc.rust-lang.org/book/) for some detailed information.

#### At this stage 💁

Make sure that you're able to run the following commands successfully:
* `rustc -V`
* `cargo -V`

If you have failed either of these, please review the getting started
guide and make sure that you have the latest versions of `rustc` and `cargo`.

## Installing Dependencies ⚙️

Some parts of the Piston project depend on native C libraries. For example, in
order to display a window and hook it up to an OpenGL context, we can use
either Glutin, GLFW or SDL2 as the implementation of the windowing system.

The rest of this tutorial uses Glutin for windowing, so there's no need to
directly install any additional libraries for that purpose.

## Setting Up The Project 🛠️

If everything is set up correctly, it's time to create a Cargo project
and specify dependencies.


```bash
cargo new --bin getting-started
cd getting-started
```

Now, add the following project settings and dependencies to `Cargo.toml`
using your favorite editor:

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
with an OpenGL context. Glutin is a pure Rust alternative.
In this tutorial we're going forward with Glutin; you'll notice that in the
cargo file, we imported `glutin_window`.

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


## Writing Some Code 🧑‍💻

Now, let's dive into some game logic! Edit `src/main.rs` in your favorite editor:

^code(./src/main.rs)


Awesome! Now that we have the game code, let's get it running!

## Compiling And Running 🚀

With Cargo, downloading dependencies and building the application is as
simple as running `cargo build` from the main project directory.

If all goes well, you should have the binary `spinning-square` inside the `target/debug`
directory. You can run it by executing `cargo run`.

On your screen you should have a rotating square that looks like this:

![Result](./out.gif)

Woohoo! Well done! 🎉

## What's Next?

Take a look at the [piston-examples](https://github.com/pistondevelopers/piston-examples) repository for some interesting examples around what you can build using the Piston game engine. 🎮
