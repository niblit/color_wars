use color_wars::prelude::*;

fn main() {
    // --- Initial Setup Phase ---
    // This phase handles the placement of the first two pieces on the board.
    let board = Board::new(Player::Red);
    print_board(&board);

    // Create a list of all possible coordinates for the initial piece placements.
    let mut valid_placements = Vec::new();
    for row in 0..BOARD_ROW_SIZE {
        for column in 0..BOARD_COLUMN_SIZE {
            valid_placements.push(Coordinates::new(row, column));
        }
    }

    // Get Player 1 (Red)'s starting position.
    println!("Red player, choose your starting square.");
    let red_placement = input_coordinates(&valid_placements);

    // Remove Red's choice so Blue cannot pick the same square.
    if let Some(index) = valid_placements
        .iter()
        .position(|value| *value == red_placement)
    {
        valid_placements.remove(index);
    }

    // Get Player 2 (Blue)'s starting position.
    println!("Blue player, choose your starting square.");
    let blue_placement = input_coordinates(&valid_placements);

    // Create the board with the starting pieces.
    let mut board = Board::setup(red_placement, blue_placement);

    // --- Main Game Loop ---
    // The game continues as long as neither player has been eliminated.
    while !board.is_game_over() {
        print_board(&board);

        // Use the engine to suggest a move for the current player.
        println!("Engine evaluation:");
        let (score, best_move)= search(&board);
        println!("Current score: {}", score);
        println!("Best move: [{},{}]", best_move.column(), best_move.row());

        // Get the actual move from the user.
        let play = crate::input_coordinates(&board.get_valid_moves());

        board = board.make_move(play);
    }

    // --- Game Over ---
    // Announce the winner. The winner is the opponent of the player whose turn it is,
    // because that player has no moves left.
    print_board(&board);
    println!("Game over!, {:?} wins", board.turn().opponent());
}
