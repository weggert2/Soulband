use macroquad::text::Font;
use macroquad::{color, text, window};

/// Draws a centered loading screen with "Loading..." text
///
/// # Arguments
///
/// * `font` - The font to use for rendering the loading text
pub fn draw_loading_screen(font: &Font) {
    window::clear_background(color::BLACK);

    let loading_text = "Loading...";
    let text_size = 40.0;
    let text_dims = text::measure_text(loading_text, Some(font), text_size as u16, 1.0);

    text::draw_text_ex(
        loading_text,
        (window::screen_width() - text_dims.width) / 2.0,
        window::screen_height() / 2.0,
        text::TextParams {
            font: Some(font),
            font_size: text_size as u16,
            color: color::WHITE,
            ..Default::default()
        },
    );
}
