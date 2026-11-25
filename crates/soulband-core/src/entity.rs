use crate::pos::Pos;
use crate::sprite::Sprite;

pub struct Entity {
    /// Top-left tile of the entity on the grid.
    pub pos: Pos,

    /// Size in tiles. The player is always 1x1; large monsters can be bigger.
    pub width: i32,
    pub height: i32,
    pub sprite: Sprite,
}

impl Entity {
    pub fn new(pos: Pos, width: i32, height: i32, sprite: Sprite) -> Self {
        Entity {
            pos,
            width,
            height,
            sprite,
        }
    }
}
