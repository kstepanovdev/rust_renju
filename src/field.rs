pub struct Renju {
    pub field: [Mark; 225],
    pub turn: Player,
}

#[derive(Clone, Copy)]
pub enum Mark {
    None,
    Red,
    Blue
}

pub enum Player {
    PlayerOne,
    PlayerTwo
}

impl Renju {
    pub fn new() -> Self {
        Renju {
            field: [Mark::None; 225],
            turn: Player::PlayerOne
        }
    }
}

