# Advanced Piston

## Chapter 2 - Writing Idiomatic Code

What is meant by "idiomatic" code is a pattern that works and has a predictable maintenance cost for large projects.
It is not meant to mean that you can not do any better or that everyone should program this way.
The great thing about Rust is that it allows a wide style of designs, e.g. functional APIs, stream processing etc.
You should not think that one particular flavor is superior to another,
but learn different techniques and figure out the trade-offs.
You know best what works for you! 

### Model/View/Controller

The Piston project often organizes code by the [Model/View/Controller](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller) pattern.
This pattern has been tested in the industry for many years.
The good thing about this pattern is that it scales, so you can combine it with other patterns at lower levels.

- A model is simply some data structure, database etc.
- A view is how something is rendered.
- A controller stores the state, transforms input events into other events or actions, or deals with application logic in general.

Currently, there are no traits defined for this abstraction, it is simply a way of splitting up code into reusable parts.

Two simple examples of this pattern is [timer_controller](https://github.com/PistonDevelopers/timer_controller) and [button_controller](https://github.com/pistondevelopers/button_controller).
More libraries will be added over time.
You can take a look [here](https://github.com/PistonDevelopers/piston/wiki/Piston-overview) for an updated overview.

This code is reusable for any model or view, but it is also more work to set up. You might want to use a tailored API for your use cases, e.g. [Conrod](https://github.com/PistonDevelopers/conrod/) for UI. This is why Piston uses a modular design with a small core, so the code that is reused for particular projects, e.g. editors, can be worked on in parallel with other projects.

### Handling events in controller

Most controllers have an `event` method that handles events.
This pattern allows users to hook up the controller logic using a single line of code.

Here is an example from the [button_controller](https://github.com/PistonDevelopers/button_controller) library:

```rust
pub fn event<E: GenericEvent>(&mut self, layout: Rectangle, transform: Matrix2d, e: &E) {
    ...
}
```

It is common to pass in extra parameters before `e: &E` that changes dynamically or are controlled by some external logic.

The `event` method is often called only when the controller is activated.
When the controller contains enabled/disabled state or requires calling at all times,
this must be documented on the `event` method.

It is recommended to not do any rendering in the `event` method,
because the order of rendering is often reversed event logic order.
Instead, you can use a `View` struct to do rendering and a `Controller` struct to do application logic.

### Rendering

A recommended pattern is to end names of structs that do rendering with `View` or `Visualizer`.

- A `View` struct displays information from a controller
- A `Visualizer` struct displays information without any controller

By separating this code from controllers, one can reuse the same `View` struct with multiple controllers.

E.g. if you are using the [button_controller](https://github.com/PistonDevelopers/button_controller) library,
you might name a struct `ButtonView` that renders all buttons.

### Settings

When the behavior of a controller or view depends on some settings,
it is common to call the constructor with a settings struct.

The Piston project recommends using [the builder pattern](https://doc.rust-lang.org/1.12.0/style/ownership/builders.html)
and end the name with `Settings`.

A settings struct might be used to build multiple types.
One trick is to add a `BuildFromXSettings` trait and add a `build<X: BuildFromXSettings>` method to the settings struct.

For example, when you create a window in Piston,
you specify the type of the window like this:

```rust
let mut window: GlutinWindow = settings.build().expect("Could not create window");
```

### Properties

When writing methods for getting and setting options, you can have `&mut self`, `self`, `&self` etc.
This might easily lead to inconsistent usage across libraries.

The Piston project recommends the following design:

- Whenever a method starts with `set_` it should take `&mut self`
- Whenever there is a `set_` method and you need a get, it should start with `get_`
- Methods without a prefix are either read only or take `self` and returns `Self`

This means that builder patterns doesn't need `set_` prefixes, for example:

``` Rust
let events = events.max_fps(60).ups(120);
```

The method without prefix never gets in the way for the read only version, because when a property is read only you do not need a builder version. It also makes it possible to add either a read only or builder method later without breaking the API.

For mutable references to the interior of an object:
- `get_foo_mut` when there is a `set_` method
- `foo_mut` when there is no `set_` method

The Rust API guidelines distinguishes between Builder patterns and ordinary objects, so the `get_` prefix is removed for ordinary objects.
See https://aturon.github.io/style/naming/README.html#getter/setter-methods-[rfc-344].

In several Piston libraries the distinction between builders and ordinary objects is blurry, so `get_` can be used avoid breaking the API.

### Scalar precision, vectors and matrices

The recommended float number format for application logic is `f64`.
If you do not know what precision you need in your application logic, you should use `f64`.
It only takes up a little more memory, but it gives much, much better accuracy than `f32`.
File formats should use `f64` when possible, because going from `f32` to `f64` is harder than vice versa.
In the vast number of applications, choosing between `f32` and `f64` is not the bottleneck,
so you should choose with `f64` unless there is a good reason.

*Using `f32` instead of `f64`, when `f32` does not give good enough accuracy in application logic, might lead to serious accidents.*

The default float number format for 2D graphics math is `f64`.
Most modern CPUs have hardware support for `f64`.
Counter-intuitively, `f32` is sometimes slower than `f64`,
because the CPU might use `f64` under the hood for `f32` intructions.
`f64` gives also larger range for invariant matrix compositions `M * M^-1 = I`.
This is a common problem with `f32` in large hierarchial scenes for e.g. 2D animation software.

The default float number format for 3D rendering is `f32`.
In most cases, 3D rendering does not affect application logic.

By default, [Piston-Graphics](https://github.com/pistondevelopers/graphics) converts from `f64` to `f32` after doing
matrix transformations on the CPU to achieve better accuracy and pack triangles in dynamic buffers.
This behavior can be overriden by a graphics backend by changing how the `Graphics` trait is implemented.

The default float number format for colors is `f32`.
Colors often do not need `f64` precision.
The default color space for 2D APIs is sRGB.

The Piston project recommends fixed arrays as vector format, e.g. `[f64; 4]`.
One benefit of arrays over tuples is that they can be iterated over.

The Piston project recommends fixed arrays as matrix format `[[f64; 4]; 4]`.

### SIMD optimizations

When using Rust with LLVM, vector and matrix operations are often optimized with SIMD instructions.
It is rarely need to use SIMD data types directly in your Rust code.

### Writing generic code for float formats

The Piston project uses the [piston-float](https://crates.io/crates/piston-float) crate to support generic code for float formats.

The `Float` trait inherits `Send` and `Sync` to make it easier to write multi-threaded code.

### Organizing math modules in Piston libraries

Mathematics is often hard to formalize because it has many applications with slightly different usages.
The architecture of Piston is designed to let users decide which math abstractions they want to use.

The Piston project uses a small library [vecmath](https://github.com/PistonDevelopers/vecmath) for ecosystem integration.
This library is often used by authors that contribute to the Piston ecosystem. 

A common pattern is to add a `math` module in your library and reexport/rename the functions that are needed.
This allows one to add extra functions that are useful for that particular library.

### Converting custom types

It is common to use the conversion trait `Into` when supporting user specified types.
This allows developers to choose their own custom types for convenience.

Here is an example from [Piston-Graphics](https://github.com/PistonDevelopers/graphics):

```rust
pub fn rectangle<R: Into<types::Rectangle>, G>(color: types::Color,
                                               rect: R,
                                               transform: math::Matrix2d,
                                               g: &mut G) {
    ...
}
```
