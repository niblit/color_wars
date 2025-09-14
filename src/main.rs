use color_wars::prelude::*;
use colored::{ColoredString, Colorize};

fn main() {
    let mut board = Board::setup(
        Coordinates::new(2, 2),
        Coordinates::new(1, 3)
    );


    while !board.is_game_over() {
        for row in board.grid() {
            format_row_to_print(row);
        };

        let mut row = String::new();
        std::io::stdin().read_line(&mut row).expect("STDIN error");
        let row: usize = row.trim().parse().unwrap();

        let mut column = String::new();
        std::io::stdin().read_line(&mut column).expect("STDIN error");
        let column: usize = column.trim().parse().unwrap();

        let play = Coordinates::new(row, column);

        board = board.make_move(play);
    }
}

fn format_row_to_print(row: [Square; BOARD_SIZE]) {
    for square in row {
        print!("{}", format_square_to_print(square));
    };
    println!();
}

fn format_square_to_print(square: Square) -> ColoredString {
    if let Some(player) = square.owner() {
        let value = square.value().to_string();
        match player {
            Player::Red => value.red(),
            Player::Blue => value.blue()
        }
    }
    else {
        String::from("-").white()
    }
}
