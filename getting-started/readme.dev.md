## Getting Started Spinning Square
### A spinning square

[![Build Status](https://travis-ci.org/PistonDevelopers/Piston-Tutorials.svg?branch=master)](https://travis-ci.org/PistonDevelopers/Piston-Tutorials)

In this tutorial, I hope to get you from an empty Cargo project to having a
window with a rotating square in it.
This tutorial does ___not___ explain concepts used in the game, as those
will be covered by other tutorials.
This tutorial only covers project setup and contains a sample "game" simply
to test the build environment.


I assume that you have installed Rust and Cargo, and have already built a
hello-world project with Cargo.
If you haven't met these criteria, please read the first few chapters of
[The Rust Guide](http://doc.rust-lang.org/guide.html) and come back once
you've finished.

![Result](./out.gif)

#### At this stage

* You should be able to run the command `rustc -v`
* You should be able to run the command `cargo -V`

If you have failed either of these, please review the getting started
guide and make sure that you have the latest versions of `rustc` and `cargo`.

## Installing Dependencies

Parts of the Piston project depend on native C libraries.  For example, in
order to display a window and hook it up to an OpenGL context, we can use
either GLFW or SDL2 as the implementation of the windowing system.

The rest of this tutorial uses SDL2 for windowing, so we will need to
install the SDL2 native library.

### GLFW on OSX

If you use [Homebrew](http://brew.sh), installing sdl2 is as simple as
`brew install sdl2`.  That's it.  Done.

Otherwise, follow the steps under [sdl2 on Linux](#sdl2-on-linux)

Honestly, it's probably easier to just install Homebrew and then follow the
homebrew instructions.

### SDL2 on Ubuntu
If you are on Ubuntu Trusty, you can run
`sudo apt-get install libsdl2-dev`!

### SDL2 on Linux
Follow the instructions found [here](http://nothingtocode.blogspot.com/2013/07/setting-up-sdl2-in-ubuntu-or-linux-mint.html)

#### At this stage
`ldconfig -p | grep libSDL2` should print out some paths to the .so libraries.

### SDL2 on Windows
TODO

## Setting Up The Project

If everything is set up correctly, it's time to create a Cargo project
and specify dependencies.


```bash
mkdir getting-started
cd getting-started
touch Cargo.toml
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
in a cross-platform manner, you can use either GLFW or SDL2.
GLFW and SDL2 are both C and C++ cross-platform libraries for creating windows
with an OpenGL context.
In this tutorial I chose SDL2, so you will notice that in the cargo file, we
imported `sdl2_game_window`.
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

Ok, time for some game logic.

First create the source directory and a file use as the entry point for
our application.

```bash
mkdir src
touch src/game.rs
```

Now in your favorite editor edit `src/main.rs`.

^code(./src/main.rs)

## Compiling and running.

Awesome!  Now that we have the game code, let's get it running!
With Cargo, downloaing dependencies and building the application is as
simple as running `cargo build` from the main project directory..

If all goes well, you should have the binary `game` inside the `target`
directory.

Run it by executing `./target/game`.

You can also directly run it by running `cargo run`

On your screen you should have a rotating square that looks like this:

![Result](./out.gif)
