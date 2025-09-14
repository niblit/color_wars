mod board;
mod cli;
mod coordinates;
mod engine;
mod player;
mod square;

pub mod prelude {
    pub use crate::BOARD_SIZE;
    pub use crate::board::Board;
    pub use crate::cli::{input_coordinates, print_board};
    pub use crate::coordinates::Coordinates;
    pub use crate::engine::search;
    pub use crate::player::Player;
    pub use crate::square::Square;
}

pub const BOARD_SIZE: usize = 5;
