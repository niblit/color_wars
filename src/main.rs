use color_wars::prelude::*;

fn main() {
    let board = Board::new(Player::Red);
    print_board(&board);

    let mut valid_placements = Vec::new();
    for r in 0..BOARD_SIZE {
        for c in 0..BOARD_SIZE {
            valid_placements.push(Coordinates::new(r, c));
        }
    }

    let red_placement = input_coordinates(&valid_placements);

    if let Some(index) = valid_placements
        .iter()
        .position(|value| *value == red_placement)
    {
        valid_placements.remove(index);
    }

    let blue_placement = input_coordinates(&valid_placements);

    let mut board = Board::setup(red_placement, blue_placement);

    while !board.is_game_over() {
        print_board(&board);

        println!("Engine evaluation:");
        let (score, best_move) = search(&board, board.turn());
        println!("Current score: {}", score);
        println!("The best move is: {},{}", best_move.unwrap().column(), best_move.unwrap().row());

        // let play = crate::input_coordinates(&board.get_valid_moves());
        let play = best_move.unwrap();

        board = board.make_move(play);
    }

    print_board(&board);
    println!("Game over!, {:?} wins", board.turn().opponent());
}
