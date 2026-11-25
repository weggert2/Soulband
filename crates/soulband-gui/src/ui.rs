use egui_macroquad::egui;
use macroquad::text::Font;
use macroquad::{color, text, window};

/// Draws a centered loading screen with "Loading..." text
///
/// * `font` (`&Font`) - The font to use for rendering the loading text
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

/// Draws the top menu bar with File and Options menus
///
/// Renders a menu bar at the top of the window using egui. Currently includes:
/// - File menu with Save (TODO), Load (TODO), and Quit options
/// - Options menu with Use Graphics option (TODO)
///
/// The Quit option will immediately exit the application. Save and Load options
/// are placeholders pending implementation of serialization system.
///
/// * `use_graphics` (`&mut bool`) - Current state of the use_graphics setting; toggled when the
///   menu item is clicked
/// * `use_fonts` (`bool`) - If true, switch egui to the JetBrains Mono font (configured once)
pub fn draw_menu_bar(use_graphics: &mut bool, use_fonts: bool) {
    egui_macroquad::ui(|egui_ctx| {
        // Configure egui fonts on first call if use_fonts is true
        if use_fonts {
            load_fonts_once(egui_ctx);
        }

        egui::TopBottomPanel::top("menu_bar").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        // TODO: Implement save functionality
                        ui.close_menu();
                    }
                    if ui.button("Load").clicked() {
                        // TODO: Implement load functionality
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
                ui.menu_button("Options", |ui| {
                    let check_mark = if *use_graphics { "✓ " } else { "  " };
                    let label = format!("{}Use Graphics", check_mark);
                    if ui.button(label).clicked() {
                        *use_graphics = !*use_graphics;
                    }
                });
            });
        });
    });

    egui_macroquad::draw();
}

/// Switches egui to the JetBrains Mono font; only does the work on the first call
///
/// * `egui_ctx` (`&egui::Context`) - The egui context to configure
fn load_fonts_once(egui_ctx: &egui::Context) {
    static FONTS_CONFIGURED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    if !FONTS_CONFIGURED.load(std::sync::atomic::Ordering::Relaxed) {
        let mut fonts = egui::FontDefinitions::default();

        // Load JetBrainsMono font data
        fonts.font_data.insert(
            "jetbrains".to_owned(),
            egui::FontData::from_static(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../assets/fonts/ttf/JetBrainsMono-Regular.ttf"
            )))
            .into(),
        );

        // Set JetBrainsMono as the first priority font
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "jetbrains".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "jetbrains".to_owned());

        egui_ctx.set_fonts(fonts);
        FONTS_CONFIGURED.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
