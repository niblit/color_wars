//! The game's AI engine, powered by a minimax search algorithm with alpha-beta pruning.
use std::{collections::{HashMap, HashSet}, time::Duration};

use crate::{board::Board, player::Player, prelude::Coordinates, BOARD_ROW_SIZE, BOARD_COLUMN_SIZE};

/// Evaluates the board state from a static, Red-player perspective.
///
/// A positive score is favorable for Red, while a negative score is favorable for Blue.
/// The magnitude of the score represents the degree of advantage. A win for either
/// player is represented by `i32::MAX` or `i32::MIN`.
///
/// # Returns
/// An `i32` score representing the board state.
pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    let mut red_squares = 0usize;
    let mut blue_squares = 0usize;

    let mut visited_squares = HashSet::new();

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
                let our_value = board.grid()[r][c].value();
                for neighbor in Coordinates::new(r, c).neighbors() {
                    if visited_squares.contains(&neighbor) ||board.grid()[neighbor.row()][neighbor.column()].owner().is_none(){
                        continue;
                    }

                    let perimeter_value = if our_value > board.grid()[neighbor.row()][neighbor.column()].value() {
                        10
                    } else if our_value == board.grid()[neighbor.row()][neighbor.column()].value() {
                        if owner == board.turn() {
                            10
                        }
                        else {
                            -10
                        }
                    } else {
                        0
                    };

                    if board.turn() == Player::Red {
                        score += perimeter_value;
                    } else {
                        score -= perimeter_value;
                    }
                    visited_squares.insert(neighbor);
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
        score = i32::MIN;
    }

    score
}

/// K = board, V = (score, depth)
type TranspositionTable = HashMap<Board, (i32, usize)>;

/// The main entry point for the AI search.
///
/// It finds the best move for the player whose turn it is on the given `board`.
///
/// # Arguments
/// * `board` - The current `Board` state to analyze.
/// * `time` - The max time the engine can use to search
///
/// # Returns
/// A tuple containing the evaluation score of the best move and the `Coordinates` of that move.
pub fn search(board: &Board, time: Duration) -> (i32, Coordinates) {
    let mut current_depth = 1;
    let maximizing_player = board.turn() == Player::Red;
    let mut transposition_table = TranspositionTable::new();
    let placement_evaluations = board.get_valid_moves();

    let mut best_placement = placement_evaluations[0];
    let mut best_score = if maximizing_player { i32::MIN } else { i32::MAX };

    let start = std::time::Instant::now();

    let mut last_iteration_duration = Duration::from_secs(0);
    loop {
        if (start.elapsed() + last_iteration_duration) >= time {
            break;
        }

        let iteration_start = std::time::Instant::now();
        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        let mut current_best_score_for_depth = if maximizing_player { i32::MIN } else { i32::MAX };

        for current_placement in &placement_evaluations {
            let current_placement = *current_placement;
            let board_after_move = board.make_move(current_placement);
            let score = alpha_beta_prunning(&board_after_move, current_depth - 1, alpha, beta, !maximizing_player, &mut transposition_table);

            if maximizing_player {
                if score > current_best_score_for_depth {
                    current_best_score_for_depth = score;
                    // Only update the final best_placement if we complete a full search at this depth
                    best_placement = current_placement;
                }
                alpha = alpha.max(current_best_score_for_depth); // Update alpha for the *next sibling's* search window
            } else { // Minimizing player
                if score < current_best_score_for_depth {
                    current_best_score_for_depth = score;
                    best_placement = current_placement;
                }
                beta = beta.min(current_best_score_for_depth); // Update beta for the *next sibling's* search window
            }
        }

        // The best score from the completed search becomes the official best_score
        best_score = current_best_score_for_depth;

        current_depth += 1;
        last_iteration_duration = iteration_start.elapsed();
    }
    println!("Searched to depth: {}", current_depth - 1); // -1 because we increment after the last successful search

    (best_score, best_placement)
}

/// The recursive core of the minimax algorithm with alpha-beta pruning.
///
/// This function explores the game tree to find the best possible score from a given
/// board state, pruning branches that are probably suboptimal.
fn alpha_beta_prunning(board: &Board, depth: usize, mut alpha: i32, mut beta: i32, maximizing_player: bool, transposition_table: &mut TranspositionTable) -> i32 {
    if depth == 0 || board.is_game_over() {
        return evaluate(board);
    }

    if let Some((cached_score, cached_depth)) = transposition_table.get(board)
        && *cached_depth >= depth {
            return *cached_score;
    };

    if maximizing_player {
        let mut value = -i32::MAX;

        for current_move in board.get_valid_moves() {
            let board_after_move = board.make_move(current_move);
            value = value.max(
                alpha_beta_prunning(
                    &board_after_move,
                    depth - 1,
                    alpha,
                    beta,
                    false,
                    transposition_table
                )
            );
            if value >= beta {
                break; // beta cutoff
            }
            alpha = alpha.max(value);
        }
        transposition_table.insert(board.clone(), (value, depth));
        value
    }
    else
    {
        let mut value = i32::MAX;

        for current_move in board.get_valid_moves() {
            let board_after_move = board.make_move(current_move);
            value = value.min(
                alpha_beta_prunning(
                    &board_after_move,
                    depth - 1,
                    alpha,
                    beta,
                    true,
                    transposition_table
                )
            );
            if value <= alpha {
                break; // alpha cutoff
            }
            beta = beta.min(value);
        }
        transposition_table.insert(board.clone(), (value, depth));
        value
    }
}
