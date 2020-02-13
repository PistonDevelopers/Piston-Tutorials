# Chapter 2 - World Map #

By the end of this chapter we will have taken our plain, simple window and create a World Map which 
we can play with.

The Map will consist of a 16x16 grid of tiles, and a tile will have a set of rules associated with it, 
such as whether or not the player can pass through it.

Before we start on making tiles, let's clean up some of the code we already have. 
Firstly, since we are using colours so much, lets define them specifically:
``` rust
type Colour = [f32; 4];

``` 
In essence this allows us to use the word `Colour` when we want to have an array of 4 32-bit floating point numbers.

Secondly, we should make all of our previously defined colours as constants so that they can be used anywhere within the program by moving them from main, and placing them at the top of our program:

```rust
const RED: Colour = [1.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 1.0, 0.0, 1.0];
const BLUE: Colour = [0.0, 0.0, 1.0, 1.0];
const WHITE: Colour = [1.0; 4];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
```
Finally we should scale the size of our world, by the size of our window and the size of each tile, which means that we will also define the size of the window and the size of each tile:

```rust
const WINDOW_SIZE: i32 = 512;
const PIXEL_SIZE: f64 = 32.0;
const WORLD_SIZE: i32 = WINDOW_SIZE / PIXEL_SIZE as i32;
```

The last line is saying that we want to cast `PIXEL_SIZE` as an `i32` number so that we can use it in the calculation.

## Tiles ##

Now that that has been set up we can now define what a `Tile` is!
For now, a `Tile` is a `struct` that contains a single variable:
	*`colour` which is of type `Colour` (as we previously defined) and determines what colour we should display it as
	
So now we have defined what the `Tile struct`, we will need to implement how we can use it like so:

```rust
impl Tile {
    pub fn empty() -> Self {
        Tile {
            colour: WHITE,
        }
    }

    pub fn wall() -> Self {
        Tile {
            colour: BLACK,
        }
    }
}
```
This is simply saying that there are 2 types of tiles, `empty` and `wall` which correspond to an empty space or a wall respectively, and they also have different colours to represent that.

Finally to finish off our tiles, we will need to derive the `Clone` trait for our `struct Tile`, we can do that by adding the following line above our `struct` definition:
```rust
#[derive(Clone)]
```

## Map ##
So now we have tiles which we can use to fill up our map with, but what is a map? It's a 2D array of 
`Tiles`, or a `Vec` (vector) of `Vec` of `Tile`. Like how we defined the `Colour` type, we can do tthe same for the `Map`:

```rust
type Map = Vec<Vec<BLANK>>;
```

For simplicity's sake, lets make a function (`fn`) to initialise our map, called `make_map()` and it returns a `Map`. Fill the body of your function with the following:

```rust
let mut map = vec![vec![Tile::empty(); WORLD_SIZE as usize]; WORLD_SIZE as usize];
    map[WORLD_SIZE as usize / 2][WORLD_SIZE as usize / 2] = Tile::wall();
    map
```

What the first line does is define a mutable variable map which is made of a vector `vec!` of vectors which is initialised with `WORLD_SIZE` many `empty` tiles.
The Second line just sets the middle square to be a `wall` so that we can see that everything is working correctly.
Finall the last line allows us to return this newly created `map`.

## Rendering the Map ##

We are nearly done now, all thats left is to create and output our map!
To start with we will need to create an instance of our map in our `main()` function:

```rust
//TODO
let map = BLANK; 
```

Now in our render function that we had earlier, we want to loop through from `0` to `WORLD_SIZE` twice so that we can access all of our squares:

``` rust

for i in BLANK .. BLANK {
	for j in BLANK .. BLANK {
	
	}
}
```

We have the loop structure set up so that we can access every tile in our world, now we need to decide what to do with each tile.
Each tile is an instance of a `graphics::Rectangle`.
To `draw` a rectangle, you will see in that you need have it's position. We can define that like so:

```rust
let pos: [f64;4] = PIXEL_SIZE * i as f64,
	PIXEL_SIZE * j as f64,
	PIXEL_SIZE * (i + 1) as f64,
	PIXEL_SIZE * (j + 1) as f64,
]
```
The code above defines the position of the corners of our tile. Now that we have that, we can actually draw our tile:

```rust
graphics::Rectangle::new(BLANK.colour).draw(
                            BLANK,
                            &c.draw_state,
                            c.transform,
                            g,
                        );
```
Try to think about what the first `BLANK` here represents. We want it to be the colour of the `Tile` at that `i`, `j` coordinate in our `map`. The second `BLANK` is the `pos` of our `Tile`.

When you run this, your window should now be entirely white with a single black square in it meaning that our map has been correctly rendered!

## Chapter 3 - Player ## 

Follow the link to go to Chapter 3, which goes through creating a player character.

[Chapter 3 - Player](Chapter_3-Player.md)
