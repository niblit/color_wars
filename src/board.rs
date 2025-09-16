//! Manages the game state and enforces the rules of Color Wars.
use crate::{BOARD_ROW_SIZE, BOARD_COLUMN_SIZE, coordinates::Coordinates, player::Player, square::Square};

/// A type alias for the 2D array representing the game grid.
pub type Grid = [[Square; BOARD_COLUMN_SIZE]; BOARD_ROW_SIZE];

/// Represents the entire game board and its current state.
///
/// This struct holds the grid of squares and tracks whose turn it is.
/// It is the central point for all game logic, such as validating and applying moves.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    grid: Grid,
    turn: Player,
}

impl Board {
    /// Creates a new, empty board for a given player's turn.
    pub fn new(current_turn: Player) -> Self {
        Board {
            grid: [
                [
                    Square::empty(); BOARD_COLUMN_SIZE
                ]; BOARD_ROW_SIZE
            ],
            turn: current_turn,
        }
    }

    /// Returns a copy of the current grid state.
    pub fn grid(&self) -> Grid {
        self.grid
    }

    /// Returns the player whose turn it is.
    pub fn turn(&self) -> Player {
        self.turn
    }

    /// An alternate constructor to set up the initial game state with two starting pieces.
    ///
    /// Player Red always gets the first turn after setup.
    pub fn setup(red_placement: Coordinates, blue_placement: Coordinates) -> Self {
        let mut board = Board::new(Player::Red);

        board.grid[red_placement.row()][red_placement.column()] =
            Square::occupied(Player::Red, 3);

        board.grid[blue_placement.row()][blue_placement.column()] =
            Square::occupied(Player::Blue, 3);

        board
    }

    /// Returns a list of all squares the current player can choose for a move.
    /// A move is represented by the `Coordinates` of a square that the player occupies.
    pub fn get_valid_moves(&self) -> Vec<Coordinates> {
        let mut moves = Vec::new();
        for row in 0..BOARD_ROW_SIZE {
            for column in 0..BOARD_COLUMN_SIZE {
                if let Some(player) = self.grid[row][column].owner()
                    && player == self.turn
                {
                    moves.push(Coordinates::new(row, column));
                }
            }
        }
        moves
    }

    /// Applies a move to the board, handling pops and chain reactions.
    ///
    /// This method takes the coordinates of a player's square to increment. It processes
    /// the initial value increase and any subsequent chain reaction of "pops."
    /// It returns a new `Board` representing the state after the move is completed
    /// and the turn has been passed to the opponent.
    ///
    /// # Panics
    /// Panics if the provided `placement` is not a valid move for the current player.
    pub fn make_move(&self, placement: Coordinates) -> Board {
        assert!(self.get_valid_moves().contains(&placement));

        let mut new_board = self.clone();
        new_board.grid[placement.row()][placement.column()].increment_value();

        let mut pops = vec![];
        if new_board.grid[placement.row()][placement.column()].value() >= 4 {
            pops.push(placement);
        }

        // Process all pops in the chain reaction
        while let Some(pop_location) = pops.pop() {
            // The square that pops becomes empty
            new_board.grid[pop_location.row()][pop_location.column()].reset_square();

            for neighbor_position in pop_location.neighbors() {
                let square =
                    &mut new_board.grid[neighbor_position.row()][neighbor_position.column()];

                square.set_owner(self.turn);
                square.increment_value();

                // If the neighbor pops, add it to the list to be processed
                if square.value() >= 4 {
                    pops.push(neighbor_position);
                }
            }
        }

        new_board.turn = self.turn.opponent();
        new_board
    }

    /// Checks if the game has ended.
    ///
    /// The game is over if either player has no squares left on the board.
    pub fn is_game_over(&self) -> bool {
        let mut red_squares = 0;
        let mut blue_squares = 0;
        for row in 0..BOARD_ROW_SIZE {
            for column in 0..BOARD_COLUMN_SIZE {
                if let Some(owner) = self.grid[row][column].owner() {
                    if owner == Player::Red {
                        red_squares += 1;
                    } else {
                        blue_squares += 1;
                    }
                }
            }
        }

        (red_squares > 0 || blue_squares > 0) && (red_squares == 0 || blue_squares == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a board for testing
    fn setup_test_board() -> Board {
        Board::setup(Coordinates::new(0, 0), Coordinates::new(4, 4))
    }

    #[test]
    fn test_setup() {
        let board = setup_test_board();
        assert_eq!(board.turn(), Player::Red);
        assert_eq!(board.grid[0][0].owner(), Some(Player::Red));
        assert_eq!(board.grid[0][0].value(), 3);
        assert_eq!(board.grid[4][4].owner(), Some(Player::Blue));
        assert_eq!(board.grid[4][4].value(), 3);
    }

    #[test]
    fn test_get_valid_moves() {
        let board = setup_test_board();
        assert_eq!(board.get_valid_moves(), vec![Coordinates::new(0, 0)]);
        let board_p2 = Board { turn: Player::Blue, ..board };
        assert_eq!(board_p2.get_valid_moves(), vec![Coordinates::new(4, 4)]);
    }

    #[test]
    fn test_simple_pop() {
        let board = setup_test_board(); // Red at (0,0) with value 3
        let new_board = board.make_move(Coordinates::new(0, 0));

        // 1. The popped square (0,0) should be empty
        assert_eq!(new_board.grid[0][0], Square::empty());

        // 2. Neighbors (1,0) and (0,1) should be Red with value 1
        assert_eq!(new_board.grid[1][0].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[1][0].value(), 1);
        assert_eq!(new_board.grid[0][1].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[0][1].value(), 1);

        // 3. Turn should switch to Blue
        assert_eq!(new_board.turn(), Player::Blue);
    }

    #[test]
    fn test_pop_captures_opponent() {
        let mut board = setup_test_board();
        // Manually place a Blue square next to the Red one
        board.grid[0][1] = Square::occupied(Player::Blue, 2);

        let new_board = board.make_move(Coordinates::new(0, 0));

        // The neighbor at (0,1) should now be Red with value 3 (2+1)
        assert_eq!(new_board.grid[0][1].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[0][1].value(), 3);
    }

    #[test]
    fn test_chain_reaction() {
        let mut board = Board::new(Player::Red);
        board.grid[0][0] = Square::occupied(Player::Red, 3);
        board.grid[0][1] = Square::occupied(Player::Red, 3); // This will pop from the first pop

        let new_board = board.make_move(Coordinates::new(0, 0));

        // (0,0) pops, increments (0,1) to 4, which also pops.
        // Final state: (0,0) is red and value is 1, (0,1) is empty, and (0,2) and (1, 1) are red and values are 1
        assert_eq!(new_board.grid[0][1], Square::empty());
        // Check squares affected by the second pop
        assert_eq!(new_board.grid[0][0].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[0][0].value(), 1);
        assert_eq!(new_board.grid[0][2].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[0][2].value(), 1);
        assert_eq!(new_board.grid[1][1].owner(), Some(Player::Red));
        assert_eq!(new_board.grid[1][1].value(), 1);
    }

    #[test]
    fn test_is_game_over() {
        let mut board = Board::new(Player::Red);
        assert!(!board.is_game_over()); // Empty board is not game over
        board.grid[0][0] = Square::occupied(Player::Red, 1);
        assert!(board.is_game_over()); // Only one player has squares
    }
}
