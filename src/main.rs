use macroquad::{color, input, text, texture, window};
use rand::{rngs::StdRng, SeedableRng};

mod entity;
mod game_state;
mod level;
mod player;
mod renderer;
mod sprite;
// mod tile;
mod ui;

#[macroquad::main("Soulband")]
async fn main() {
    let debug: bool = true; // TODO: make configurable.
    let mut use_graphics: bool = false; // TODO: make configurable.

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

    // Seed the RNG.
    // TODO: This should be random, not just zero.
    let mut rng: StdRng = StdRng::seed_from_u64(0);

    // Make a new level.
    // TODO: this should go elsewhere. The real workflow here will be: load title screen, let people
    // choose 'new' or 'load'. Based on what they choose, the level will be generated or loaded.
    // until we have this, we can just pop in a level.
    let mut level: level::Level = level::Level::default_size();
    level.generate(&mut rng);

    let renderer: renderer::Renderer =
        renderer::Renderer::new(game_tile_size, tileset_tile_size, font, tileset);

    let player: player::Player = player::Player::new();
    let mut game_state: game_state::GameState = game_state::GameState::new(player, level, rng);

    loop {
        window::clear_background(color::BLACK);

        game_state.handle_input();
        renderer.draw_entity(&game_state.player.entity, use_graphics);

        if debug {
            renderer.draw_grid();
        }

        ui::draw_menu_bar(&mut use_graphics, true);
        window::next_frame().await;
    }
}
