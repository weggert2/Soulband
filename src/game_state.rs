use crate::player;

pub struct GameState {
    pub player: player::Player,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player: player::Player::new(),
        }
    }
}
