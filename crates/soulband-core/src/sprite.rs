/// An RGB colour. Frontend-independent; the renderer converts it to its own colour type.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const WHITE: Rgb = Rgb::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

/// How an entity looks: its ASCII glyph and colour, and its tile in the tileset.
pub struct Sprite {
    pub glyph: char,
    pub color: Rgb,
    pub tile_id: u32,
}
