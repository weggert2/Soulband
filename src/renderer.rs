use macroquad::shapes;
use macroquad::text::Font;
use macroquad::texture::Texture2D;

use crate::entity::Entity;

/// Handles rendering of entities, tiles, and debug overlays
pub struct Renderer {
    /// Size of each tile in the game grid (in pixels)
    pub game_tile_size: f32,
    /// Size of each tile in the tileset texture (in pixels)
    pub tileset_tile_size: f32,
    /// Font used for ASCII glyph rendering
    pub font: Font,
    /// Texture containing the tile graphics
    pub tileset: Texture2D,
}

impl Renderer {
    /// Creates a new Renderer
    ///
    /// # Arguments
    ///
    /// * `game_tile_size` - Size of each tile in pixels (for our grid)
    /// * `tileset_tile_size` - Size of each tile in the tileset texture
    /// * `font` - Font to use for text rendering
    /// * `tileset` - Texture containing the tile graphics
    pub fn new(
        game_tile_size: f32,
        tileset_tile_size: f32,
        font: Font,
        tileset: Texture2D,
    ) -> Self {
        Renderer {
            game_tile_size,
            tileset_tile_size,
            font,
            tileset,
        }
    }

    /// Draws an entity at its grid position
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to draw
    /// * `use_graphics` - If true, use tileset; if false, use ASCII glyphs
    pub fn draw_entity(&self, entity: &Entity, use_graphics: bool) {
        let screen_x: f32 = entity.x * self.game_tile_size;
        let screen_y: f32 = entity.y * self.game_tile_size;

        if use_graphics {
            self.draw_tile(entity, screen_x, screen_y);
        } else {
            self.draw_glyph(entity, screen_x, screen_y);
        }
    }

    /// Draws an entity's ASCII glyph centered within a tile
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to draw (for sprite data)
    /// * `screen_x` - X position on screen in pixels
    /// * `screen_y` - Y position on screen in pixels
    pub fn draw_glyph(&self, entity: &Entity, screen_x: f32, screen_y: f32) {
        let font_size: f32 = self.game_tile_size * entity.height;
        let glyph_str: String = entity.sprite.glyph.to_string();

        let text_dimensions: macroquad::text::TextDimensions =
            macroquad::text::measure_text(&glyph_str, Some(&self.font), font_size as u16, 1.0);

        // Center horizontally and vertically
        let x_offset: f32 = 0.5 * (self.game_tile_size - text_dimensions.width);
        let y_offset: f32 =
            0.5 * (self.game_tile_size - text_dimensions.height) + text_dimensions.offset_y;

        macroquad::text::draw_text_ex(
            &glyph_str,
            screen_x + x_offset,
            screen_y + y_offset,
            macroquad::text::TextParams {
                font: Some(&self.font),
                font_size: font_size as u16,
                color: entity.sprite.color,
                ..Default::default()
            },
        );
    }

    /// Draws an entity's tile from the tileset to the screen
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to draw (for sprite tile_id)
    /// * `screen_x` - X position on screen in pixels
    /// * `screen_y` - Y position on screen in pixels
    pub fn draw_tile(&self, entity: &Entity, screen_x: f32, screen_y: f32) {
        // Tileset is 8192 / 64 = 128 tiles per row
        let tiles_per_row: u32 = 128;
        let tile_x: f32 = (entity.sprite.tile_id % tiles_per_row) as f32; // column in tileset
        let tile_y: f32 = (entity.sprite.tile_id / tiles_per_row) as f32; // row in tileset

        // Source rectangle in the tileset (uses tileset_tile_size)
        let src_rect = macroquad::math::Rect {
            x: tile_x * self.tileset_tile_size,
            y: tile_y * self.tileset_tile_size,
            w: self.tileset_tile_size,
            h: self.tileset_tile_size,
        };

        macroquad::texture::draw_texture_ex(
            &self.tileset,
            screen_x,
            screen_y,
            macroquad::color::WHITE, // WHITE = no tint, draw with original colors
            macroquad::texture::DrawTextureParams {
                source: Some(src_rect),
                dest_size: Some(macroquad::math::Vec2::new(
                    self.game_tile_size,
                    self.game_tile_size,
                )),
                ..Default::default()
            },
        );
    }

    /// Draws a debug grid overlay showing tile boundaries
    pub fn draw_grid(&self) {
        let screen_w: f32 = macroquad::window::screen_width();
        let screen_h: f32 = macroquad::window::screen_height();

        // draw vertical lines
        let mut x: f32 = 0.0;
        while x < screen_w {
            shapes::draw_line(x, 0.0, x, screen_h, 1.0, macroquad::color::GRAY);
            x += self.game_tile_size;
        }

        // draw horizontal lines
        let mut y: f32 = 0.0;
        while y < screen_h {
            shapes::draw_line(0.0, y, screen_w, y, 1.0, macroquad::color::GRAY);
            y += self.game_tile_size;
        }
    }
}
