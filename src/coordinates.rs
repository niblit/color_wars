use crate::BOARD_SIZE;

// Points to a square on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    row: usize,
    column: usize
}

impl Coordinates {
    pub fn new(row: usize, column: usize) -> Self {
        assert!(row < BOARD_SIZE);
        assert!(column < BOARD_SIZE);
        Self {
            row,
            column
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
