use crate::player::Player;

// Represents a single square on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square {
    owner: Option<Player>,
    value: u8,
}

impl Default for Square {
    fn default() -> Self {
        Self::new(None, 0)
    }
}

impl Square {
    // Creates a new, empty square.
    pub fn new(owner: Option<Player>, value: u8) -> Self {
        Square { owner, value }
    }

    pub fn owner(&self) -> Option<Player> {
        self.owner
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn increment_value(&mut self) {
        self.value += 1
    }

    pub fn reset_value(&mut self) {
        self.value = 0
    }

    pub fn set_owner(&mut self, owner: Player) {
        self.owner = Some(owner)
    }

    pub fn reset_square(&mut self) {
        self.owner = None;
        self.reset_value();
    }
}
