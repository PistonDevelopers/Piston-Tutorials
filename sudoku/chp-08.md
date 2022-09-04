# Sudoku tutorial
by Brent Westbrook, 2022

## Chapter 8

As mentioned in the previous chapter, the main thing left to do is actually
check the input to the puzzle to make sure it represents a valid solution. There
are two major times to check cells. First, we can check each cell as it is put
in. And second, we want to verify that the finished puzzle is correct. Since the
second case is just a special case of the first, we'll focus our effort on the
first case and think about adding some kind of "Solved!" message in the second
case later. We'll make both of these checks in our `Gameboard::set` method. In
both cases, we'll call out to a helper method called `validate`:

```rust
/// validate the `val` to be put into `ind`
fn validate(&mut self, ind: [usize; 2], val: u8) {
	let [b, a] = ind;
	// check row
	for i in 0..SIZE {
		if i == a {
			continue;
		}
		if self.cells[a][i].value == val {
			self.cells[a][b].invalid = true;
			return;
		}
	}
	// check col
	for i in 0..SIZE {
		if i == b {
			continue;
		}
		if self.cells[i][b].value == val {
			self.cells[a][b].invalid = true;
			return;
		}
	}
	// check box
	let (row, col) = (a / 3, b / 3);
	for i in 3 * row..3 * row + 3 {
		for j in 3 * col..3 * col + 3 {
			if i == a && j == b {
				continue;
			}
			if self.cells[i][j].value == val {
		self.cells[a][b].invalid = true;
				return;
			}
		}
	}
	self.cells[a][b].invalid = false;
}
```

As you can see, this is updating another field we have to add to our `Cell`
implementation:

```rust
/// Stores information for a single `Gameboard` cell
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub value: u8,
    pub loaded: bool,
    pub invalid: bool,
}
```

I'll leave it as an exercise to propagate this change since it should be
virtually identical to what we did before with `loaded`.

Now we just need to use the new `invalid` property to color those cells
differently. We'll add two more fields on `GameboardViewSettings` called
`invalid_cell_background_color` and `invalid_selected_cell_background_color` so
that we can still see our selection when a `Cell` is invalid. Here's the new `GameboardViewSettings`:

```rust
/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Selected cell background color.
    pub selected_cell_background_color: Color,
    /// Loaded cell background color.
    pub loaded_cell_background_color: Color,
    /// Invalid cell background color.
    pub invalid_cell_background_color: Color,
    /// Invalid selected cell background color.
    pub invalid_selected_cell_background_color: Color,
    /// Text color.
    pub text_color: Color,
}
```

and the new `new` function:

```rust
impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            loaded_cell_background_color: [1.0, 1.0, 1.0, 1.0],
            invalid_cell_background_color: [1.0, 0.0, 0.0, 1.0],
            invalid_selected_cell_background_color: [1.0, 0.0, 0.5, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}
```

and here we can update our background-drawing code in the `draw` method:

```rust
// Draw loaded and invalid cell backgrounds
for i in 0..9 {
	for j in 0..9 {
		if controller.gameboard.cells[i][j].loaded {
			color_cell(
				settings,
				[j, i],
				settings.loaded_cell_background_color,
				c,
				g,
			);
		} else if controller.gameboard.cells[i][j].invalid {
			color_cell(
				settings,
				[j, i],
				settings.invalid_cell_background_color,
				c,
				g,
			);
		}
	}
}

// Draw selected cell background.
if let Some(ind) = controller.selected_cell {
	let cell = controller.gameboard.cells[ind[1]][ind[0]];
	let color = if !cell.loaded {
		if !cell.invalid {
			settings.selected_cell_background_color
		} else {
			settings.invalid_selected_cell_background_color
		}
	} else {
		settings.loaded_cell_background_color
	};
	color_cell(settings, ind, color, c, g);
};
```

### Detecting victory
