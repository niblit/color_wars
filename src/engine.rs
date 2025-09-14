use std::collections::HashMap;

use crate::{BOARD_SIZE, board::Board, player::Player, prelude::Coordinates};

pub fn search(board: &Board, player: Player) -> (i32, Option<Coordinates>) {
    let mut tt = TranspositionTable::new();
    alphabeta(
        board,
        8,
        -i32::MAX,
        i32::MAX,
        player == Player::Red,
        &mut tt,
        None,
    )
}

// A simple transposition table (memoization) to store results of evaluated board states.
pub type TranspositionTable = HashMap<Board, (i32, usize)>;

// Evaluates the board state from the perspective of a given player.
// Positive score is good for the player, negative is bad.
pub fn evaluate(player: Player, board: &Board) -> i32 {
    let mut score = 0;
    let mut p1_squares = 0;
    let mut p2_squares = 0;

    for r in 0..BOARD_SIZE {
        for c in 0..BOARD_SIZE {
            if let Some(owner) = board.grid()[r][c].owner() {
                match owner {
                    Player::Red => {
                        score += board.grid()[r][c].value() as i32;
                        p1_squares += 1;
                    }
                    Player::Blue => {
                        score -= board.grid()[r][c].value() as i32;
                        p2_squares += 1;
                    }
                }
            }
        }
    }

    // P1 wins
    if p2_squares == 0 && p1_squares > 0 {
        score = i32::MAX;
    }

    // P2 wins
    if p1_squares == 0 && p2_squares > 0 {
        score = -i32::MAX;
    }

    if player == Player::Red { score } else { -score }
}

// Minimax algorithm with corrected alpha-beta pruning and transposition table.
pub fn alphabeta(
    board: &Board,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    tt: &mut TranspositionTable,
    last_played_move: Option<Coordinates>,
) -> (i32, Option<Coordinates>) {
    let player_to_optimize = match maximizing_player {
        true => Player::Red,
        false => Player::Blue,
    };

    // Base case: Stop recursion at max depth or game over.
    if depth == 0 || board.is_game_over() {
        return (
            evaluate(player_to_optimize, board),
            last_played_move,
        );
    }

    // Check if the current board state is already in the table and if
    // the stored result is from a search of at least the same depth.
    if let Some(&(stored_eval, stored_depth)) = tt.get(board)
        && stored_depth >= depth
    {
        return (stored_eval, last_played_move); // Return the cached evaluation.
    }

    let mut best_move = last_played_move;
    let mut best_score = if maximizing_player {
        -i32::MAX
    } else {
        i32::MAX
    };

    // Iterate through all valid moves from the current position.
    for current_move in board.get_valid_moves() {
        let new_board = board.make_move(current_move);
        // Recursive call for the new board state.
        let (eval, _) = alphabeta(
            &new_board,
            depth - 1,
            alpha,
            beta,
            !maximizing_player, // Switch player perspective.
            tt,
            Some(current_move),
        );

        if maximizing_player {
            if eval > best_score {
                best_score = eval;
                best_move = Some(current_move);
            }
            alpha = alpha.max(eval);
        } else {
            // Minimizing player
            if eval < best_score {
                best_score = eval;
                best_move = Some(current_move);
            }
            beta = beta.min(eval);
        }

        // Alpha-beta pruning.
        if beta <= alpha {
            break;
        }
    }

    // Store the result of this search in the transposition table.
    tt.insert(board.clone(), (best_score, depth));

    let valid = board.get_valid_moves();

    if let Some(best_placement) = best_move
        && !valid.contains(&best_placement) {
            best_move = Some(board.get_valid_moves()[0]);
        }

    if best_move.is_none() {
        best_move = Some(board.get_valid_moves()[0]);
    }
    (best_score, best_move)
}
