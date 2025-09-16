//! Contains all functions for interacting with the user via the Command Line Interface (CLI).
//!
//! This module is responsible for two main tasks:
//! 1. Printing a colorized, human-readable representation of the game board.
//! 2. Prompting the user for input and validating their moves.
use crate::{BOARD_ROW_SIZE, BOARD_COLUMN_SIZE, board::Board, coordinates::Coordinates, player::Player, square::Square};
use colored::{ColoredString, Colorize};
use std::io::Write;

/// Prompts the user to enter coordinates and loops until a valid move is chosen.
///
/// This function ensures that the user's input corresponds to a coordinate
/// that is both within the board's bounds and is listed in the `valid_moves` slice.
///
/// # Arguments
/// * `valid_moves` - A slice of `Coordinates` representing all legal moves for the current player.
///
/// # Returns
/// The `Coordinates` of the valid move selected by the user.
pub fn input_coordinates(valid_moves: &[Coordinates]) -> Coordinates {
    loop {
        // Prompt for and parse column and row numbers.
        let column = input_usize("Column: ");
        let row = input_usize("Row: ");

        // Validate that the coordinates are on the board.
        if (0..BOARD_ROW_SIZE).contains(&row) && (0..BOARD_COLUMN_SIZE).contains(&column) {
            let placement = Coordinates::new(row, column);

            // Validate that the chosen square is a legal move.
            if valid_moves.contains(&placement) {
                return placement;
            } else {
                println!("Invalid move!");
            }
        } else {
            println!("Coordinates not in range!");
        }
    }
}

// A private helper function to reliably get a `usize` from standard input.
fn input_usize(message: &str) -> usize {
    loop {
        print!("{message}");

        // Ensure the prompt message is displayed before waiting for input.
        std::io::stdout().flush().unwrap();

        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("STDIN error");
        let value = value.trim().parse::<usize>();

        if let Ok(value) = value {
            return value;
        } else {
            println!("Invalid input");
        }
    }
}

/// Prints a color-coded representation of the entire board to the console.
///
/// It displays the grid, coordinate hints, and the current player's turn.
pub fn print_board(board: &Board) {
    let mut lower_coordinates_hint = String::from("   ");
    for i in 0..BOARD_COLUMN_SIZE {
        lower_coordinates_hint = format!("{lower_coordinates_hint} {} ", i);
    }
    let column_coordinates = format!("{lower_coordinates_hint}   ");

    let column_coordinates_hint = column_coordinates.white().on_black();

    // Top coordinate hints
    println!("{column_coordinates_hint}");

    // Print each row with its contents and side hints.
    for (i, &row) in board.grid().iter().enumerate() {
        format_row_to_print(i, row);
    }

    // Bottom coordinate hints
    println!("{column_coordinates_hint}");

    // Display the current player's turn.
    println!(
        "Turn: {}",
        match board.turn() {
            Player::Red => " ▪ ".black().on_red(),
            Player::Blue => " ▪ ".black().on_blue(),
        }
    );
}

// Formats and prints a single row of the board, including side coordinate hints.
fn format_row_to_print(i: usize, row: [Square; BOARD_COLUMN_SIZE]) {
    let row_coordinates_hint = format!(" {i} ").white().on_black();
    print!("{row_coordinates_hint}");
    for square in row {
        print!("{}", format_square_to_print(square));
    }
    println!("{row_coordinates_hint}");
}

// Formats a single square into a colored string for display.
fn format_square_to_print(square: Square) -> ColoredString {
    if let Some(player) = square.owner() {
        let value = format!(" {} ", square.value());
        match player {
            Player::Red => value.black().on_red(),
            Player::Blue => value.black().on_blue(),
        }
    } else {
        String::from(" ▪ ").on_black()
    }
}
