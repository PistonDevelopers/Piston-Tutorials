# Sudoku tutorial
by Sven Nilsen, 2017; updated by Brent Westbrook, 2022

## Chapter 2

The Piston core is separate from any underlying window API. The reason for this
is that developers need to ship to many different platforms, and it is very rare
that a single window API is supported across all these devices.

Sometimes there are tiny differences between window APIs that matter for user
experience, e.g. how mouse coordinates are handled outside the window border.
This means developers pick the API that works best for the specific kind of
application they are developing.

You can also write your own window backend to quickly set up a project just the
way you like it. In Rust you can create a new library and reexport the stuff you
need. Such window backends are called "window wrappers".

One window wrapper that is often used for prototyping is that from the
`piston_window` crate. In this tutorial we will not use `piston_window` since I
want to explain how the core of Piston works relative to window backends and
graphics APIs. After learning how this works, you can decide which window
backend is right for your project.

In the Terminal window, type:

```
cargo add pistoncore-glutin_window
```

This adds the Glutin window backend for Piston. Notice the prefix "pistoncore-"
in the package name. This is common for libraries in the Piston game engine,
e.g. "piston2d-", "piston3d-" etc. The "pistoncore-" prefix means a library that
is required to create an application.

Glutin supports the 3 major operating systems: Linux, Window and OSX.

In "main.rs", add `extern crate glutin_window;` and import `GlutinWindow`:

```rust
use piston::WindowSettings;
use glutin_window::GlutinWindow;
```

In the `main()` function, create a window:

```rust
    let settings = WindowSettings::new("Sudoku", (640, 480)).exit_on_esc(true);
    let window: GlutinWindow =
        settings.build().expect("Could not create window");
```

When we generated docs for the first time, we only added the "piston" crate. To
update the docs, type `cargo doc` in the Terminal window. Click refresh in the
browser to see the changes.

When you look up `WindowSettings::build` in the docs, you will see this
signature:

```rust
fn build<W: BuildFromWindowSettings>(&self) -> Result<W, String>
```

This means the function takes a generic parameter `W` with a trait constraint
`BuildFromWindowSettings`. All types that implement this trait are accepted.

Rust has a smart type system that understands how to infer the `W` generic
argument to the build method. Since you are specifying the type of the left
side, the type system infers that `W` must be equal to `GlutinWindow`:

```rust
let window: GlutinWindow = ...
```

You can picture how types propagate through your code and "fill the holes". This
is how you pick the window backend to use with Piston.

By using generics and traits and taking advantage of the smart type system, you
can write portable code that runs on all backends. At the same time, the
concrete type of a window backend is exposed at application level in case you
need some platform specific code.

The `GlutinWindow` struct exposes the underlying Glutin API, such that you can
write platform specific code when you need it.

The Piston core tries to put as little abstraction as possible between you and
the underlying platform, while at the same time offering portability. Most of
the code, perhaps 90%, can be written using generics and traits. The motivation
for this design is to reuse code across projects. If Alice works on a library
that Bob finds useful except for a small feature, then Bob might send a PR to
Alice and they both save work.

Notice that it is only the types and traits that matters, not where these types
or traits are located. You can find them by all searching the docs.

Some people are worried about these extra window backend libraries, but the
smart thing is to focus on the larger ecosystem. A way to measure code
reusability: How many libraries in a code base have as few dependencies as
possible? The more libraries with few dependencies, the less likely are they to
be affected by breaking changes. This means that generic libraries should have
few dependencies.

The Piston project organizes these types and traits in way to reuse code across
projects with the minimum number of dependencies per library. There is a
trade-off between the number of dependencies per library and the number of
dependencies in an application, so a typical Piston project pulls in a lot of
crates in the dependency graph. You should not worry about this at all, it does
not slow down compilation, or development time, or make your codebase more
fragile. Actually, this pattern makes a large codebase less fragile, since most
of the code, the generic libraries, breaks less often.

Piston is under development and this leads to more frequent breaking changes
than what is ideal. Over time the number of breaking changes goes down, so the
expected payoff to organize code this way is a long term motivation.

In addition, the way the Piston project is architected is to use *zero* shared
dependencies when possible. This means people can work on projects that happen
completely in parallel. For example, some projects are experimenting with an
idea while others are building an application. Since this is equivalent to using
the larger Rust ecosystem, it is hard to see where the boundary goes between the
Piston project and other projects. This does not matter at all, because the only
thing that matters is *development efficiency*. Instead of sticking to a fixed
idea of what a game engine is, we turn the problem around and ask "what is the
best way to make games?".

Now, add another import:

```rust
use piston::event_loop::{EventSettings, Events};
```

This imports two new structs, `Events` and `EventSettings`, which we use to
create an event loop. The event loop is a kind of iterator that polls events
from the window and does its own internal logic.

To create an event loop, you must first make the `window` mutable. Add the `mut`
keyword before the variable name:

```rust
    let settings = WindowSettings::new("Sudoku", (640, 480)).exit_on_esc(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {

    }
```

The `events` object contains a state that keeps track of what to do next. A
typical game event loop does rendering, updating and handles input.

By default, the event settings are set to a typical FPS shooter. This means that
the event loop will consume a lot of energy that is not needed by every kind of
game. The default frame rate and update rate settings are specified by
`DEFAULT_MAX_FPS` (60 frames per second) and `DEFAULT_UPS` (120 updates per
second).

We will not do any animations in our Sudoku game, so the game only needs to
render when the window receives some user input.

Import the trait `EventLoop`:

```rust
use piston::EventLoop;
```

Add a `.lazy(true)`:

```rust
let mut events = Events::new(EventSettings::new().lazy(true));
```

This setting tells the event loop to not bother updating at all, and render only
when user input is received.

Notice how Piston separates settings, like `WindowSettings` and `EventSettings`,
from the object that uses these settings. This is a common pattern in Piston
libraries.

The final contents of your `main.rs` file should be:

```rust
use glutin_window::GlutinWindow;
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, WindowSettings};

fn main() {
    let settings = WindowSettings::new("Sudoku", (640, 480)).exit_on_esc(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));

    while let Some(e) = events.next(&mut window) {}
}
```

and your editor will probably be complaining about the unused variable `e`
inside of our `while let`.

### Troubleshooting

Although we haven't written any code inside of the even loop yet, you can go
ahead and try to `cargo run` the code to make sure everything is working. On my
laptop, I get an error like

```
thread 'main' panicked at 'Could not create window: CreationErrors([OpenGlVersionNotSupported, OsError("GL context creation failed")])', src/main.rs:8:26
```

To fix this, you can invoke `cargo run` with the `LIBGL_ALWAYS_SOFTWARE`
environment variable set.

```
LIBGL_ALWAYS_SOFTWARE=1 cargo run
```

Again on my laptop, I get another error:

```
thread 'main' panicked at 'Could not create window: CreationErrors([NoAvailablePixelFormat, OsError("Couldn't find any available vsync extension")])', src/main.rs:8:26
```

To fix this, enable `vsync` in your original `WindowSettings::new` call:

```rust
    let settings = WindowSettings::new("Sudoku", (640, 480))
        .exit_on_esc(true)
        .vsync(true);
```

[Goto Chapter 3](chp-03.md)
