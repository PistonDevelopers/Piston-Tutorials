//! Game board logic.

use std::fs::read_to_string;

/// Size of game board.
const SIZE: usize = 9;

/// Stores information for a single `Gameboard` cell
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cell {
    pub value: u8,
    pub loaded: bool,
    pub invalid: bool,
}

/// Stores game board information.
#[derive(Debug, PartialEq)]
pub struct Gameboard {
    /// Stores the content of the cells. `0` is an empty cell.
    pub cells: [[Cell; SIZE]; SIZE],
    /// Whether or not the puzzle is completed
    pub completed: bool,
}

impl Gameboard {
    /// Creates a new game board.
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[Cell::default(); SIZE]; SIZE],
            completed: false,
        }
    }

    /// Load a new game board from the SDM file in `filename`
    pub fn load_sdm(filename: &str) -> Self {
        let data = read_to_string(filename).expect("failed to read SDM file");
        let mut cells = [[Cell::default(); SIZE]; SIZE];
        let mut row = 0;
        let mut col = 0;
        for c in data.chars() {
            if col == SIZE {
                col = 0;
                row += 1;
            }
            let value = c.to_digit(10).unwrap() as u8;
            cells[row][col] = Cell {
                value,
                loaded: value != 0,
                invalid: false,
            };
            col += 1;
        }
        Self {
            cells,
            completed: false,
        }
    }

    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]].value {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if !self.cells[ind[1]][ind[0]].loaded {
            self.validate(ind, val);
            self.cells[ind[1]][ind[0]].value = val;
        }
        // check for puzzle completion
        self.completed = self
            .cells
            .iter()
            .flatten()
            .all(|cell| !cell.invalid && cell.value != 0);
    }

    /// validate the `val` to be put into `ind`
    fn validate(&mut self, ind: [usize; 2], val: u8) {
        let [b, a] = ind;
        if val == 0 {
            self.cells[a][b].invalid = false;
            return;
        }
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

    /// Build a `Gameboard` from an array of `Cell` values
    pub fn from_cells(cells: [[u8; SIZE]; SIZE]) -> Gameboard {
        let mut ret = Gameboard::new();
        for (i, row) in cells.iter().enumerate() {
            for (j, &col) in row.iter().enumerate() {
                ret.cells[i][j] = Cell {
                    value: col,
                    loaded: col != 0,
                    invalid: false,
                };
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sdm() {
        let got = Gameboard::load_sdm("puzzles/puzzle.sdm");
        let want = Gameboard::from_cells([
            [0, 1, 6, 4, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 9, 0, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 6, 2],
            [0, 7, 0, 2, 3, 0, 1, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 3, 0, 8, 7, 0, 4, 0],
            [9, 6, 0, 0, 0, 0, 0, 0, 5],
            [0, 0, 0, 8, 0, 0, 0, 0, 7],
            [0, 0, 0, 0, 0, 6, 8, 2, 0],
        ]);
        assert_eq!(got, want);
    }
}
