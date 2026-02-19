use macroquad::{color, input, text, texture, window};

#[macroquad::main("Tile Selector")]
async fn main() {
    let tileset_tile_size: f32 = 64.0;
    let mut selected_tile_id: u32 = 0;
    let mut zoom: f32 = 1.0;
    let mut last_move_time: f64 = 0.0;
    let move_delay: f64 = 0.1; // Delay in seconds between moves

    // Load tileset.
    let tileset = texture::load_texture("assets/tiles.png")
        .await
        .expect("Failed to load tileset");

    // Center tileset vertically on screen
    let screen_w = window::screen_width();
    let screen_h = window::screen_height();
    let tileset_w = tileset.width();
    let tileset_h = tileset.height();
    let base_scale = (screen_w / tileset_w).min(screen_h / tileset_h);
    let scaled_h = tileset_h * base_scale;

    let mut pan_x: f32 = 0.0;
    let mut pan_y: f32 = (screen_h - scaled_h) / 2.0;

    loop {
        window::clear_background(color::BLACK);

        // Get screen dimensions
        let screen_w = window::screen_width();
        let screen_h = window::screen_height();

        // Handle zoom with mouse wheel
        let mouse_wheel = input::mouse_wheel();
        if mouse_wheel.1 != 0.0 {
            let old_zoom = zoom;
            zoom *= 1.0 + mouse_wheel.1 * 0.1;
            zoom = zoom.clamp(0.5, 10.0);

            // Adjust pan to zoom towards mouse position
            let mouse_pos = input::mouse_position();
            pan_x = mouse_pos.0 - (mouse_pos.0 - pan_x) * (zoom / old_zoom);
            pan_y = mouse_pos.1 - (mouse_pos.1 - pan_y) * (zoom / old_zoom);
        }

        // Handle panning with WASD or mouse drag
        let pan_speed = 10.0;
        if input::is_key_down(input::KeyCode::W) {
            pan_y += pan_speed;
        }
        if input::is_key_down(input::KeyCode::S) {
            pan_y -= pan_speed;
        }
        if input::is_key_down(input::KeyCode::A) {
            pan_x += pan_speed;
        }
        if input::is_key_down(input::KeyCode::D) {
            pan_x -= pan_speed;
        }

        // Handle mouse click to select tile
        if input::is_mouse_button_pressed(input::MouseButton::Left) {
            let mouse_pos = input::mouse_position();
            let tileset_w = tileset.width();
            let tileset_h = tileset.height();
            let base_scale = (screen_w / tileset_w).min(screen_h / tileset_h);
            let scale = base_scale * zoom;

            // Convert mouse position to tile coordinates
            let tile_x = ((mouse_pos.0 - pan_x) / (tileset_tile_size * scale)).floor() as i32;
            let tile_y = ((mouse_pos.1 - pan_y) / (tileset_tile_size * scale)).floor() as i32;

            // Check if click is within tileset bounds
            if tile_x >= 0 && tile_x < 128 && tile_y >= 0 && tile_y < 32 {
                selected_tile_id = (tile_y * 128 + tile_x) as u32;
            }
        }

        // Handle Shift+Up/Down for zoom
        let shift_held = input::is_key_down(input::KeyCode::LeftShift) || input::is_key_down(input::KeyCode::RightShift);

        if shift_held {
            if input::is_key_pressed(input::KeyCode::Up) {
                let old_zoom = zoom;
                zoom *= 1.2;
                zoom = zoom.clamp(0.5, 10.0);

                // Zoom towards center of screen
                let center_x = screen_w / 2.0;
                let center_y = screen_h / 2.0;
                pan_x = center_x - (center_x - pan_x) * (zoom / old_zoom);
                pan_y = center_y - (center_y - pan_y) * (zoom / old_zoom);
            }
            if input::is_key_pressed(input::KeyCode::Down) {
                let old_zoom = zoom;
                zoom /= 1.2;
                zoom = zoom.clamp(0.5, 10.0);

                // Zoom towards center of screen
                let center_x = screen_w / 2.0;
                let center_y = screen_h / 2.0;
                pan_x = center_x - (center_x - pan_x) * (zoom / old_zoom);
                pan_y = center_y - (center_y - pan_y) * (zoom / old_zoom);
            }
        }

        // Handle input for tile selection (hold to repeat with delay)
        let mut tile_changed = false;
        let current_time = macroquad::time::get_time();

        if current_time - last_move_time >= move_delay && !shift_held {
            if input::is_key_down(input::KeyCode::Right) {
                selected_tile_id += 1;
                tile_changed = true;
            }
            if input::is_key_down(input::KeyCode::Left) {
                if selected_tile_id > 0 {
                    selected_tile_id -= 1;
                    tile_changed = true;
                }
            }
            if input::is_key_down(input::KeyCode::Up) {
                if selected_tile_id >= 128 {
                    selected_tile_id -= 128; // Jump up a row
                    tile_changed = true;
                }
            }
            if input::is_key_down(input::KeyCode::Down) {
                selected_tile_id += 128; // Jump down a row
                tile_changed = true;
            }

            if tile_changed {
                last_move_time = current_time;
            }
        }

        // Auto-pan to keep selected tile visible
        if tile_changed {
            let tileset_w = tileset.width();
            let tileset_h = tileset.height();
            let base_scale = (screen_w / tileset_w).min(screen_h / tileset_h);
            let scale = base_scale * zoom;

            let tile_x = (selected_tile_id % 128) as f32;
            let tile_y = (selected_tile_id / 128) as f32;
            let tile_screen_x = pan_x + tile_x * tileset_tile_size * scale;
            let tile_screen_y = pan_y + tile_y * tileset_tile_size * scale;
            let tile_size_scaled = tileset_tile_size * scale;

            // Check if tile is off-screen and adjust pan
            let margin = 50.0; // Keep tile at least 50px from edge
            if tile_screen_x < margin {
                pan_x += margin - tile_screen_x;
            } else if tile_screen_x + tile_size_scaled > screen_w - margin {
                pan_x -= (tile_screen_x + tile_size_scaled) - (screen_w - margin);
            }

            if tile_screen_y < margin {
                pan_y += margin - tile_screen_y;
            } else if tile_screen_y + tile_size_scaled > screen_h - margin - 40.0 { // Extra margin for text at bottom
                pan_y -= (tile_screen_y + tile_size_scaled) - (screen_h - margin - 40.0);
            }
        }

        // Draw entire tileset with zoom
        let tileset_w = tileset.width();
        let tileset_h = tileset.height();
        let base_scale = (screen_w / tileset_w).min(screen_h / tileset_h);
        let scale = base_scale * zoom;

        texture::draw_texture_ex(
            &tileset,
            pan_x,
            pan_y,
            color::WHITE,
            texture::DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(tileset_w * scale, tileset_h * scale)),
                ..Default::default()
            },
        );

        // Draw rectangle around selected tile
        let tile_x = (selected_tile_id % 128) as f32;
        let tile_y = (selected_tile_id / 128) as f32;
        let rect_x = pan_x + tile_x * tileset_tile_size * scale;
        let rect_y = pan_y + tile_y * tileset_tile_size * scale;
        let rect_size = tileset_tile_size * scale;

        macroquad::shapes::draw_rectangle_lines(rect_x, rect_y, rect_size, rect_size, 3.0, color::RED);

        // Draw instructions
        text::draw_text(
            &format!("tile_id: {} | Arrows: Navigate | WASD: Pan | Shift+Up/Down or Wheel: Zoom ({:.1}x)", selected_tile_id, zoom),
            10.0,
            screen_h - 20.0,
            20.0,
            color::WHITE,
        );

        window::next_frame().await;
    }
}
