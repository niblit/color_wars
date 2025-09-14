mod board;
mod player;
mod square;
mod coordinates;

pub const BOARD_SIZE: usize = 5;

pub mod prelude {
    pub use crate::board::Board;
    pub use crate::player::Player;
    pub use crate::square::Square;
    pub use crate::coordinates::Coordinates;
    pub use crate::BOARD_SIZE;
}
