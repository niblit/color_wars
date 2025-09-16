//! Defines the `Square` type, which represents a single tile on the game board.
use crate::player::Player;

/// Represents a single square on the board, which can be empty or owned by a player.
///
/// A square's state is defined by its owner and its numeric value.
/// - An **empty** square has an owner of `None` and a value of `0`.
/// - An **occupied** square has an owner of `Some(Player)` and a value from `1` to `3`.
// TODO! add Popped state to handle value >= 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Square {
    Empty,
    Occupied {
        owner: Player,
        value: u8
    }
}

impl Default for Square {
    /// Creates a new, empty square by default.
    fn default() -> Self {
        Self::Empty
    }
}

impl Square {
    /// Creates a new square with a specified owner and value.
    ///
    /// This is the primary constructor for creating any square state.
    pub fn new(owner: Player, value: u8) -> Self {
        Self::Occupied {
            owner,
            value
        }
    }

    /// Returns the player who owns the square, if any.
    pub fn owner(&self) -> Option<Player> {
        match self {
            Square::Empty => None,
            Square::Occupied { owner, value: _ } => Some(*owner),
        }
    }

    /// Returns the numeric value of the square.
    /// An empty square is valued as 0
    pub fn value(&self) -> u8 {
        match self {
            Square::Empty => 0,
            Square::Occupied { owner: _, value } => *value,
        }
    }

    /// Increases the square's value by 1.
    pub fn increment_value(&mut self) {
        match self {
            Self::Occupied { owner: _, value } => *value += 1,
            Square::Empty => {},
        }
    }

    /// Assigns a new owner to the square.
    pub fn set_owner(&mut self, new_owner: Player) {
        match self {
            Self::Occupied { owner, value: _ } => *owner = new_owner,
            Square::Empty => {
                *self = Self::new(new_owner, 0);
            },
        }
    }

    /// Resets the square to an empty state.
    pub fn reset_square(&mut self) {
        *self = Self::default();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn default_square_is_empty() {
        let s = Square::default();
        assert_eq!(s.owner(), None);
        assert_eq!(s.value(), 0);
    }

    #[test]
    fn new_creates_occupied_square() {
        let s = Square::new(Player::Red, 3);
        assert_eq!(s.owner(), Some(Player::Red));
        assert_eq!(s.value(), 3);
    }

    #[test]
    fn increment_value_works() {
        let mut s = Square::new(Player::Blue, 1);
        s.increment_value();
        assert_eq!(s.value(), 2);
    }

    #[test]
    fn set_owner_works() {
        let mut s = Square::default();
        assert_eq!(s.owner(), None);

        s.set_owner(Player::Red);
        assert_eq!(s.owner(), Some(Player::Red));

        let mut s = Square::new(Player::Blue, 1);
        s.set_owner(Player::Red);

        assert_eq!(
            s,
            Square::new(Player::Red, 1)
        );
    }

    #[test]
    fn reset_square_works() {
        let mut s = Square::new(Player::Blue, 3);
        s.reset_square();
        // After reset, it should be identical to a default square
        assert_eq!(s, Square::default());
    }
}
