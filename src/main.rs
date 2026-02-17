use macroquad::color;
use macroquad::window;

mod game_state;
mod player;

#[macroquad::main("Soulband")]
async fn main() {
    loop {
        window::clear_background(color::BLACK);
        window::next_frame().await;
    }
}
