# Sudoku tutorial
by Sven Nilsen, 2017

## Chapter 2

Piston separates the core from the window backend.
The reason for this is that developers need to ship to many different platforms,
and it is very rare that a single window API is supported across all these devices.

Sometimes there are tiny differences between window APIs that makes a
difference in user experience, e.g. how mouse coordinates are handled outside
the window border. This means developers pick the API that work best for
the specific kind of application they are developing.

You can also write your own window backend.
For example, to make it easier to set up a project just the way you like it.
In Rust you can create a new library and reexport the stuff you need.
Such window backends are called "window wrappers".

One window wrapper that is often used for prototyping is the "piston_window" library.
In this tutorial we will not use it, because I want to explain how
the core of Piston works in order, so you can decide which window backend
is right for you.

In the Terminal window, type:

```
cargo add pistoncore-glutin_window
```

This adds the Glutin window backend for Piston.
Notice the prefix "pistoncore-" in the package name.
This is common for libraries in the Piston game engine, e.g. "piston2d-",
"piston3d-" etc.
The "pistoncore-" prefix means a library that is required to create an application.

Glutin supports the 3 major operating systems: Linux, Window and OSX.

In "main.rs", add `extern crate glutin_window;` and import `GlutinWindow`:

```rust
extern crate piston;
extern crate glutin_window;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;
```

In the `main()` function, create a window:

```rust
  let settings = WindowSettings::new("Sudoku", [512; 2])
      .exit_on_esc(true);
  let window: GlutinWindow = settings.build()
      .expect("Could not create window");
```

When we generated docs for the first time, we only added the "piston" crate.
This means the docs are not updated.
To update the docs, type `cargo doc` in the Terminal window.
Click refresh in the browser to see the changes.

When you look up `WindowSettings::build` in the docs, you will see this signature:

```rust
fn build<W: BuildFromWindowSettings>(&self) -> Result<W, String>
```

This means the function takes a generic parameter `W` with a trait constraint
`BuildFromWindowSettings`. All types that implement this trait are accepted.

Rust has a smart type system that understands how to infer the `W` generic
arguments to the build method.
Since you are specifying the type of the left side, the type system infers
that `W` must be equal to `GlutinWindow`:

```rust
let window: GlutinWindow = ...
```

You can picture how types propagate through your code and "fills the holes".
This is how you pick the window backend to use with Piston.

By using generics and traits and taking advantage of the smart type system,
you can write portable code that runs on all backends.
At the same time, the concrete type of a window backend is
exposed at application level in case you need some platform specific code.

The `GlutinWindow` struct exposes the underlying Glutin API, such that
you can write platform specific code when you need it.

The Piston core tries to put as little abstraction as possible
between you and the underlying platform, while at the same time offering portability.
Most of the code, perhaps 90%, can be written using generics and traits.
The motivation for this design is to reuse code across projects.
If Alice works on a library that Bob finds useful,
then Bob might send a PR to Alice and they both save work.

Notice that it is only the types and traits that matters,
not where these types or traits are located.
You can find them by all searching the docs.

Some people are worried about these extra window backend libraries,
but the smart thing to focus on is the larger ecosystem.
A way to measure code reusability is: How large potion of a code base
have as few dependencies as possible?

The Piston project organizes these types and traits in way to reuse
code across projects with the minimum number of dependencies per library.
There is a trade-off between the number of dependencies per library
and the number of dependencies in an application,
so a typical Piston project pulls in a lot of crates in the dependency graph.
You should not worry about this at all, it does not slow down compilation,
or development time, or makes your codebase more fragile.
Actually, this pattern makes a large codebase less fragile, since most of
the code, the generic libraries, breaks less often.

Piston is being developed and this leads to more frequent breaking changes
than what is ideal. Over time the number of breaking changes goes down,
so the expected payoff to organize code this way is a long term motivation.

In addition, the way the Piston project is architected is to use *zero*
shared dependencies when possible.
This means people can work on projects that happens completely in parallel.
For example, some projects are experimenting with an idea while others
are building an application.
Since this is equivalent to just using the larger Rust ecosystem,
there is hard to see where the boundary goes between the Piston project
and other projects. This does not matter at all,
because the only thing that matters is *development efficiency*.
Instead of sticking to a fixed idea of what a game engine is,
we turn the problem around and ask "what is the best way to make games?".

Now, add another import:

```rust
use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings};
```

This imports two new structs, `Events` and `EventSettings`,
which we use to crate an event loop.
The event loop is a kind of iterator that polls events from the window
and does its own internal logic.

To create an event loop, you must first make the `window` mutable.
Add the `mut` keyword before the variable name:

```rust
  let settings = WindowSettings::new("Sudoku", [512; 2])
      .exit_on_esc(true);
  let mut window: GlutinWindow = settings.build()
      .expect("Could not create window");

  let mut events = Events::new(EventSettings::new());

  while let Some(e) = events.next(&mut window) {

  }
```

The `events` object contains a state that keeps track of what to do next.
A typical game event loop does rendering, updating and handles input.

By default, the event settings are set to a typical FPS shooter.
This means that the event loop will consume a lot of energy
that is not needed by every kind of game.
The default frame rate and update rate settings are specified by `DEFAULT_MAX_FPS` and `DEFAULT_UPS`.

We will not do any animations. Our Sudoku game only needs to update
when the window receives some user input.

Import the trait `EventLoop` like this:

```rust
use piston::event_loop::{Events, EventLoop, EventSettings};
```

Add a `.lazy(true)` like this:

```rust
let mut events = Events::new(EventSettings::new().lazy(true));
```

This setting tells the event loop to not bother updating at all,
and rendering only when user input is received.

Notice how Piston separates settings, like `WindowSettings` and `EventSettings`, from the object that uses these settings.
This is a common pattern in Piston libraries.

[Goto Chapter 3](chp-03.md)
