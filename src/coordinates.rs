//! Defines the `Coordinates` type for locating squares on the game board.
use crate::{BOARD_COLUMN_SIZE, BOARD_ROW_SIZE};

/// Points to a specific square on the board using row and column indices.
///
/// This struct guarantees that its coordinates are always within the bounds
/// of the board, as defined by `BOARD_ROW_SIZE` and `BOARD_COLUMN_SIZE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    /// Creates a new `Coordinates` instance.
    /// row and column are always zero-based indexes.
    ///
    /// # Panics
    ///
    /// This function will panic if `row` or `column` are outside the board dimensions.
    pub fn new(row: usize, column: usize) -> Self {
        assert!(row < BOARD_ROW_SIZE);
        assert!(column < BOARD_COLUMN_SIZE);
        Self { row, column }
    }

    /// Returns the zero-based row index.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns the zero-based column index.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Returns a list of valid orthogonally adyacent neighbor coordinates.
    pub fn neighbors(&self) -> Vec<Coordinates> {
        let mut neighbors = Vec::with_capacity(4);

        // Check North
        if self.row > 0 {
            neighbors.push(Coordinates::new(self.row - 1, self.column));
        }
        // Check South
        if self.row < (BOARD_ROW_SIZE - 1) {
            neighbors.push(Coordinates::new(self.row + 1, self.column));
        }
        // Check West
        if self.column > 0 {
            neighbors.push(Coordinates::new(self.row, self.column - 1));
        }
        // Check East
        if self.column < (BOARD_COLUMN_SIZE - 1) {
            neighbors.push(Coordinates::new(self.row, self.column + 1));
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_coordinates_valid() {
        for row in 0..BOARD_ROW_SIZE {
            for column in 0..BOARD_COLUMN_SIZE {
                let coordinates = Coordinates::new(row, column);

                assert_eq!(coordinates.row, row);
                assert_eq!(coordinates.column, column);
            }
        }
    }

    #[test]
    fn coordinates_getters() {
        for row in 0..BOARD_ROW_SIZE {
            for column in 0..BOARD_COLUMN_SIZE {
                let coordinates = Coordinates::new(row, column);

                assert_eq!(coordinates.row(), row);
                assert_eq!(coordinates.column(), column);
            }
        }
    }

    #[test]
    #[should_panic]
    fn new_coordinates_invalid_row() {
        // This should panic because the row is out of bounds.
        Coordinates::new(BOARD_ROW_SIZE, 0);
    }

    #[test]
    #[should_panic]
    fn new_coordinates_invalid_column() {
        // This should panic because the column is out of bounds.
        Coordinates::new(0, BOARD_COLUMN_SIZE);
    }

    #[test]
    #[should_panic]
    fn invalid_coordinates() {
        Coordinates::new(BOARD_ROW_SIZE, BOARD_COLUMN_SIZE);
    }
}
