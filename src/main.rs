use macroquad::{color, input, text, window};

mod entity;
mod game_state;
mod player;
mod renderer;
mod sprite;

#[macroquad::main("Soulband")]
async fn main() {
    input::show_mouse(true);
    input::set_cursor_grab(false);

    let tile_size: u32 = 32;
    let font_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/ttf/JetBrainsMono-Regular.ttf",
    ));

    let font = text::load_ttf_font_from_bytes(font_bytes).expect("Failed to load font");

    let renderer: renderer::Renderer = renderer::Renderer::new(tile_size, font);
    let player: player::Player = player::Player::new();
    let game_state: game_state::GameState = game_state::GameState::new(player);

    loop {
        window::clear_background(color::BLACK);
        renderer.draw_entity(&game_state.player.entity);
        window::next_frame().await;
    }
}
