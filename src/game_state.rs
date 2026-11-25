use crate::level;
use crate::player;
use macroquad::input::KeyCode;
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

    /// Handles keyboard input and updates game state accordingly
    pub fn handle_input(&mut self) {
        let key = macroquad::input::get_last_key_pressed();
        if key.is_none() {
            return; // No key pressed, skip processing
        }
        let key = key.unwrap();

        let mut move_player = |dx: i32, dy: i32| {
            self.player.entity.pos.x += dx;
            self.player.entity.pos.y += dy;
        };

        use KeyCode::*;
        match key {
            // Movement
            // q w e       7 8 9
            // a . d       4 5 6
            // z s c       1 2 3
            W | Kp8 | Up => move_player(0, -1),   // Up
            S | Kp2 | Down => move_player(0, 1),  // Down
            A | Kp4 | Left => move_player(-1, 0), // Left
            D | Kp6 | Right => move_player(1, 0), // Right
            Q | Kp7 => move_player(-1, -1),       // Up Left
            E | Kp9 => move_player(1, -1),        // Up Right
            Z | Kp1 => move_player(-1, 1),        // Down Left
            C | Kp3 => move_player(1, 1),         // Down Right

            // Wait/pass turn
            Space | Period | Kp5 => {
                // TODO: Implement turn passing/resting when turn system is added
            }

            // Unhandled keys
            _ => {
                // TODO: Log unhandled key to message log system once implemented
                println!("Unhandled key: {:?}", key);
            }
        }
    }
}
