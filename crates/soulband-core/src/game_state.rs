use crate::level;
use crate::player;
use rand::rngs::StdRng;

pub struct GameState {
    pub player: player::Player,
    pub level: level::Level,
    pub rng: StdRng,
}

impl GameState {
    pub fn new(player: player::Player, level: level::Level, rng: StdRng) -> Self {
        GameState { player, level, rng }
    }
}
