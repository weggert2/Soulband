use macroquad::shapes;
use macroquad::text::{self, Font};

use crate::entity::Entity;

pub struct Renderer {
    pub tile_size: u32,
    pub font: Font,
}

impl Renderer {
    pub fn new(tile_size: u32, font: Font) -> Self {
        Renderer { tile_size, font }
    }

    pub fn draw_entity(&self, entity: &Entity) {
        let screen_x: u32 = entity.x * self.tile_size;
        let screen_y: u32 = entity.y * self.tile_size;

        // ascii only for now
        let font_size: u32 = self.tile_size * entity.height;
        let glyph_str: String = entity.sprite.glyph.to_string();
        text::draw_text_ex(
            &glyph_str,
            screen_x as f32,
            (screen_y + font_size) as f32,
            text::TextParams {
                font: Some(&self.font),
                font_size: font_size as u16,
                color: entity.sprite.color,
                ..Default::default()
            },
        );
    }

    pub fn draw_grid(&self) {
        let screen_w: f32 = macroquad::window::screen_width();
        let screen_h: f32 = macroquad::window::screen_height();
        let tile_size_f: f32 = self.tile_size as f32;

        // draw vertical lines
        let mut x: f32 = 0.0;
        while x < screen_w {
            shapes::draw_line(x, 0.0, x, screen_h, 1.0, macroquad::color::GRAY);
            x += tile_size_f;
        }

        // draw horizontal lines
        let mut y: f32 = 0.0;
        while y < screen_h {
            shapes::draw_line(0.0, y, screen_w, y, 1.0, macroquad::color::GRAY);
            y += tile_size_f;
        }
    }
}
