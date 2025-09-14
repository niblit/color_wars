mod board;
mod player;
mod square;
mod coordinates;
mod cli;

pub mod prelude {
    pub use crate::board::Board;
    pub use crate::player::Player;
    pub use crate::square::Square;
    pub use crate::coordinates::Coordinates;
    pub use crate::BOARD_SIZE;
    pub use crate::cli::{input_coordinates, print_board};
}

pub const BOARD_SIZE: usize = 5;
