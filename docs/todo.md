# Soulband TODO

## Current Priority

### 1. Create basic dungeon level with tiles
- Design a tile system (floor, wall, door types)
- Create a simple level data structure (2D grid)
- Implement basic level generation (start with simple rooms + corridors)
- Render the dungeon tiles to screen
- Position player within the dungeon level
- Add basic field-of-view/visibility system

---

## Next Steps (After Current Priority)

### Implement basic movement and input handling
- Add keyboard input handling (arrow keys/WASD/vi-keys)
- Implement player movement logic
- Add collision detection (can't move through walls)
- Make the game turn-based (wait for player input before advancing)
- Test movement with the current rendered player entity

### Zoom System
- Implement camera zoom in/out functionality
- Allow player to zoom in to see tile details better
- Allow player to zoom out to see more of the map
- Add keybindings for zoom controls (+/- or mouse wheel)
- Ensure UI elements scale appropriately with zoom

### Later Features
- Add basic monsters that can be seen on the map
- Implement turn-based combat system
- Add player stats (HP, basic attributes)
- Create UI overlay (show HP, gold, etc.)

---

## Design & Architecture (Needs Planning)

### Character Creation
- Define what's needed for character creation system
- Add specifications to design doc
- Determine character attributes, classes, or customization options

### Save System
- Design save/load architecture
- Determine what game state needs to be persisted
- Choose save format (JSON, binary, etc.)
- Implement save/load functionality

### Top Menu System
- Create in-game menu for saving/quitting
- Design menu UI and navigation
- Add menu keybindings (ESC, etc.)

### UI Design (Major TODO - Needs Own Design Doc)
- Create comprehensive UI/UX design document
- Define all UI elements needed (HUD, menus, inventory, character sheet, etc.)
- Plan UI layout and visual style
- Consider accessibility and usability

---

## Notes
- See `/docs/design.md` for full game design
- Current status: Tileset rendering works, player character displays with tile_id 391
- Using macroquad for graphics and window management
- Tileset: 8192x2048, 64x64 tiles, 128 tiles per row
- Utility: `cargo run --bin tile_selector` to browse tileset
