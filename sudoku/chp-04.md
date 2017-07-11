# Sudoku tutorial
by Sven Nilsen, 2017

## Chapter 4

Before we do anything, we will force ourselves to document the code.
Add `#![deny(missing_docs)]` to the top of "src/main.rs" and a doc comment:

```rust
#![deny(missing_docs)]

//! A Sudoku game.

extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
```

The `//!` is used on doc comments that comment the object they are inside.
When commenting something from the outside, you use `///`.

Create a new file in the "src/" directory called "gameboard.rs".
Add the following code:

```rust
//! Game board logic.

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// `0` is an empty cell.
    pub cells: [[u8; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }
}
```

Import the `Gameboard` struct in "main.rs":

```rust
pub use gameboard::Gameboard;

mod gameboard;
```

Create a new file in the "src/" directory called "gameboard_controller.rs".
Add the following code:

```rust
//! Gameboard controller.

use piston::input::GenericEvent;

use Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
```

A controller is an object that handles events and manipulates some data.
The data manipulated by a controller is called a "model".
By separating model and controller you can reuse the model in other projects.

Import `GameboardController` in "main.rs":

```rust
pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;

mod gameboard;
mod gameboard_controller;
```

Create a new file in the "src/" directory called "gameboard_view.rs".
Add the following code:

```rust
//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use GameboardController;

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
}

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
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, c: &Context, controller: &GameboardController, g: &mut G) {

    }
}
```

Import `GameboardView` in "main.rs":

```rust
pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::GameboardView;

mod gameboard;
mod gameboard_controller;
mod gameboard_view;
```

Set up objects and handle like this:

```rust
  let mut events = Events::new(EventSettings::new().lazy(true));
  let mut gl = GlGraphics::new(opengl);

  let gameboard = Gameboard::new();
  let mut gameboard_controller = GameboardController::new(gameboard);
  let gameboard_view_settings = GameboardViewSettings::new();
  let gameboard_view = GameboardView::new(gameboard_view_settings);

  while let Some(e) = events.next(&mut window) {
      gameboard_controller.event(&e);
      if let Some(args) = e.render_args() {
          gl.draw(args.viewport(), |c, g| {
              use graphics::{clear};

              clear([1.0; 4], g);
              gameboard_view.draw(&gameboard_controller.gameboard, &c, g);
          });
      }
  }
```

The call `gameboard_controller.event(&e);` passes events to the controller.

The call `gameboard_view.draw(&gameboard_controller.gameboard, &c, g);`
renders the gameboard.

[Goto Chapter 5](chp-05.md)
