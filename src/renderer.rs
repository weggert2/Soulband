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

        let font_size: f32 = (self.tile_size * entity.height) as f32 * 0.8;
        let glyph_str: String = entity.sprite.glyph.to_string();

        let text_dimensions: text::TextDimensions =
            text::measure_text(&glyph_str, Some(&self.font), font_size as u16, 1.0);

        // Center horizontally and vertically
        let x_offset: f32 = 0.5 * (self.tile_size as f32 - text_dimensions.width);
        let y_offset: f32 =
            0.5 * (self.tile_size as f32 - text_dimensions.height) + text_dimensions.offset_y;

        text::draw_text_ex(
            &glyph_str,
            screen_x as f32 + x_offset,
            screen_y as f32 + y_offset,
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
