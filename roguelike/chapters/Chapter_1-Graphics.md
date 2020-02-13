# Chapter 1 - Graphics  #

By the end of this chapter you will have a simple window which will be the
foundation of our game!

## Window  ##
We want to create a window, and have it display a blank, white screen. To start
with let's add the piston crate.

On your terminal, type the following:
`cargo add piston`

If you look into you `Cargo.toml` file, underneath `[dependencies]`, you should
now see: `piston = "0.49.0"` (or whatever the most recent version of piston
is).

Following on from this, you will want to add the GlutinWindow crate to your
project like so:

`cargo add pistoncore-glutin_window`

Which again will add the most recent version of `glutin_window` to your
dependency list.

Now we can create a window which will allow us to run the game!
To do this, **you** will need to create some Settings for your window:

``` rust
//TODO
let settings = WindowSettings::new(BLANK, BLANK).exit_on_exc(true);
```
Look through the generated documents and it should guide you on what to replace
the `BLANK` spaces with (**hint: there are 2 BLANKS for a reason!**).

Now this is not going to run, even though we have added the dependencies to our
`Cargo.toml` file, we aren't actually using them, so let's do that!
Add the following lines of code to the top of your file:

``` rust
extern crate piston;
use piston::WindowSettings;
```

Now that we have created our WindowSettings, let's create our Window itself.
To do this, we will need to add and use the `glutin_window` crate by adding the
following lines to the top of your file:

``` rust
extern crate glutin_window;
use glutin_window::GlutinWindow;
```

Now that the crate is available to use, let's actually use it and do something
with it!

To do this we are now going to want to create a window itself:

`let mut window: GlutinWindow = settings.build().expect("Could not create window");`

What this does is create a Window and, if there is an error creating is, will
report the error message, otherwise it will return the window.

**NB:** that we are using a `mut window`, the reason for using the keyword `mut`
is because by default, everything in Rust is *immutable* which means that it
cannot be changed, but with our window, we are going to want to change it
constantly so that it can reflect the current state of our game and so it needs
to be *mutable* hence the `mut` keyword.

If you run this now you will see that the window appears and then disappears
immediately! This happens because the program finishes and so it closes
everything.

## Colours  ##
Now that we have a window, we will want to add some colours to it so that it
looks nicer than a simple black screen. To start with, let's define a colour
that you want to be the background colour. In Rust this is represented by
an array of 4 numbers, which represent the R, G, B and Opacity channels.
For convenience sake, I am going to provide you will 4 basic colours, and you
can play around with values yourself if there is a particular colour you
prefer.

``` rust
let RED = [1.0, 0.0, 0.0, 1.0];
let GREEN = [0.0, 1.0, 0.0, 1.0];
let BLUE = [0.0, 0.0, 1.0, 1.0];
let WHITE = [1.0;4];
```


The last line, `let WHITE = [1.0;4];` means that we want and array which stores the
value `1.0`, `4` times.

## Events ##
We now have a set of colours that we want to display as our background, so lets
actually do that.

To start with, lets set up the structure so that we can continually render our
window, to do this we will need to `use` the following from the `piston`
library:
+ `RenderEvent`

We will also need to use some classes from the piston event loop like so:
`use piston::event_loop::{EventSettings, Events, EventLoop};`

Now we have access to game `Events`, we want to create a `new` event. If we
look into the docs we can see that we can create an `Event` by using its
constructor function `new` (**HINT:** the same thing can be done for
`EventSettings`).

`//TODO
let mut events = Events::BLANK();`

We now have an object that will allow us to get and handle a variety of events,
so lets do that:

`//TODO
while let Some(e) = events.next(&mut BLANK){}`

Again, fill in the `BLANK` with what's appropriate by looking at the docs.
This now will create an infinite loop which will constantly poll the system
getting events for us.

The `Event` that we are most interested in is the `RenderEvent` since it allows
us to change what's on the screen.
How we handle this is by checking to see whether or not our `Event` is in fact
a `RenderEvent` by using an if statement like this within the `while`loop:

`if let Some(r) = e.render_args(){}`

This creates an object that will allow us to

## Adding Graphics ##

Now execute the following line in your terminal:
`cargo add piston2d-graphics
cargo add piston2d-opengl_graphics`
To automatically add the crates we need for creating images to put onto our
window to the dependency list.

Now that these dependencies have been added, lets bring them into the game:
`extern crate graphics;
extern crate opengl_graphics;`

This time we only need to `use` `GlGraphics` and `OpenGL` from
`opengl_graphics`, so make sure to add that at the top of your file.

Now that we can, let's go about using these two things, to start with let's
define a variable to be the version of `OpenGL` that we want to use:
`let opengl = OpenGL::BLANK;`

Replace `BLANK` with whatever version you want to use, for example `V3_2`

Now we want to create a `new` object that will handle all of our graphics and uses
`OpenGL`:
`let mut gl = GlGraphics::BLANK();`

Use the docs to fill in the `BLANK` space.

## Rendering ##

Finally we are now able to change the background of our window! The code to do
this sits within the `if` statement we made earlier for the `RenderEvents`:

``` rust
gl.draw(r.viewport(), |_c, g| {
    graphics::clear(YOUR_COLOUR, g);
});
```

If you replace `YOUR_COLOUR` with whatever colour you want run the code!

## Chapter 2 - World Map ##

Follow the link to go to Chapter 2, which allows us to create a world map for our game.

[Chapter 2 - World Map](Chapter_2-World-Map.md)
