use crate::sprite::Sprite;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub sprite: Sprite,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 0,
            y: 0,
            sprite: Sprite {
                glyph: '@',
                color: macroquad::color::WHITE,
                tile_id: 0,
            },
        }
    }
}
