# Soulband Development Log

## 2026-02-19

Spent a bunch of time getting the characters centered in the grid. It looks good! Next up is getting the tileset to work.

![Centered glyphs rendering](2026-02-19%20123921.png)

Started working on tileset loading and rendering. Added Rust doc comments to the Renderer struct and methods. Cargo doc generates nice HTML documentation!

![Rust documentation for Renderer](2026-02-19%20164134.png)

Got tileset rendering working! The tileset uses 64x64 tiles that we scale down to 32x32 for the game grid. Player character now renders as tile_id 391.

Built a tile_selector utility (`cargo run --bin tile_selector`) to browse the tileset. It has zoom (mouse wheel or Shift+Up/Down), pan (WASD or right mouse drag), and arrow key navigation. You can click tiles to select them and see their tile_id. Really useful for finding the right tiles to use!

![Tile selector utility](2026-02-19%20173921.png)
