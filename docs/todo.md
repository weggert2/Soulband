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

### Camera/Viewport System
- Implement camera that tracks player as they move through dungeon
- Support two camera modes:
  - Edge-scrolling mode: Camera adjusts when player gets close to edge, keeping player on screen with margin (preferred)
  - Center-lock mode: Camera always centers on player
- Add keybind to recenter camera on player
- Ensure camera doesn't show areas outside dungeon bounds
- Convert between world coordinates (dungeon tiles) and screen coordinates

### Collision Detection & Turn System
- Add collision detection (can't move through walls)
- Implement turn-based system (wait for player input before advancing game state)
- Connect wait/pass turn keys to actual turn advancement

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
- Implement message log system for displaying game events
  - Show player actions ("You hit the goblin for 10 damage", "You open the door", etc.)
  - Show environmental events ("The trap activates!", "You hear a distant roar")
  - Display unhandled key presses during development
  - Scrollable history of recent messages
  - Consider color-coding by message type (combat, environment, errors, etc.)

---

## Design & Architecture (Needs Planning)

### Character Creation
- Define what's needed for character creation system
- Add specifications to design doc
- Determine character attributes, classes, or customization options

### Save System
- Determine what game state needs to be persisted (player, dungeon, inventory, etc.)
- Choose serialization approach (serde with JSON/RON/bincode, custom format, etc.)
- Design save file format and structure
- Implement save functionality (File menu -> Save)
- Implement load functionality (File menu -> Load)
- Add save file location/management (single save vs multiple saves)
- Error handling for corrupted/missing save files

### Top Menu System
- Add Save/Load option implementations (see Save System above)
- Add Help menu with Controls documentation
  - Display all keybindings (movement, actions, commands)
  - Show controls for both keyboard schemes (QWEZASD and numpad)
  - Consider allowing players to rebind keys (save preferences to config file)
- Add menu keybindings (ESC to close menus, etc.)

### UI Design (Major TODO - Needs Own Design Doc)
- Create comprehensive UI/UX design document
- Define all UI elements needed (HUD, menus, inventory, character sheet, etc.)
- Plan UI layout and visual style
- Consider accessibility and usability

---

## Learning & Exploration

### Animated Loading Screen with Threading
- Implement proper threaded asset loading with animated progress
- Learn about Rust threading: `std::thread::spawn`, channels, and GPU context limitations
- Challenge: Background thread decodes PNG while main thread shows animated "Loading..." with dots
- Requires understanding of OpenGL thread-local contexts (GPU upload must be on main thread)
- Good exercise in async/threading patterns for future features (streaming assets, background saves, etc.)

---

## Notes
- See `/docs/design.md` for full game design
- Tileset: 8192x2048, 64x64 tiles, 128 tiles per row
- Utility: `cargo run --bin tile_selector` to browse tileset
