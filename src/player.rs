use crate::entity::Entity;
use crate::sprite::Sprite;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new() -> Self {
        Player {
            entity: Entity {
                x: 10,
                y: 10,
                width: 1,
                height: 1,
                sprite: Sprite {
                    glyph: '@',
                    color: macroquad::color::WHITE,
                    tile_id: 0,
                },
            },
        }
    }
}
