use crate::sprite::Sprite;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub sprite: Sprite,
}

impl Entity {
    pub fn new(x: f32, y: f32, width: f32, height: f32, sprite: Sprite) -> Self {
        Entity {
            x: x,
            y: y,
            width: width,
            height: height,
            sprite: sprite,
        }
    }
}
