use crate::entity::Entity;
use crate::sprite::Sprite;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new() -> Self {
        Player {
            entity: Entity {
                x: 10.0,
                y: 10.0,
                width: 1.0,
                height: 1.0,
                sprite: Sprite {
                    glyph: '@',
                    color: macroquad::color::WHITE,
                    tile_id: 391,
                },
            },
        }
    }
}
