# Advanced Piston

## Chapter 1 - Events

### Imports

- The Piston core libraries
  - Used by generic libraries
  - `use input::*;`
- The [piston](https://crates.io/crates/piston) crate
  - Reexports core libraries
  - Used by generic applications
  - `use piston::input::*;`
- Window wrappers such as [piston_window](https://crates.io/crates/piston_window)
  - Used for convenience
  - Should not be depended on by libraries, because they choose a specific backend
  - `use piston_window::*;`

The word "generic" is used to describe something that does not depend on a specific platform, operating system or data structure.

### Core libraries

Piston consists of 3 small core libraries.
The core libraries only have one purpose: To describe events, how they are retrieved from the window and how they behave in an event loop.
Piston uses events to tell the application when to update, render and handle user input.

- [pistoncore-input](https://crates.io/crates/pistoncore-input)
- [pistoncore-window](https://crates.io/crates/pistoncore-window)
- [pistoncore-event_loop](https://crates.io/crates/pistoncore-event_loop)

These 3 libraries are reexported in the [piston](https://crates.io/crates/piston) crate.

Reasons Piston uses a modular core:

1. Less maintenance when there are breaking changes
2. Customized design for windows and event loops

### How to handle events

The recommended way of handling events is to import event traits.
These are defined in the [pistoncore-input](https://crates.io/crates/pistoncore-input) crate.

For example:

```rust
use input::{RenderEvent, UpdateEvent};

while let Some(e) = events.next(&mut window) {
    // Closure style.
    e.render(|args| {
        ...
    });
    // `if let` style.
    if let Some(args) = e.render_args() {
        ...
    }
}
```

The reason to use event traits instead of matching on the `Input` structure,
is that in generic code an event can be anything that implements the `GenericEvent` trait.

Some libraries handles events for you, for example from the [piston_window](https://crates.io/crates/piston_window) crate:

```rust
// Calls the closure on render events.
window.draw_2d(&e, |c, g| {
   ...
});
```

A common trick used by such methods, is a generic return value `U` from the closure that is wrapped into an `Option<U>`.
If it returns `None`, the closure was not called.

### Window events

When the user presses a key or scrolls with the mouse,
this information is registered by the operating system and put in an event queue.
The window manager keeps track of which events should be sent to window.
On platforms with a mouse cursor device, the window manager keeps track of a focused window.
Events, such as mouse coordinates, are sent to the currently active window.

The window backend interfaces with the operating system and knows how to poll out
events from the queue.
A window backend consists of 2 things:

- A window API library (e.g. [Glutin](https://crates.io/crates/glutin), [Glfw](https://crates.io/crates/glfw), [SDL2](https://crates.io/crates/sdl2))
- An window backend for Piston (e.g. [glutin_window](https://crates.io/crates/pistoncore-glutin_window), [glfw_window](https://crates.io/crates/pistoncore-glfw_window), [sdl2_window](https://crates.io/crates/pistoncore-sdl2_window))

Notice that window backends for Piston are prefixed with "pistoncore-",
such that if you depend on e.g. "glutin_window" you write "pistoncore-glutin_window" in the Cargo.toml.
The prefix "pistoncore-" is used on libraries that are often required to make an application up and running.

In Piston, a window backend is a struct that implements the traits `Window` and `AdvancedWindow`.
Those traits are defined in the [pistoncore-window](https://crates.io/crates/pistoncore-window) crate.
The `Window` trait specifies the minimum requirements to get a game loop working.
The `AdvancedWindow` trait is a fully supported window backend and includes all features that are supported in Piston.

Piston also has a `NoWindow` implementation that can be used in game servers.
This is included in the [pistoncore-window](https://crates.io/crates/pistoncore-window) crate.

### Update and render

The event loop is an algorithm that makes sure to shedule update and render events at the right moments.
These are timed to follow updates per second (UPS) and frames per second (FPS) respectively.

- Updating is what the application "does", measured by UPS
- Rendering is how the application "looks", measured by FPS

When you are deciding which UPS and FPS to use, there is a tradeoff between energy usage, accuracy and graphics.
In advanced applications, you might change these settings on the fly to spend less energy, e.g. when
the window is not in focus.

The way you set UPS and FPS is though `EventSettings`, defined in [pistoncore-event_loop](https://crates.io/crates/pistoncore-event_loop).

Update events in Piston uses fixed time steps, such that an application using no inputs or random events is deterministic.
This means you can run the application multiple times and expect the exact same behavior.
By default, the updates per second (UPS) are set to 120, which is usually good enough for VR and first person shooters.
For simple games, such as tetris, this is probably not needed, so you can set it lower to e.g. 20 or 30.

Render events in Piston can slip over time when rendering takes too long.
This means the event loop will not try to catch up.
You should not put update logic in the render event because it will lead to undeterministic behavior.

### Benchmark mode

`EventSettings::bench_mode` is a flag that tells the event loop to ignore user input
and run updates and rendering as fast as it can, while pretending that update and render events are perfect.
This is used to benchmark the performance of the whole game engine and give reliable results.
A common technique is to make the application run e.g. 1000 render frames and then exit the loop.

In the terminal window you can type `time <app>` to measure how long time the program takes to run.
A standard test in the Piston project is to run 3 times and delete the slowest result.
This reduces inaccuracy caused by background processes in the operating system.
