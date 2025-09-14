use std::io::Write;
use crate::{board::Board, coordinates::Coordinates, player::Player, square::Square, BOARD_SIZE};
use colored::{ColoredString, Colorize};

pub fn input_coordinates(valid_moves: &[Coordinates]) -> Coordinates {
    loop {
        let column = input_usize("Column: ");
        let row = input_usize("Row: ");

        if (0..BOARD_SIZE).contains(&row) && (0..BOARD_SIZE).contains(&column) {
            let placement = Coordinates::new(row, column);
            if valid_moves.contains(&placement) {
                return placement;
            }
            else {
                println!("Invalid move!");
            }
        }
        else {
            println!("Coordinates not in range!");
        }
    }
}

fn input_usize(message: &str) -> usize {
    loop {
        print!("{message}");
        std::io::stdout().flush().unwrap();

        let mut value = String::new();
        std::io::stdin().read_line(&mut value).expect("STDIN error");
        let value = value.trim().parse::<usize>();

        if let Ok(value) = value {
            return  value;
        }
        else {
            println!("Invalid input");
        }
    }
}

pub fn print_board(board: &Board) {
    let mut lower_coordinates_hint = String::from("   ");
    for i in 0..BOARD_SIZE {
        lower_coordinates_hint = format!("{lower_coordinates_hint} {} ", i);
    }
    println!("{lower_coordinates_hint}");

    for (i, &row) in board.grid().iter().enumerate() {
        format_row_to_print(i, row);
    };

    println!("{lower_coordinates_hint}");

    println!(
        "Turn: {}",
        match board.turn() {
            Player::Red => "   ".on_red(),
            Player::Blue => "   ".on_blue(),
        }
    );
}

fn format_row_to_print(i: usize, row: [Square; BOARD_SIZE]) {
    print!(" {i} ");
    for square in row {
        print!("{}", format_square_to_print(square));
    };
    println!(" {i} ");
}

fn format_square_to_print(square: Square) -> ColoredString {
    if let Some(player) = square.owner() {
        let value = format!(" {} ", square.value());
        match player {
            Player::Red => value.on_red(),
            Player::Blue => value.on_blue()
        }
    }
    else {
        String::from(" ▪ ").on_black()
    }
}
