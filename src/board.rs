use crate::{BOARD_ROW_SIZE, BOARD_COLUMN_SIZE, coordinates::Coordinates, player::Player, square::Square};

// Represents the game board.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    grid: [[Square; BOARD_COLUMN_SIZE]; BOARD_ROW_SIZE],
    turn: Player,
}

impl Board {
    // Creates a new, empty board for a given player's turn.
    pub fn new(current_turn: Player) -> Self {
        Board {
            grid: [[Square::default(); BOARD_COLUMN_SIZE]; BOARD_ROW_SIZE],
            turn: current_turn,
        }
    }

    pub fn grid(&self) -> [[Square; BOARD_COLUMN_SIZE]; BOARD_ROW_SIZE] {
        self.grid
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    // An alternate constructor to set up the initial game state.
    pub fn setup(red_placement: Coordinates, blue_placement: Coordinates) -> Self {
        let mut board = Board::new(Player::Red);
        board.grid[red_placement.row()][red_placement.column()] = Square::new(Player::Red, 3);
        board.grid[blue_placement.row()][blue_placement.column()] =
            Square::new(Player::Blue, 3);
        board
    }

    // Returns a list of valid moves for the current player.
    // A move is represented by the (row, col) of the square to increment.
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

    // Applies a move to the board, handling pops and chain reactions.
    // Returns a new Board state after the move.
    pub fn make_move(&self, placement: Coordinates) -> Board {
        assert!(self.get_valid_moves().contains(&placement));

        let mut new_board = self.clone();
        new_board.grid[placement.row()][placement.column()].increment_value();

        let mut pops = vec![];
        if new_board.grid[placement.row()][placement.column()].value() >= 4 {
            pops.push(placement);
        }

        while let Some(pop_location) = pops.pop() {
            // The square that pops becomes empty
            new_board.grid[pop_location.row()][pop_location.column()].reset_square();

            for neighbor_position in pop_location.neighbors() {
                let square =
                    &mut new_board.grid[neighbor_position.row()][neighbor_position.column()];
                square.set_owner(self.turn);
                square.increment_value();
                if square.value() >= 4 {
                    pops.push(neighbor_position);
                }
            }
        }

        new_board.turn = self.turn.opponent();
        new_board
    }

    // Checks if the game is over.
    pub fn is_game_over(&self) -> bool {
        let mut red_squares = 0;
        let mut blue_squares = 0;
        for r in 0..BOARD_ROW_SIZE {
            for c in 0..BOARD_COLUMN_SIZE {
                if let Some(owner) = self.grid[r][c].owner() {
                    if owner == Player::Red {
                        red_squares += 1;
                    } else {
                        blue_squares += 1;
                    }
                }
            }
        }
        red_squares == 0 || blue_squares == 0
    }
}
