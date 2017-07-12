# Sudoku tutorial
by Sven Nilsen, 2017

## Chapter 3

Do to rendering, we need to add a graphics API to our project.

Piston has a 2D graphics API called Piston-Graphics,
but it is completely separated from the core.
You can choose another graphics API if you want to,
or you can use Piston-Graphics in a project without the Piston core.

When we started the Piston project, there was no working alternative for 2D
that did not require bindings to other libraries.
Actually, the Piston project started because we needed a way to test the 2D graphics library!

Piston-Graphics is designed to take advantage of GPU rendering,
but it does not support any complex features like setting shader parameters,
concave shapes. Instead, it does simple 2D with some few blend settings
that are supported by most graphics cards.

If you need 3D or advanced looking 2D graphics, then you can use OpenGL
or Gfx. This gives you more control over the performance and is more flexible.
In case you start with prototyping something with Piston-Graphics and gradually
transition to another API,
you can create your own traits and implement them for the types that are
used in Piston-Graphics. Such hybrid approaches are rare, but it is worth
considering if you want to reuse the logic.

Piston-Graphics is designed to be used with lower level graphics APIs.
We will use the OpenGl backend for Piston-Graphics.

In the Terminal window, type:

```
cargo add piston2d-graphics
cargo add piston2d-opengl_graphics
```

In "main.rs", add `extern crate graphics;` and `extern crate opengl_graphics`:

```rust
extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
```

Import `OpenGL` and `GlGraphics`:

```rust
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};
```

Add a setting that tells the window backend which OpenGL version to use:

```rust
  let opengl = OpenGL::V3_2;
  let settings = WindowSettings::new("Sudoku", [512; 2])
      .opengl(opengl)
      .exit_on_esc(true);
```

The most widely supported version is OpenGL 3.2,
but if you are e.g. developing inside a virtual environment you might
need to change it.

Create a new `GlGraphics` object:

```rust
  let mut events = Events::new(EventSettings::new().lazy(true));
  let mut gl = GlGraphics::new(opengl);
```

The `gl` object stores shaders and buffers that the OpenGL backend for Piston-Graphics needs to talk with the GPU.

Now we will handle the render events emitted by the event loop.
To do this we need a trait `RenderEvent` from the `input` submodule of
the `piston` library:

```rust
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
```

This puts some methods into scope such we can write:

```rust
  while let Some(e) = events.next(&mut window) {
      if let Some(args) = e.render_args() {

      }
  }
```

Every once in a while, the event loop emits a render event.
The event object is an `enum`, so you could match on it directly to
handle the event. However, in Piston it is recommended to use trait methods
because:

1. A trait can be implemented on more than one type.
2. It is easier to keep code from breaking.

In generic code, the `GenericEvent` is used because it is easier to
reason about the code when there is only one trait constraint.
It makes it easier to avoid a lot of work to fix breaking changes, e.g. in nested function calls.

The `GenericEvent` trait also makes it possible to implement custom
events. This is important on platforms that require special hardware.

Inside the render `if let` block, we call a method on the `gl` object
to create a `graphics::Context` and a graphics backend implementing
the `graphics::Graphics` trait.

```rust
    if let Some(args) = e.render_args() {
        gl.draw(args.viewport(), |c, g| {

        });
    }
```

OpenGL sets a rectangular area inside the frame buffer of the window
where fragments are written.
This rectangular area is called the "viewport".
The render arguments `args` has a method `.viewport()` that returns the viewport rectangle.

Before `GlGraphics` can render to the screen, it needs to make sure that
the correct shader program is activated on the GPU.
This is why you need to pass in a closure, so it can call it back when
everything is ready for rendering.
The `g` object is used to write the data from Piston-Graphics into buffers.
After the closure is called, `GlGraphics` flushes the buffers such that
stuff is rendered before the program does anything else OpenGL related rendering.

It is common to call the `.draw` method once for all 2D graphics.
This reduces the number of draw calls to the GPU.

Piston-Graphics is an immediate API, which means all rendering is decided
on the fly. The disadvantage with this approach is the bandwidth
between the CPU and the GPU can limit how much stuff you can draw on the screen.
The advantage with this approach is flexibility:
You can decide what to draw for each frame, without needing to keep track of state.

Game developers focus on *bottlenecks* when optimizing applications,
because that results the largest local gains in efficiency.
For applications that do not use animations, there is less benefit in
handling static buffers on the GPU, unless rendering is very expensive.
This is because rendering on user input only is a low frequency event.
Performance optimizations that you implement have less gains on average because they are multiplied with a low number.
When you press hardware to its limits, the bottlenecks are more likely to occur
in places with *high frequency events*.
One smart way to optimize an application is to reduce high frequency events into low frequency events,
instead of focusing on micro-optimizing drawing routines.

1. Focus on the top-down approach to optimization. Observe and learn
what the bottlenecks are.
2. Preparing code for micro-optimization is often more important than
doing the micro-optimization right away.
3. Do micro-optimization when there are no large gains in large scale optimization.

If you micro-optimize right away,
it can be more difficult to recognize the architectural bottlenecks.
By optimizing code from top to bottom, you make sure that more potential efficiency gains are implemented.
Piston-Graphics is not the fastest way to do 2D graphics,
but it will be fast enough for many kinds of applications.
This is why the game engine is organized in a modular way,
such that developers can make top-down decisions about their project.

For the rest of the tutorial, we will focus on the game itself.

Clear the window in a white color:

```rust
      gl.draw(args.viewport(), |c, g| {
          use graphics::{clear};

          clear([1.0; 4], g);
      });
```

Now we are ready to start working on the actual game!

[Goto Chapter 4](chp-04.md)
