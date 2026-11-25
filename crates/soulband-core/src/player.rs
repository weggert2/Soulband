use crate::entity::Entity;
use crate::pos::Pos;
use crate::sprite::{Rgb, Sprite};

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new() -> Self {
        Player {
            entity: Entity {
                pos: Pos::new(10, 10),
                width: 1,
                height: 1,
                sprite: Sprite {
                    glyph: '@',
                    color: Rgb::WHITE,
                    tile_id: 391,
                },
            },
        }
    }
}
