// Represents one of the two players.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    Red,
    Blue,
}

impl Player {
    // Returns the other player.
    pub fn opponent(&self) -> Player {
        match self {
            Player::Red => Player::Blue,
            Player::Blue => Player::Red,
        }
    }
}
