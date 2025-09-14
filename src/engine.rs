use std::collections::HashMap;

use crate::{board::Board, player::Player};

// A simple transposition table (memoization) to store results of evaluated board states.
pub type TranspositionTable = HashMap<Board, (i32, usize)>;

// Evaluates the board state from the perspective of a given player.
// Positive score is good for the player, negative is bad.
pub fn evaluate(player: Player, board: &Board) -> i32 {
    let mut score = 0;
    let mut p1_squares = 0;
    let mut p2_squares = 0;

    for r in 0..5 {
        for c in 0..5 {
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

    if p2_squares == 0 && p1_squares > 0 {
        score = i32::MAX;
    } // P1 wins
    if p1_squares == 0 && p2_squares > 0 {
        score = i32::MIN;
    } // P2 wins

    if player == Player::Red { score } else { -score }
}

// Minimax algorithm with corrected alpha-beta pruning and transposition table.
pub fn minimax(
    board: &Board,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
    player_to_optimize: Player,
    tt: &mut TranspositionTable,
) -> (i32, Option<(usize, usize)>) {
    // Base case: Stop recursion at max depth or game over.
    if depth == 0 || board.is_game_over() {
        return (evaluate(player_to_optimize, board), None);
    }

    // --- FIX 1 & 2: Corrected Transposition Table Lookup ---
    // Check if the current board state is already in the table and if
    // the stored result is from a search of at least the same depth.
    if let Some(&(stored_eval, stored_depth)) = tt.get(board)
        && stored_depth >= depth {
            return (stored_eval, None); // Return the cached evaluation.
        }

    let mut best_move = None;
    let mut best_value = if maximizing_player {
        i32::MIN
    } else {
        i32::MAX
    };

    // Iterate through all valid moves from the current position.
    for m in board.get_valid_moves() {
        let new_board = board.make_move(m);
        // Recursive call for the new board state.
        let (eval, _) = minimax(
            &new_board,
            depth - 1,
            alpha,
            beta,
            !maximizing_player, // Switch player perspective.
            player_to_optimize,
            tt,
        );

        if maximizing_player {
            if eval > best_value {
                best_value = eval;
                best_move = Some(m);
            }
            alpha = alpha.max(eval);
        } else { // Minimizing player
            if eval < best_value {
                best_value = eval;
                best_move = Some(m);
            }
            beta = beta.min(eval);
        }

        // Alpha-beta pruning.
        if beta <= alpha {
            break;
        }
    }

    // Store the result of this search in the transposition table.
    tt.insert(board.clone(), (best_value, depth));

    (best_value, best_move)
}
