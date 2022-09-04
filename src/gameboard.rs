//! Game board logic.

use std::fs::read_to_string;

/// Size of game board.
const SIZE: usize = 9;

/// Stores game board information.
#[derive(Debug, PartialEq)]
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

    /// Load a new game board from the SDM file in `filename`
    pub fn load_sdm(filename: &str) -> Self {
        let data = read_to_string(filename).expect("failed to read SDM file");
        let mut cells = [[0; SIZE]; SIZE];
        let mut row = 0;
        let mut col = 0;
        for c in data.chars() {
            if col == SIZE {
                col = 0;
                row += 1;
            }
            cells[row][col] = c.to_digit(10).unwrap() as u8;
            col += 1;
        }
        Self { cells }
    }

    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
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
        self.cells[ind[1]][ind[0]] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sdm() {
        let got = Gameboard::load_sdm("puzzles/puzzle.sdm");
        let want = Gameboard {
            cells: [
                [0, 1, 6, 4, 0, 0, 0, 0, 0],
                [2, 0, 0, 0, 0, 9, 0, 0, 0],
                [4, 0, 0, 0, 0, 0, 0, 6, 2],
                [0, 7, 0, 2, 3, 0, 1, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 3],
                [0, 0, 3, 0, 8, 7, 0, 4, 0],
                [9, 6, 0, 0, 0, 0, 0, 0, 5],
                [0, 0, 0, 8, 0, 0, 0, 0, 7],
                [0, 0, 0, 0, 0, 6, 8, 2, 0],
            ],
        };
        assert_eq!(got, want);
    }
}
