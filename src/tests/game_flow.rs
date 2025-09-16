use color_wars::prelude::*;

#[test]
fn test_a_few_turns() {
    // 1. Setup the board
    let mut board = Board::setup(Coordinates::new(0, 0), Coordinates::new(4, 4));
    assert!(!board.is_game_over());
    assert_eq!(board.turn(), Player::Red);

    // 2. Player 1 (Red) makes a move (a pop)
    let red_moves = board.get_valid_moves();
    assert_eq!(red_moves.len(), 1);
    board = board.make_move(red_moves[0]); // Pop at (0,0)

    // Check state after Red's move
    assert_eq!(board.turn(), Player::Blue);
    assert_eq!(board.grid()[0][0].owner(), None); // Popped square is empty
    assert_eq!(board.grid()[0][1].owner(), Some(Player::Red)); // Neighbor is captured
    assert_eq!(board.grid()[1][0].owner(), Some(Player::Red)); // Neighbor is captured

    // 3. Player 2 (Blue) makes a move (a pop)
    let blue_moves = board.get_valid_moves();
    assert_eq!(blue_moves.len(), 1);
    board = board.make_move(blue_moves[0]); // Pop at (4,4)

    // Check state after Blue's move
    assert_eq!(board.turn(), Player::Red);
    assert_eq!(board.grid()[4][4].owner(), None); // Popped square is empty
    assert_eq!(board.grid()[4][3].owner(), Some(Player::Blue)); // Neighbor is captured
    assert_eq!(board.grid()[3][4].owner(), Some(Player::Blue)); // Neighbor is captured

    // The game continues...
    assert!(!board.is_game_over());
}
