# Chapter 3 - Player #

In this chapter we will finally learn how to create our player that can move around our map!

Before we get started we will need to `use` some more stuff.
Add or update the following lines to the top of your file:

```rust

use piston::input::{Button, ButtonState, Key};
use piston::{ButtonEvent, RenderEvent};

use graphics::character::CharacterCache;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
```

## Object Structure ##

Last chapter we created the `struct Tile`, now we want to do the same thing but for a `character`:

```rust
#[derive(Clone)]
struct Object{
//TODO
}
```

Fill in the above template with the following fields:
	*`x` position which of type `i32`
	*`y` position which of type `i32`
	*`character` of type `char` which represents what the character will appear as on screen
	* `colour` of type `Colour` representing what colour our character should be

We now need to implement our `Object` struct like so:

``` rust
impl Object {
//TODO
    pub fn new(BLANK, BLANK, BLANK, BLANK) -> Self {
        Object {
            BLANK,
            BLANK,
            BLANK,
            BLANK,
        }
    }
}
```

Fill in the `BLANK` spaces with the appropriate values. **(HINT: The fields we defined earlier)**

## Adding a player ##

Now that we have defined what a `Object` is, we can create an instance of it for our `player`.
In the main function add the following:

```rust
let mut player : Object = Object::new(BLANK,BLANK,'@',RED);
```
Where the BLANKS represent the starting position in our map (eg. (0,0)).

## Displaying the Player ##

We now have a player object, so lets go about displaying them on our window. To do this, copy the following lines into your `main()` function:

``` rust
	let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
	.expect("Could not load font");
```

This creates a new `texture_settings` and `GlyphCache` so that we can print characters to our window.

In our render function, underneath where we render the world map, we now want to display our player:

```rust
use graphics::Transformed;
let character = glyphs.character(PIXEL_SIZE, BLANK).unwrap();
graphics::Image::new_color(BLANK).draw(
	character.texture,
	&c.draw_state,
	c.transform.trans(BLANK , BLANK ),
	g,
	);
```

The first `BLANK` represents what `character` our `player` is.
The next `BLANK` represents what the `Colour` of our `player` is.
The last set of blanks represent the `x` and `y` positions of our `player`

## Movement ##

Make sure before starting this section that your code compiles and runs displaying your player on the screen.

Assuming that this works we can now implement the logic that will allow our player to move around the screen!

To do this we need to be able to handle `ButtonEvent`.
Similar to how we were able to define what happens when we encounter a `RenderEvent`, we need to add the following code in our `while` loop:

`if let Some(k) = e.button_args(){}`

In this function we can now decide what to do when we push a button. 
To start with, we should only do something when a button is *pressed* by adding the following line within this `if` statement:

`if k.state == ButtonState::Press{}`

Now within this statement we can add the logic for a button press like so:

``` rust
match k.button {
	Button::Keyboard(Key::Up) => BLANK -= PIXEL_SIZE as i32,
	Button::Keyboard(Key::Down) => player.y += PIXEL_SIZE as i32,
	Button::Keyboard(Key::Left) => player.x -= PIXEL_SIZE as i32,
	Button::Keyboard(Key::Right) => player.x += PIXEL_SIZE as i32,
	_ => (),
}
```

Fill in the `BLANK` spaces with the logic required to move our player about depending on which arrow key is pressed.

Once all this is done you should now have player who can walk around a map!
