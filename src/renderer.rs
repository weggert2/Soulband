use macroquad::shapes;
use macroquad::text::{self, Font};
use macroquad::texture::Texture2D;

use crate::entity::Entity;

/// Handles rendering of entities, tiles, and debug overlays
pub struct Renderer {
    pub game_tile_size: f32,
    pub tileset_tile_size: f32,
    pub font: Font,
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
    pub fn new(game_tile_size: f32, tileset_tile_size: f32, font: Font, tileset: Texture2D) -> Self {
        Renderer {
            game_tile_size,
            tileset_tile_size,
            font,
            tileset,
        }
    }

    /// Draws an entity's glyph at its grid position, centered within the tile
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to draw
    pub fn draw_entity(&self, entity: &Entity, use_tiles: bool) {
        let screen_x: f32 = entity.x * self.game_tile_size;
        let screen_y: f32 = entity.y * self.game_tile_size;

        if use_tiles {
            // Tileset is 8192 / 64 = 128 tiles per row
            self.draw_tile(
                (entity.sprite.tile_id % 128) as f32, // tile_x in tileset grid
                (entity.sprite.tile_id / 128) as f32, // tile_y in tileset grid
                screen_x,
                screen_y,
            );
        } else {
            // error out for now
            panic!("TODO: glyph rendering");
        }

        // let font_size: f32 = self.tile_size * entity.height;
        // let glyph_str: String = entity.sprite.glyph.to_string();

        // let text_dimensions: text::TextDimensions =
        //     text::measure_text(&glyph_str, Some(&self.font), font_size as u16, 1.0);

        // // Center horizontally and vertically
        // let x_offset: f32 = 0.5 * (self.tile_size - text_dimensions.width);
        // let y_offset: f32 =
        //     0.5 * (self.tile_size - text_dimensions.height) + text_dimensions.offset_y;

        // text::draw_text_ex(
        //     &glyph_str,
        //     screen_x + x_offset,
        //     screen_y + y_offset,
        //     text::TextParams {
        //         font: Some(&self.font),
        //         font_size: font_size as u16,
        //         color: entity.sprite.color,
        //         ..Default::default()
        //     },
        // );
    }

    /// Draws a single tile from the tileset to the screen
    ///
    /// # Arguments
    ///
    /// * `tile_x` - X coordinate in the tileset grid (which column)
    /// * `tile_y` - Y coordinate in the tileset grid (which row)
    /// * `screen_x` - X position on screen in grid coordinates (will be multiplied by tile_size)
    /// * `screen_y` - Y position on screen in grid coordinates (will be multiplied by tile_size)
    pub fn draw_tile(&self, tile_x: f32, tile_y: f32, screen_x: f32, screen_y: f32) {
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
