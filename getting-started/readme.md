### Spinning Square [![Build Status](https://travis-ci.org/TyOverby/Piston-Tutorial.svg?branch=master)](https://travis-ci.org/TyOverby/Piston-Tutorial)

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

```rust
[package]

name = "getting-started"
version = "0.0.0"
authors = [
    "TyOverby <ty@pre-alpha.com>",
    "Nikita Pekin <contact@nikitapek.in>"
]

[dependencies.piston]

git = "https://github.com/PistonDevelopers/piston.git"

[dependencies.sdl2_game_window]

git = "https://github.com/PistonDevelopers/sdl2_game_window.git"

[dependencies.graphics]

git = "https://github.com/PistonDevelopers/rust-graphics.git"

[dependencies.opengl_graphics]

git = "https://github.com/PistonDevelopers/opengl_graphics.git"

```

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

```rust
extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;
extern crate shader_version;

use sdl2_game_window::WindowSDL2;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_2_1;

use piston::{
    Window,
    Render,
    RenderArgs,
    Update,
    UpdateArgs
};

use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d,
};

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    rotation: f64 // Rotation for the square.
}

impl<W: Window> App {
    fn render(&mut self, _: &mut W, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);
        // Clear the screen.
        context.rgba(0.0,1.0,0.0,1.0).draw(&mut self.gl);

        // Draw a box rotating around the middle of the screen.
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0,1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
    }

    fn update(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Create an SDL window.
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_2_1,
        piston::WindowSettings::default()
    );

    // Some settings for how the game should be run.
    let event_settings = piston::EventSettings {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    // Create a new game and run it.
    let mut app = App { gl: Gl::new(OpenGL_2_1), rotation: 0.0 };

    for e in piston::EventIterator::new(&mut window, &event_settings) {
        match e {
            Render(_args) =>
                app.render(&mut window, &_args),
            Update(_args) =>
                app.update(&mut window, &_args),
            _ => {},
        }
    }
}

```

## Compiling and running.

Awesome!  Now that we have the game code, let's get it running!
With Cargo, downloaing dependencies and building the application is as
simple as running `cargo build` from the main project directory..

If all goes well, you should have the binary `game` inside the `target`
directory.

Run it by executing `./target/game`.

On your screen you should have a rotating square that looks like this:

![Result](./out.gif)
