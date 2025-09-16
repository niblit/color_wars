//! Contains all functions for interacting with the user via the Command Line Interface (CLI).
//!
//! This module is responsible for two main tasks:
//! 1. Printing a colorized, human-readable representation of the game board.
//! 2. Prompting the user for input and validating their moves.
use crate::{board::{ Board}, coordinates::Coordinates, player::Player, square::Square, BOARD_COLUMN_SIZE, BOARD_ROW_SIZE};
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
        // Prompt for and parse row and column numbers.
        let row = input_usize("Row: ");
        let column = input_usize("Column: ");

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
    println!("{}", format_board(board));
}

fn format_board(board: &Board) -> ColoredString {
    let mut formatted_board = format_column_coordinates_hint();
    for row_index in 0..BOARD_ROW_SIZE {
        let formatted_row = format_single_row(row_index, board.grid()[row_index]);
        formatted_board = format!("{formatted_board}{formatted_row}\n").into();

    }
    formatted_board = format!("{formatted_board}{}", format_column_coordinates_hint()).into();

    let formatted_turn = format_turn_bar(board.turn());

    formatted_board = format!("{formatted_board}{formatted_turn}").into();

    formatted_board
}

fn format_column_coordinates_hint() -> ColoredString {
    let empty_pad = String::from("   ").on_black();
    let mut column_coordinates_hint: ColoredString = empty_pad.clone();

    for column_index in 0..BOARD_COLUMN_SIZE {
        let formatted_index = format!(" {} ", column_index).white().on_black();
        column_coordinates_hint = format!("{column_coordinates_hint}{formatted_index}").into();
    }

    column_coordinates_hint = format!("{column_coordinates_hint}{empty_pad}\n").into();

    column_coordinates_hint
}

fn format_turn_bar(player: Player) -> ColoredString {
    let turn = format_turn(player);
    let mut turn_bar = String::new().into();
    for _ in 0..(BOARD_COLUMN_SIZE + 2) {
        turn_bar = format!("{turn_bar}{}", turn.clone()).into();
    }

    turn_bar
}

fn format_turn(player: Player) -> ColoredString {
    let turn = String::from(" ▪ ").black();
    let turn = match player {
        Player::Red => turn.on_red(),
        Player::Blue => turn.on_blue(),
    };

    turn
}

// Formats a single row of the board, including side coordinate hints.
fn format_single_row(i: usize, row: [Square; BOARD_COLUMN_SIZE]) -> ColoredString {
    let mut formatted_row: ColoredString = String::new().into();

    let row_coordinates_hint = format!(" {i} ").white().on_black();
    formatted_row = format!("{formatted_row}{row_coordinates_hint}").into();

    for square in row {
        formatted_row = format!("{formatted_row}{}", format_single_square(square)).into();
    }
    formatted_row = format!("{formatted_row}{row_coordinates_hint}").into();

    formatted_row
}

// Formats a single square into a colored string for display.
fn format_single_square(square: Square) -> ColoredString {
    if let Some(player) = square.owner() {
        let value = format!(" {} ", square.value()).black();
        match player {
            Player::Red => value.on_red(),
            Player::Blue => value.on_blue(),
        }
    } else {
        String::from(" ▪ ").white().on_black()
    }
}
