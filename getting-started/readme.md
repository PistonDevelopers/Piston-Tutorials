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
either GLFW or SDL2 as the implementation of the windowing system.

The rest of this tutorial uses SDL2 for windowing, so we will need to
install the SDL2 native library.

### SDL2 and Freetype on OSX

If you use [Homebrew](http://brew.sh), installing sdl2 and freetype is as simple as
`brew install sdl2 freetype`. That's it. Done.

Otherwise, follow the steps under [sdl2 on Linux](#sdl2-on-linux).

Honestly, it's probably easier to just install Homebrew and then follow the
homebrew instructions.

### SDL2 and Freetype on Ubuntu
If you are on Ubuntu Trusty, you can run
`sudo apt-get install libsdl2-dev libfreetype6-dev`!

### SDL2 on Linux
Follow the instructions found [here](http://nothingtocode.blogspot.com/2013/07/setting-up-sdl2-in-ubuntu-or-linux-mint.html).

#### At this stage
`ldconfig -p | grep libSDL2` should print out some paths to the .so libraries.

### SDL2 and Freetype on Windows
Copy `SDL2.dll` to the project directory root (where your `Cargo.toml` is, see below).
You will need to distribute this file alongside your `.exe`.
- For 32-bit Windows, download [this SDL2.dll](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/i686/SDL2.dll?raw=true).
- For 64-bit Windows, download [that SDL2.dll](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/x86_64/SDL2.dll?raw=true).

Copy `libSDL2.dll.a` and `libfreetype-6.a` to `$RUST_ROOT\bin\rustlib\$CPU_ARCH-pc-windows-gnu\lib`.
- For 32-bit Windows, download [this libSDL2.dll.a](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/i686/libSDL2.dll.a?raw=true) and [this libfreetype-6.a](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/i686/libfreetype-6.a?raw=true) to e.g. `C:\Rust\bin\rustlib\i686-pc-windows-gnu\lib\`.
- For 64-bit Windows, download [that libSDL2.dll.a](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/x86_64/libSDL2.dll.a?raw=true) and [that libfreetype-6.a](https://github.com/tedsta/getting-started-with-piston/blob/master/windows_clibs/x86_64/libfreetype-6.a?raw=true) to e.g. `C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\`.

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

```toml
[package]

name = "getting-started-spinning-square"
version = "0.0.0"
authors = [
    "TyOverby <ty@pre-alpha.com>",
    "Nikita Pekin <contact@nikitapek.in>"
]

[[bin]]
name = "spinning-square"

[dependencies.piston]
git = "https://github.com/PistonDevelopers/piston.git"

[dependencies.piston2d-graphics]
git = "https://github.com/PistonDevelopers/graphics.git"

[dependencies.pistoncore-sdl2_window]
git = "https://github.com/PistonDevelopers/sdl2_window.git"

[dependencies.piston2d-opengl_graphics]
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
imported `sdl2_window`.
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
touch src/main.rs
```

Now in your favorite editor edit `src/main.rs`.

```rust
extern crate piston;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;
use piston::window::{ WindowSettings, Size };
use piston::event::{ RenderArgs, UpdateArgs };
use sdl2_window::Sdl2Window as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64   // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        let center_context = &context.trans((args.width / 2) as f64, (args.height / 2) as f64)
                                     .rot_rad(self.rotation)
                                     .trans(-25.0, -25.0);
        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw([0, 0, args.width as i32, args.height as i32], |_, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, center_context.transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::_3_2;

    // Create an SDL window.
    let window = Window::new(
        opengl,
        WindowSettings::new("spinning-square".to_string(),
                            Size { width: 200, height: 200 })
                           .exit_on_esc(true)
    );
    let window = Rc::new(RefCell::new(window));

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    for e in piston::events(window) {
        use piston::event::*;

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
```

## Compiling And Running

Awesome! Now that we have the game code, let's get it running!
With Cargo, downloading dependencies and building the application is as
simple as running `cargo build` from the main project directory.

If all goes well, you should have the binary `spinning-square` inside the `target/debug`
directory.

Run it by executing `./target/debug/spinning-square`.

You can also directly run it by running `cargo run`.

On your screen you should have a rotating square that looks like this:

![Result](./out.gif)
