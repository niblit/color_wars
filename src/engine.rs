use crate::{board::Board, player::Player, prelude::Coordinates, BOARD_ROW_SIZE, BOARD_COLUMN_SIZE};

// Evaluates the board state from the perspective of a given player.
// Positive score is good for the player, negative is bad.
pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    let mut red_squares = 0usize;
    let mut blue_squares = 0usize;

    for r in 0..BOARD_ROW_SIZE {
        for c in 0..BOARD_COLUMN_SIZE {
            if let Some(owner) = board.grid()[r][c].owner() {
                match owner {
                    Player::Red => {
                        score += 1;
                        red_squares += 1;
                    }
                    Player::Blue => {
                        score -= 1;
                        blue_squares += 1;
                    }
                }
            }
        }
    }

    // P1 wins
    if blue_squares == 0 && red_squares > 0 {
        score = i32::MAX;
    }

    // P2 wins
    if red_squares == 0 && blue_squares > 0 {
        score = -i32::MAX;
    }

    score
}

pub fn search(board: &Board) -> (i32, Coordinates) {
    let depth = 8;

    let mut alpha = -i32::MAX;
    let beta = i32::MAX; // Beta is constant at the root for the maximizer

    let placement_evaluations = board.get_valid_moves();

    // You must have a default best move in case no better move is found
    let mut best_placement = placement_evaluations[0];
    let mut best_score = -i32::MAX;


    for current_placement in placement_evaluations {
        let board_after_move = board.make_move(current_placement);
        // The first recursive call is for the opponent (minimizing player)
        let score = alphabeta(&board_after_move, depth - 1, alpha, beta, board.turn().opponent());

        if score > best_score {
            best_score = score;
            best_placement = current_placement;
        }

        // This is the crucial part: update alpha and prune if possible
        alpha = alpha.max(best_score);
        if alpha >= beta {
            break; // Prune remaining moves
        }
    }

    (best_score, best_placement)
}

fn alphabeta(board: &Board, depth: usize, mut alpha: i32, mut beta: i32, maximizing_player: Player) -> i32 {
    if depth == 0 || board.is_game_over() {
        return evaluate(board);
    }

    if maximizing_player == Player::Red {
        let mut value = -i32::MAX;

        for current_move in board.get_valid_moves() {
            value = value.max(
                alphabeta(&board.make_move(current_move), depth - 1, alpha, beta, maximizing_player.opponent())
            );
            if value >= beta {
                break;
            }
            alpha = alpha.max(value);
        }
        value
    }
    else
    {
        let mut value = i32::MAX;
        for current_move in board.get_valid_moves() {
            value = value.min(
                alphabeta(&board.make_move(current_move), depth - 1, alpha, beta, maximizing_player.opponent())
            );
            if value <= alpha {
                break;
            }
            beta = beta.min(value);
        }
        value
    }
}
