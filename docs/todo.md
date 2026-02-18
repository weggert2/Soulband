# Soulband TODO

## Current Priority

### 1. Implement basic movement and input handling
- Add keyboard input handling (arrow keys/WASD/vi-keys)
- Implement player movement logic
- Add collision detection (can't move through walls)
- Make the game turn-based (wait for player input before advancing)
- Test movement with the current rendered player entity

### 2. Create basic dungeon level with tiles
- Design a tile system (floor, wall, door types)
- Create a simple level data structure (2D grid)
- Implement basic level generation (start with simple rooms + corridors)
- Render the dungeon tiles to screen
- Position player within the dungeon level
- Add basic field-of-view/visibility system

---

## Next Steps (After Current Priority)
- Add basic monsters that can be seen on the map
- Implement turn-based combat system
- Add player stats (HP, basic attributes)
- Create UI overlay (show HP, gold, etc.)

---

## Notes
- See `/docs/design.md` for full game design
- Current status: Basic rendering works, player entity displays
- Using macroquad for graphics and window management
