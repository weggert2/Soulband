use crate::sprite::Sprite;

pub struct Entity {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub sprite: Sprite,
}

impl Entity {
    pub fn new(x: u32, y: u32, width: u32, height: u32, sprite: Sprite) -> Self {
        Entity {
            x,
            y,
            width,
            height,
            sprite,
        }
    }
}
