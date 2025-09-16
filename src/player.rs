//! Defines the `Player` type, representing one of the two participants in the game.
use std::fmt;

/// Represents one of the two players in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    /// The Red player.
    Red,
    /// The Blue player.
    Blue,
}

impl Player {
    /// Returns the opposing player.
    ///
    /// This is useful for switching turns or checking for wins where the
    /// context needs to be inverted.
    #[must_use]
    pub fn opponent(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}

/// A way to print to terminal the player information
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::Red => write!(f, "Red"),
            Player::Blue => write!(f, "Blue"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Player::Red, Player::Red);
        assert_eq!(Player::Blue, Player::Blue);
    }

    #[test]
    fn inequality() {
        assert_ne!(Player::Red, Player::Blue);
    }

    #[test]
    fn opponent() {
        assert_eq!(
            Player::Red.opponent(),
            Player::Blue
        );
        assert_eq!(
            Player::Blue.opponent(),
            Player::Red
        );
    }

    #[test]
    fn double_opponent() {
        assert_eq!(
            Player::Red.opponent().opponent(),
            Player::Red
        );
        assert_eq!(
            Player::Blue.opponent().opponent(),
            Player::Blue
        );
    }

    #[test]
    fn display_trait() {
        assert_eq!(format!("{}", Player::Red), "Red");
        assert_eq!(Player::Blue.to_string(), "Blue");
    }
}
