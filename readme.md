# About This Tutorial
travis.ci

This tutorial is also a git repository.  Feel free to make pull requests or
simply clone the tutorial and immediately get the code that _should_ work.

# Tutorial

In this tutorial, I hope to get you from an empty Cargo project, to having a
window with a triangle drawn on it.
I assume that you have installed Rust and Cargo, and have already built a
hello-world project with Cargo.
If you haven't met these criteria, please read the first few chapters of
[The Rust Guide](http://doc.rust-lang.org/guide.html) and come back once
you've finished.

## Installing Dependencies

Parts of the Piston project depend on native C libraries.  For example, in
order to display a window and hook it up to an OpenGL context, we can use
either GLFW or SDL2 as the implementation of our windowing system.

The rest of this tutorial uses GLFW for windowing, so we will need to
install its native library.

### GLFW on OSX

If you use [Homebrew](brew.sh), this is as simple as
`brew install homebrew/versions/glfw3`.  That's it.  You're done.

Otherwise, follow these steps.
1. Install CMake.  CMake is the program that is used to build GLFW
   from scratch.
2. Install git.
3. `cd` to some empty working directory.
4. `git clone https://github.com/glfw/glfw.git`
5. `cd glfw`
6. `git checkout 3.0.3`
7. `cmake -DCMAKE_C_FLAGS=-fPIC -DBUILD_SHARED_LIBS=ON -DGLFW_BUILD_EXAMPLES=OFF -DGLFW_BUILD_TESTS=OFF -DGLFW_BUILD_DOCS=OFF .`
8. `sudo make all install`



### GLFW on Linux

Install the system level dependencies `xorg-dev` and `libglu1-mesa-dev`
using apt-get, yum, or by building from source.

1. Install CMake.  CMake is the program that is used to build GLFW
   from scratch.
2. Install git.  Git is used to download GLFW.
3. `cd` to some empty working directory.
4. `git clone https://github.com/glfw/glfw.git`
5. `cd glfw`
6. `git checkout 3.0.3`
7. `cmake -DCMAKE_C_FLAGS=-fPIC -DBUILD_SHARED_LIBS=ON -DGLFW_BUILD_EXAMPLES=OFF -DGLFW_BUILD_TESTS=OFF -DGLFW_BUILD_DOCS=OFF .`
8. `sudo make all install`


### GLFW on Windows
TODO

## Setting Up The Project

Alright, if everything is set up correctly, it's time to get a Cargo project
built, and Piston installed.

```bash
mkdir my-piston-project
cd my-piston-project
touch Cargo.toml
```

Now in your favorite editor, lets add project settings and dependencies to
`Cargo.toml`.


```
[package]

name = "my-piston-project"
version = "0.0.0"
authors = ["your name here"]

[dependencies.piston]

git = "https://github.com/PistonDevelopers/piston.git"

[dependencies.glfw_game_window]

git = "https://github.com/PistonDevelopers/glfw_game_window.git"

[dependencies.graphics]

git = "https://github.com/PistonDevelopers/rust-graphics.git"

[dependencies.opengl_graphics]

git = "https://github.com/PistonDevelopers/opengl_graphics.git"

[[bin]]

name = "hello"
path = "./src/hello.rs" # We will create this file in the next section.
```

You might be thinking that this is a lot of dependencies for such a simple
example application.
This is because of how the Piston Projects are set up.
The `piston` and `graphics` libraries are do a lot of work by themselves, but
they are made to be completely independent of a backing implementation.
For example, when it comes to displaying a window and getting keyboard events
in a cross-platform maner, you can use either GLFW or SDL2.  (In this case
we went with GLFW, so we also imported `opengl_graphics`).
`graphics` is a 2d graphics API that doesn't care about how things are
*actually* drawn to the screen.
If you implement the `graphics` interface yourself, you could route it
through directx, or render straight to a png.
In our case, we're rendering using OpenGL, so we also depend on
`opengl_graphics`.

This is a common theme throught the Piston projects.
While other game engines might encompas more functionality, we prefer to have
many libraries that are seperated and extendable, but also work well when
combined.


## Writing Some Code

Ok, time for some actual programming.

First lets create the `src` directory and a file use as the entry point for
our application.

```bash
mkdir src
cd src
touch hello.rs
```

Now in your favorite editor edit `hello.rs`.

```

extern crate graphics;
extern crate piston;
extern crate glfw_game_window;

use Window = glfw_game_window::GameWindowGLFW;

use piston::Game;
use piston::GameWindowSettings;

pub struct App;

impl Game for App {}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Hello Piston",
            size: [800, 800],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings {
        updates_per_second: 120,
        max_frames_per_second: 60
    };

    app.run(&mut window, &game_iter_settings);
}
```
