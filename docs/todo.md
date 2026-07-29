# Soulband TODO

## Current Priority

### 1. Level Generation (See `/docs/levelgen.md` for detailed roadmap)

**Current Phase: Phase 1 - Rectangular Rooms Only (MVP)**

**WHERE WE LEFT OFF (April 15, 2026):**
We completed the design phase and are ready to start coding. Next step: implement basic types in `src/level.rs` ONE STRUCT AT A TIME (going slow so user can learn).

Start with:
1. TileType enum (Wall, Floor)
2. Tile struct (terrain, room_id)
3. GeometricShape enum (Rectangle for MVP)
4. RoomShape enum (Geometric/Template/Organic)
5. RoomTheme enum (Normal for MVP)
6. Room struct
7. Level struct
8. Level generation algorithm

**Implementation checklist:**
- [ ] Create `src/level.rs` module
- [ ] Implement TileType enum
- [ ] Implement Tile struct
- [ ] Implement GeometricShape enum (Rectangle only for now)
- [ ] Implement RoomShape enum
- [ ] Implement RoomTheme enum  
- [ ] Implement Room struct with `new_rectangle()` and `is_in_room()`
- [ ] Implement Level struct with `new()` and `generate()`
- [ ] Implement `try_place_room()` - random position search
- [ ] Implement `room_fits()` - collision check with 1-tile buffer
- [ ] Implement `place_room()` - place floor tiles on grid
- [ ] Test: Generate 80×30 level with 10-20 rooms (target `max_rooms`, stop early after 10 consecutive placement failures)
- [ ] Render rooms to screen
- [ ] Basic wizard mode (regen level, reveal map)

**Next phases** (see levelgen.md):
- Phase 2: Room shape variety (circles, crosses, overlapping, etc.)
- Phase 3: Organic features (lakes, chasms, caverns, bridges)
- Phase 4: Blend structured + organic ("buildings on organic ground")
- Phase 5: Corridors & connectivity
- Phase 6: Vaults & monster pits

---

### 2. Dungeon Rendering & Player
- [ ] Design tile system (floor, wall, door, water, chasm types)
- [ ] Render dungeon tiles to screen
- [ ] Position player within the dungeon level
- [ ] Add basic field-of-view/visibility system

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
- Add custom application logo/icon
  - Replace generic taskbar icon with custom Soulband logo
  - Icon should display when running `cargo run`

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

## Development Tools

### Tile Selector Enhancements
- Add visual indication when a tile is already used for something
  - Change box color for tiles that are already assigned (different from unassigned tiles)
  - Display usage information at bottom of screen (e.g., "This tile is used for: Goblin")
- Create monster/entity definition file (YAML/TOML)
  - Define all monsters and their assigned tiles
  - Load this data into tile selector to check tile usage
  - This will help prevent accidentally assigning the same tile to multiple entities

---

## Notes
- See `/docs/design.md` for full game design
- Tileset: 8192x2048, 64x64 tiles, 128 tiles per row
- Utility: `cargo run --bin tile_selector` to browse tileset
