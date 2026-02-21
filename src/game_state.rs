use crate::player;
use macroquad::input::KeyCode;

pub struct GameState {
    pub player: player::Player,
}

impl GameState {
    pub fn new(player: player::Player) -> Self {
        GameState { player }
    }

    /// Handles keyboard input and updates game state accordingly
    pub fn handle_input(&mut self) {
        let key = macroquad::input::get_last_key_pressed();
        if key.is_none() {
            return; // No key pressed, skip processing
        }
        let key = key.unwrap();

        let mut move_player = |dx: f32, dy: f32| {
            self.player.entity.x += dx;
            self.player.entity.y += dy;
        };

        use KeyCode::*;
        match key {
            // Movement
            // q w e       7 8 9
            // a . d       4 5 6
            // z s c       1 2 3
            W | Kp8 | Up => move_player(0.0, -1.0),   // Up
            S | Kp2 | Down => move_player(0.0, 1.0),  // Down
            A | Kp4 | Left => move_player(-1.0, 0.0), // Left
            D | Kp6 | Right => move_player(1.0, 0.0), // Right
            Q | Kp7 => move_player(-1.0, -1.0),       // Up Left
            E | Kp9 => move_player(1.0, -1.0),        // Up Right
            Z | Kp1 => move_player(-1.0, 1.0),        // Down Left
            C | Kp3 => move_player(1.0, 1.0),         // Down Right

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
