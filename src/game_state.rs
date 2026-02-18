use crate::player;

pub struct GameState {
    pub player: player::Player,
}

impl GameState {
    pub fn new(player: player::Player) -> Self {
        GameState { player }
    }
}
