//! A Rust implementation of the "Color Wars" tactical board game engine.
//!
//! This crate provides all the necessary types and logic to run a game of Color Wars.
//! It includes the game board, player and square representations, and the core
//! move execution logic that handles the "pop" and chain reaction mechanics.
//!
//! The easiest way to get started is by importing the commonly used items
//! from the `prelude` module.
//!
//! # Example
//!
//! ```
//! // Import the essentials
//! use color_wars::prelude::*;
//!
//! // Create a new board with a standard setup
//! let board = Board::setup(Coordinates::new(1, 1), Coordinates::new(2, 2));
//!
//! // Print the initial state
//! print_board(&board);
//!
//! // Get the first player's valid moves
//! let moves = board.get_valid_moves();
//!
//! // Make the first move
//! if let Some(first_move) = moves.get(0) {
//!     let new_board = board.make_move(*first_move);
//!     println!("After one move:");
//!     print_board(&new_board);
//! }
//! ```

mod board;
mod cli;
mod coordinates;
mod engine;
mod player;
mod square;

/// The prelude module provides convenient access to the most common types and functions.
///
/// By importing everything from this module (`use color_wars::prelude::*`), you can
/// easily access `Board`, `Player`, `Coordinates`, and other essential items.
pub mod prelude {
    pub use crate::{BOARD_ROW_SIZE, BOARD_COLUMN_SIZE};
    pub use crate::board::Board;
    pub use crate::cli::{input_coordinates, print_board};
    pub use crate::coordinates::Coordinates;
    pub use crate::engine::search;
    pub use crate::player::Player;
    pub use crate::square::Square;
}

/// The number of rows on the game board. Must be 2 or greater.
pub const BOARD_ROW_SIZE: usize = 5;

/// The number of columns on the game board. Must be 2 or greater.
pub const BOARD_COLUMN_SIZE: usize = 5;
