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
