use macroquad::{color, input, text, texture, window};

mod entity;
mod game_state;
mod player;
mod renderer;
mod sprite;
mod tile;
mod ui;

#[macroquad::main("Soulband")]
async fn main() {
    let mut use_graphics: bool = true; // TODO: make configurable.

    input::show_mouse(true);
    input::set_cursor_grab(false);

    let game_tile_size: f32 = 32.0;
    let tileset_tile_size: f32 = 64.0;

    // Load font.
    let font = text::load_ttf_font_from_bytes(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/ttf/JetBrainsMono-Regular.ttf"
    )))
    .expect("Failed to load font");

    // Show loading screen
    ui::draw_loading_screen(&font);
    window::next_frame().await;

    // Load tileset. Takes few seconds in debug mode.
    let tileset = texture::load_texture("assets/tiles.png")
        .await
        .expect("Failed to load tileset");

    let renderer: renderer::Renderer =
        renderer::Renderer::new(game_tile_size, tileset_tile_size, font, tileset);

    let player: player::Player = player::Player::new();
    let mut game_state: game_state::GameState = game_state::GameState::new(player);

    loop {
        window::clear_background(color::BLACK);

        game_state.handle_input();
        renderer.draw_entity(&game_state.player.entity, use_graphics);
        renderer.draw_grid();
        ui::draw_menu_bar(&mut use_graphics, true);

        window::next_frame().await;
    }
}
