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

```toml
[package]

name = "spinning-square"
version = "0.1.0"
authors = [
    "TyOverby <ty@pre-alpha.com>",
    "Nikita Pekin <contact@nikitapek.in>"
]

[[bin]]
name = "spinning-square"

[dependencies]
piston = "0.55.0"
piston2d-graphics = "0.44.0"
pistoncore-glutin_window = "0.72.0"
piston2d-opengl_graphics = "0.84.0"

```

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

```rust
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

```


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
