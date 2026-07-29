# Level Generation Roadmap

**Vision**: "Buildings on organic ground" - blend Angband's structured rooms with Brogue's organic terrain features (lakes, chasms, caverns).

**Reference implementations:**
- Angband: `/coderef/angband/src/gen-room.c` - structured room builders
- Brogue: `/coderef/BrogueCE/src/brogue/Architect.c` - organic generation, bridges, chasms

---

## Dungeon Dimensions

**Reference sizes:**
- **Brogue**: 79 wide (DCOLS) × 29 tall (DROWS) - compact, tight
- **Crawl**: 80 wide (GXM) × 70 tall (GYM) - medium
- **Angband**: Variable, typically 66 wide × 198 tall using block system - large, sprawling

**Soulband decision**: **Compact like Brogue (79×29 or similar)**
- Smaller levels = more deadly, more pressure
- Forces confrontation rather than endless kiting
- Fits "tactical, deliberate" design philosophy
- Can't just run away from every fight
- Makes each room and feature more impactful
- Purely rectangular map
- Can adjust exact dimensions during implementation (maybe 80×30 for round numbers)

---

## Design Decision: Track Rooms as Entities

**Why store rooms as objects instead of just drawing on the grid?**

Retaining room information gives semantic understanding of the level, not just geometric data. Many systems benefit from knowing "what is a room" and "which room is this tile part of."

### Why Track Rooms as Entities

**Monster AI & Spawning:**
- "Spawn 3-5 orcs in this room"
- "This room is cleared, spawn a reward"
- Monsters could patrol their "home room"
- Alert monsters in adjacent rooms (room-to-room sound propagation)
- Boss enters room → "lock the doors until boss is dead"

**Gameplay Events:**
- "Player enters throne room" → spawn boss encounter
- "Player cleared this room" → XP bonus, unlock secret door
- "Room cleared" status for progression tracking
- Treasure rooms only spawn loot once (track looted state)

**Environmental Effects:**
- Flood this room with water (need boundaries)
- Fill room with poison gas
- Cast darkness on entire room
- Set room on fire (everything in it burns)

**Lighting & Atmosphere:**
- Light entire room when player enters
- "You enter a large circular chamber" vs "You are in a narrow corridor"
- Room-based ambient sound (echo in large rooms, quiet in small)
- Different lighting per room type

**Special Room Mechanics:**
- Vault tracking: "Has this vault been looted?"
- Shrine rooms: "Which god is this shrine dedicated to?"
- Puzzle rooms: "Have all switches been activated in this room?"
- Trap rooms: "Spring all traps when player enters"

**Procedural Decoration:**
- After structure generation, go back and add:
  - Furniture (tables, chairs) appropriate to room size/type
  - Wall decorations (torches, banners, carvings)
  - Debris/rubble in ruined rooms
  - Pillars in large rooms
- "This is a library" → add bookshelves
- "This is a barracks" → add beds and weapon racks

**Connectivity & Pathfinding:**
- Build room connectivity graph for AI
- "Find path from room A to room B"
- Monster alert: "There's combat in the next room over"
- Strategic AI: "Retreat to more defensible room"

**Debug & Visualization:**
- Wizard mode: highlight all rooms
- Show room types overlaid on map
- Verify all rooms are connected
- "Show me all treasure rooms"

**Save/Restore & Level Persistence:**
- "Room 3 on level 5 has been cleared"
- "Boss in throne room is dead" (for corpse run levels)
- Track which rooms have been visited/explored

### Room Data Structure Design

**Key Decision**: Use different internal representations for different room generation methods.

**Three approaches compared:**
- **Brogue**: 2D grid of booleans (good for procedural/organic generation)
- **Crawl**: Vector of strings / ASCII art (good for handcrafted templates)
- **Geometric**: Mathematical computation (efficient for simple shapes)

**Soulband approach: Hybrid - use the best representation for each purpose**

```rust
// HOW is the room shaped? (Internal representation)
enum RoomShape {
    // Geometric shapes - computed mathematically, no storage needed
    Geometric(GeometricShape),
    
    // Handcrafted templates from files (vaults, special rooms)
    Template(Vec<String>),
    
    // Procedurally generated organic features (lakes, caverns)
    Organic(OrganicFeature),
}

// Geometric shapes - just parameters, computed on demand
enum GeometricShape {
    Rectangle,  // No parameters needed (uses Room's width/height)
    Circle { radius: i32 },
    Cross { arm_width: i32, arm_length: i32 },
    Octagon { size: i32 },
    Overlap { 
        rect1: (i32, i32, i32, i32),  // x, y, w, h (relative to room origin)
        rect2: (i32, i32, i32, i32),
    },
    LShaped { /* params */ },
    // Add more as needed: Triangle, Hexagon, etc.
}

// Organic features - generated with CA/blobs, stored as grid
struct OrganicFeature {
    organic_type: OrganicType,
    grid: Vec<Vec<bool>>,  // The actual generated shape
}

enum OrganicType {
    Lake,
    Lava,
    Chasm,
    Swamp,
    RoughCavern,
    Blob,
    // Each may have different CA rules or generation parameters
}

// WHAT theme/purpose does the room have? (Semantic meaning)
enum RoomTheme {
    Normal,        // Regular dungeon room
    Vault,         // Special treasure vault
    MonsterPit,    // Themed monster encounter
    BossRoom,      // Boss encounter
    Shrine,        // Interactive shrine
    Entrance,      // Level entrance
    ThroneRoom,    // Special named room
    Lake,          // Water feature
    Chasm,         // Pit/abyss
    // More themes as needed
}

// The Room entity
struct Room {
    id: usize,
    x: i32,           // Position in dungeon (top-left of bounding box)
    y: i32,
    width: i32,       // Bounding box dimensions
    height: i32,
    theme: RoomTheme, // What is this room semantically?
    shape: RoomShape, // How is it represented?
    is_cleared: bool,
    is_special: bool,
    special_data: Option<Box<dyn SpecialRoomData>>, // Boss info, loot tables, etc.
    placed_tiles: Vec<(i32, i32)>,  // World coordinates of placed tiles (optional cache)
}

// Unified interface regardless of internal representation
impl Room {
    // Check if a position (in local/room coordinates) is part of the room's shape
    // Used DURING placement for collision checking
    fn is_in_room(&self, local_x: i32, local_y: i32) -> bool {
        match &self.shape {
            // Compute mathematically
            RoomShape::Geometric(geom_shape) => {
                match geom_shape {
                    GeometricShape::Rectangle => {
                        local_x >= 0 && local_x < self.width &&
                        local_y >= 0 && local_y < self.height
                    }
                    GeometricShape::Circle { radius } => {
                        let cx = self.width / 2;
                        let cy = self.height / 2;
                        let dx = local_x - cx;
                        let dy = local_y - cy;
                        (dx * dx + dy * dy) <= (radius * radius)
                    }
                    GeometricShape::Cross { arm_width, arm_length } => {
                        // Check if in horizontal or vertical arm
                        let cx = self.width / 2;
                        let cy = self.height / 2;
                        
                        // Horizontal arm
                        let in_h_arm = local_y >= cy - arm_width/2 && 
                                       local_y <= cy + arm_width/2;
                        
                        // Vertical arm
                        let in_v_arm = local_x >= cx - arm_width/2 && 
                                       local_x <= cx + arm_width/2;
                        
                        in_h_arm || in_v_arm
                    }
                    // ... other geometric shapes
                }
            }
            
            // Look up in ASCII art template
            RoomShape::Template(strings) => {
                if local_y < 0 || local_y >= strings.len() as i32 {
                    return false;
                }
                let row = &strings[local_y as usize];
                if local_x < 0 || local_x >= row.len() as i32 {
                    return false;
                }
                let c = row.chars().nth(local_x as usize).unwrap();
                
                // Glyph dictionary - translate char to tile type
                Self::is_floor_glyph(c)
            }
            
            // Look up in generated grid
            RoomShape::Organic(organic) => {
                if local_x < 0 || local_y < 0 {
                    return false;
                }
                organic.grid
                    .get(local_x as usize)
                    .and_then(|col| col.get(local_y as usize))
                    .copied()
                    .unwrap_or(false)
            }
        }
    }
    
    fn is_floor_glyph(c: char) -> bool {
        matches!(c, '.' | '@' | '$' | '+')  // floor, entrance, treasure, door
    }
}

struct Level {
    grid: Vec<Vec<Tile>>,
    rooms: Vec<Room>,
}

// Each tile knows which room it belongs to (if any)
struct Tile {
    terrain: TileType,
    room_id: Option<usize>,  // Links back to Room - set during placement
    // ...
}
```

### Room Placement: Two-Phase Approach

**NOTE: For MVP, you might not need all this complexity!** 

The `tile.room_id` tracking and `placed_tiles` cache enable advanced room-based mechanics (events, AI, gas filling, etc.), but **you can defer this until you actually need it**. 

**For Phase 1 (MVP):**
- Just place rectangular rooms directly on the dungeon grid
- No `room_id` tracking needed yet
- No need to store `Room` objects after placement
- Keep it simple: generate → check collision → place floors → done

**Add room tracking later when you need:**
- Room-based monster spawning
- "Player entered room" events  
- Environmental effects per room
- Room clearing mechanics
- Debug visualization

Start simple, add complexity when needed!

---

### Room Placement: Two-Phase Approach

**Phase 1: Generation & Collision Check (uses `room.is_in_room()`)**

During generation, we need to check if a room's **shape** fits:

```rust
fn room_fits(dungeon: &Grid, room: &Room, x: i32, y: i32) -> bool {
    // Check every position in the room's bounding box
    for local_x in 0..room.width {
        for local_y in 0..room.height {
            // Only check positions that are actually part of the room's shape
            if room.is_in_room(local_x, local_y) {
                let dungeon_x = x + local_x;
                let dungeon_y = y + local_y;
                
                // Check bounds
                if !in_bounds(dungeon_x, dungeon_y) { 
                    return false; 
                }
                
                // Check collision (needs 1-tile buffer)
                if dungeon[dungeon_x][dungeon_y].terrain != Wall { 
                    return false; 
                }
            }
        }
    }
    true  // No collisions, room fits!
}
```

**Phase 2: Placement & Tile Marking (sets `tile.room_id`)**

Once we've found a valid position, place the room and mark tiles:

```rust
fn place_room(dungeon: &mut Grid, room: &mut Room) {
    room.placed_tiles.clear();  // Reset cache
    
    for local_x in 0..room.width {
        for local_y in 0..room.height {
            if room.is_in_room(local_x, local_y) {
                let world_x = room.x + local_x;
                let world_y = room.y + local_y;
                
                // Place the floor tile
                dungeon[world_x][world_y].terrain = Floor;
                
                // Mark which room this tile belongs to
                dungeon[world_x][world_y].room_id = Some(room.id);
                
                // Cache world position in room (optional, for faster lookups)
                room.placed_tiles.push((world_x, world_y));
            }
        }
    }
}
```

**After Placement: Fast Lookups**

Once placed, we can quickly query room membership:

```rust
// O(1) lookup: which room is this tile in?
fn get_tile_room_id(dungeon: &Grid, x: i32, y: i32) -> Option<usize> {
    dungeon[x][y].room_id
}

// Get all tiles in a room (using cached list)
fn get_room_tiles(level: &Level, room_id: usize) -> &[(i32, i32)] {
    &level.rooms[room_id].placed_tiles
}

// Or scan the dungeon grid (slower but always accurate)
fn find_room_tiles(dungeon: &Grid, room_id: usize) -> Vec<(i32, i32)> {
    let mut tiles = Vec::new();
    for x in 0..dungeon.width {
        for y in 0..dungeon.height {
            if dungeon[x][y].room_id == Some(room_id) {
                tiles.push((x, y));
            }
        }
    }
    tiles
}

// Check if player entered a new room
fn on_player_move(level: &Level, old_pos: (i32, i32), new_pos: (i32, i32)) {
    let old_room = level.grid[old_pos.0][old_pos.1].room_id;
    let new_room = level.grid[new_pos.0][new_pos.1].room_id;
    
    if old_room != new_room {
        if let Some(room_id) = new_room {
            on_enter_room(&level.rooms[room_id]);
        }
    }
}

// Fill room with poison gas
fn fill_room_with_gas(level: &mut Level, room_id: usize) {
    for &(x, y) in &level.rooms[room_id].placed_tiles {
        level.grid[x][y].add_gas(GasType::Poison);
    }
}
```

### Why Both Approaches?

- **`room.is_in_room(local_x, local_y)`**: 
  - Used **during generation** before placement
  - Checks room's **shape** (circles, templates, etc.)
  - Needed for collision detection
  
- **`tile.room_id`**: 
  - Set **during placement**
  - Used **after generation** for gameplay
  - Fast O(1) lookups: "which room am I in?"
  - Enables room-based mechanics (events, AI, effects)

- **`room.placed_tiles`** (optional cache):
  - Faster than scanning entire dungeon
  - Useful for "do something to all tiles in room X"
  - Trade-off: memory vs. speed
```

### Generation Examples

```rust
// MVP: Geometric rectangle
fn generate_rectangular_room(width: i32, height: i32) -> Room {
    Room {
        width,
        height,
        theme: RoomTheme::Normal,
        shape: RoomShape::Geometric(GeometricShape::Rectangle),
        ...
    }
}

// Phase 2: Geometric circle
fn generate_circle_room(radius: i32) -> Room {
    let size = radius * 2 + 1;
    Room {
        width: size,
        height: size,
        theme: RoomTheme::Normal,
        shape: RoomShape::Geometric(GeometricShape::Circle { radius }),
        ...
    }
}

// Phase 6: Template-based vault
fn load_vault_from_yaml(path: &str) -> Room {
    let yaml = std::fs::read_to_string(path)?;
    let layout: Vec<String> = parse_layout_from_yaml(yaml);
    
    Room {
        width: layout[0].len() as i32,
        height: layout.len() as i32,
        theme: RoomTheme::Vault,
        shape: RoomShape::Template(layout),
        ...
    }
}

// Phase 3: Organic lake
fn generate_lake(min_size: i32, max_size: i32) -> Room {
    let width = rand_range(min_size, max_size);
    let height = rand_range(min_size, max_size);
    
    // Use cellular automata or blob generation
    let grid = cellular_automata_generate(width, height, /* params */);
    
    Room {
        width,
        height,
        theme: RoomTheme::Lake,
        shape: RoomShape::Organic(OrganicFeature {
            organic_type: OrganicType::Lake,
            grid,
        }),
        ...
    }
}
```

### Glyph Dictionary (for Template rooms)

```rust
fn glyph_to_tile_type(c: char) -> Option<TileType> {
    match c {
        '.' => Some(TileType::Floor),
        '#' => Some(TileType::Wall),
        '+' => Some(TileType::Door),
        '$' => Some(TileType::Treasure),
        '^' => Some(TileType::Trap),
        '~' => Some(TileType::Water),
        '`' => Some(TileType::Lava),
        '@' => Some(TileType::Entrance),
        ' ' => None,  // Empty/outside room
        _ => None,
    }
}
```

### Benefits of This Design

- ✅ **Memory efficient**: Rectangles/circles store ~8 bytes, not full grid
- ✅ **Fast**: Geometric shapes compute in O(1)
- ✅ **Clear separation**: Each generation method is distinct
- ✅ **Extensible**: Add new shapes without changing other types
- ✅ **Template loading**: Direct ASCII → Vec<String> conversion
- ✅ **Unified interface**: `room.is_floor()` works for all types

---

## Phase 1: Rectangular Rooms Only (MVP)

**Goal**: Get basic room placement working before adding complexity.

### Implementation Steps

- [ ] Create 2D grid data structure (dungeon map)
  - Grid of tiles: Wall, Floor, Door (later: Water, Chasm, etc.)
  - Entire map starts filled with walls
  
- [ ] Implement basic `Room` struct
  ```rust
  struct Room {
      x: i32,           // Top-left position
      y: i32,
      width: i32,       // 4-12 tiles
      height: i32,      // 3-8 tiles
      shape: RoomShape, // Start with Rectangle only
  }
  ```

- [ ] Room generation function
  - `generate_rectangular_room()` - random width/height
  - Width: 4-12, Height: 3-8 (Angband simple room sizes)

- [ ] Implement `room_fits()` checker
  - Check if room + 1-tile buffer fits without overlapping
  - All tiles in room + buffer must be solid wall
  - Must be within dungeon bounds

- [ ] Greedy placement algorithm with random sampling
  ```
  max_rooms = 20  // target, not a guarantee; density self-limits via failures
  consecutive_failures = 0
  rooms_placed = 0
  
  while rooms_placed < max_rooms and consecutive_failures < 10:
      room = generate_rectangular_room()
      
      placed = false
      for attempt in 0..100:  // 100 random position attempts
          x = random(0, dungeon_width - room.width)
          y = random(0, dungeon_height - room.height)
          
          if room_fits(dungeon, room, x, y):
              place_room(dungeon, room, x, y)
              placed = true
              break
      
      if placed:
          rooms_placed += 1
          consecutive_failures = 0
      else:
          consecutive_failures += 1
  ```

- [ ] Render rooms to screen
  - Floor tiles for room interior
  - Walls around perimeter
  - Debug view: show all rooms without corridors

- [ ] Basic wizard mode commands
  - `wizard_regen_level` - Regenerate with new layout
  - `wizard_reveal_map` - Show entire map (no fog of war)
  - Press key to iterate: generate → view → regenerate

**Success criteria**: 10-20 rectangular rooms scattered across map with 1-tile spacing. The generator targets up to `max_rooms` but stops early after 10 consecutive placement failures, so the final count self-limits by how much space is left.

---

## Phase 2: Room Shape Variety (Angband-style Structured Shapes)

**Goal**: Add all Angband room types for variety and interesting layouts.

### All Angband Room Types

**Simple rectangular rooms** ✅ (Done in Phase 1)
- Width: 2-22, Height: 2-7
- Most common room type (60-80% of rooms depending on depth)

**Circular rooms**
- Radius: 4-7
- Sometimes with hollow center (donut shape)
- Large circles sometimes have inner chamber with door + treasure
- Algorithm: Bresenham circle or flood fill from center point

**Overlapping rooms**
- Two rectangles that partially overlap
- Creates L-shapes, T-shapes, or irregular blobs
- More organic feel than pure rectangles
- Random dimensions: 4-11 wide, 3-4 tall for each rectangle

**Cross-shaped rooms** (plus/+ shape)
- 3×3 center with 4 arms extending out
- Arm dimensions: 3-11 tiles long, 3-4 tiles wide
- Can be symmetrical or asymmetrical
- Creates interesting multi-directional connectivity

**Large rooms** (fixed 23×9)
- Structured rooms with internal features:
  - Option 1: Inner room within outer room (room-in-room)
  - Option 2: Pillars (single center pillar or pillar grid)
  - Option 3: Checkerboard pattern of walls/floors
  - Option 4: Four compartments divided by walls
- Rare, impactful rooms

**Moria rooms** (variable large rooms)
- Base size: 10-15 wide, 8-13 tall
- Variations:
  - Sometimes stretched 2-3× in one dimension (long halls)
  - Rarely massive (15-30 wide, 10-20 tall) - only as first room
- Named after Mines of Moria - grand hall feeling

**Huge rooms** (45-95 wide, 30-40 tall!)
- **Very rare**: 5% chance, only as first non-staircase room
- Defines level's "main chamber" or "grand hall"
- Usually lit
- Can contain other rooms or features inside
- Makes level feel epic and varied

**Room of Chambers**
- Multiple connected sub-rooms within single large space
- Complex internal structure (maze-like)
- Sub-rooms connected by doors or passages
- Creates exploration interest

**Interesting rooms** (ASCII template-based)
- Load from `room_templates.yaml` (handcrafted designs)
- Angband has hundreds in `room_template.txt`
- Can include: zigzags, spirals, boxes, compartments, etc.
- Add after other shapes are working

### Implementation Tasks

- [ ] Add `RoomShape` enum: `Rectangle, Circle, Overlap, Cross, Large, Moria, Huge, Chambers, Template`
- [ ] Implement room design functions for each shape
- [ ] Weighted random selection system
  - Early levels: 80% rectangles, 20% other
  - Deep levels: 40% rectangles, 60% other
  - Huge rooms: 5% chance, first room only
- [ ] Update `room_fits()` to handle non-rectangular shapes
- [ ] Update `place_room()` to carve each shape type

---

## Phase 3: Organic Features (Brogue-style Natural Terrain)

**Goal**: Add natural-looking terrain between structured rooms.

### Cellular Automata (CA) for Organic Generation

**What is Cellular Automata?**

Cellular Automata (CA) is an algorithm for generating organic-looking caves, lakes, and terrain. It's very popular in roguelike dungeon generation because it creates natural, irregular shapes that don't look hand-placed or geometric.

**How it works:**

1. Start with a grid filled with random noise (some cells "alive", some "dead")
2. Repeatedly apply simple rules based on neighbor counts
3. After several iterations, random noise becomes organic blob-like shapes

**Basic Algorithm (4-5 Rule):**

```rust
// Step 1: Initialize with random noise
let mut grid = vec![vec![false; height]; width];
for x in 0..width {
    for y in 0..height {
        grid[x][y] = random() < 0.45;  // 45% chance of being alive
    }
}

// Step 2: Apply CA rules multiple times (typically 5-10 iterations)
for iteration in 0..5 {
    let mut new_grid = grid.clone();
    
    for x in 1..width-1 {
        for y in 1..height-1 {
            // Count alive neighbors in 8 directions
            let alive_neighbors = count_alive_neighbors(&grid, x, y);
            
            // Apply rules (4-5 rule):
            if grid[x][y] {
                // If alive: stay alive if 4+ neighbors alive
                new_grid[x][y] = alive_neighbors >= 4;
            } else {
                // If dead: become alive if 5+ neighbors alive
                new_grid[x][y] = alive_neighbors >= 5;
            }
        }
    }
    
    grid = new_grid;
}

fn count_alive_neighbors(grid: &Grid, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 { continue; }  // Skip self
            if grid[(x + dx) as usize][(y + dy) as usize] {
                count += 1;
            }
        }
    }
    count
}
```

**Visual Example:**

```
Iteration 0 (random noise):
..##.#..#.##
#..#.##.#...
.##...#..##.
#...##.#..#.
..#.#...##..

Iteration 1 (after one step):
..####..####
####.####...
.####.###.##
#...####.###
..######....

Iteration 5 (final - organic!):
...####.####
.#####.#####
.####...####
##...####...
.....####...
```

Notice how random noise becomes natural-looking blob shapes!

**Different Rules for Different Effects:**

- **4-5 Rule** (most common): Medium-sized caverns
  - Alive stays alive if ≥4 neighbors
  - Dead becomes alive if ≥5 neighbors
  
- **5-6 Rule**: Larger, more open caves
  - Creates bigger open spaces
  
- **3-4 Rule**: Tighter, denser passages
  - Creates more connected, maze-like areas

**Why Cellular Automata is Great:**

✅ Creates natural, organic shapes (not boxy/geometric)  
✅ No two runs look the same (procedural variety)  
✅ Simple algorithm (just neighbor counting)  
✅ Fast (only a few iterations needed)  
✅ Controllable (tweak rules for different looks)  
✅ Used by Brogue for caves, lakes, and irregular rooms

**Reference**: Brogue uses this in `Architect.c:designCavern()` for organic cave generation.

### Lakes and Pools

**Implementation** (Brogue's approach: cellular automata / blob generation):
- Generate irregular organic shapes using CA or noise
- Fill with water tiles
- Water blocks movement (unless swimming/levitating)
- Place in open spaces between room clusters
- Sizes: Small pools (5-10 tiles) to large lakes (20-50 tiles)

**Gameplay**:
- Blocks paths → forces routing around or flying over
- Can contain islands with treasure
- Thematic (water levels, flooded dungeons)

### Chasms

**Description**: Deep pits that cause forced descent
- Falling into chasm = instant transport to next dungeon level
- Lose control of when you descend (risky!)
- Can be shortcuts if you want to descend quickly
- Can be traps if you fall accidentally

**Implementation**:
- Similar to lakes: irregular organic shapes
- Mark tiles as `Chasm`
- Player/monster movement into chasm = trigger fall event
- Place strategically: near edges of rooms, in corridors

**Gameplay**:
- Risk/reward: shortcut vs. accidental fall
- Skip loot/bonfires if you fall
- Enemies can be knocked into chasms (instant kill)

### Bridges (Brogue-style)

**Description**: Walkable paths over water or chasms
- **Destructible**: Can be destroyed by attacks, explosions, or weight
- **Incentivize flight/levitation**: Alternative to using bridges
- Create tactical decisions

**Bridge Types**:

1. **Wooden Bridges**
   - Moderate durability
   - Flammable (fire spells/attacks destroy them)
   - Can break under heavy weight (large monsters)
   - Most common bridge type

2. **Glass Bridges**
   - Fragile - break easily from attacks
   - Transparent (see through to chasm/water below)
   - Shatter from explosions or heavy impacts
   - Risky to use in combat

3. **Stone Bridges**
   - Very durable
   - Hard to destroy (requires significant damage)
   - Safe for heavy creatures
   - Reliable but rare

4. **Magical Bridges**
   - **Switch-activated**: Can be toggled on/off
   - Switches can be in room, on wall, or pressure plates
   - Tactical: lure monster onto bridge, deactivate switch, drop into abyss
   - Temporary: Some appear/disappear on timers
   - May require specific keys or spells to activate

**Implementation**:
- Bridge tiles placed over water/chasms
- Each bridge type has structural integrity (hit points)
- Wooden: vulnerable to fire
- Glass: vulnerable to impacts
- Stone: high durability
- Magical: linked to switch entities
- Can collapse if damaged or deactivated
- Debris falls into chasm/water below

**Gameplay**:
- Cut off enemy pursuit by destroying bridge behind you
- Lure enemies onto bridge, then collapse it (instant kill into chasm)
- Glass bridges create tension: "Will it hold?"
- Magical bridges: solve environmental puzzles
- Find switches to activate bridges to treasure/secret areas
- Flying/levitating creatures ignore bridges entirely (advantage)

### Organic Cavern Sections

**Description**: Natural-looking cave areas using cellular automata
- Irregular, organic walls (not rectangular)
- Connect room clusters
- More interesting than straight corridors
- Use between structured room groups

**Implementation** (Brogue's `designCavern()`):
- Start with random noise or blob
- Apply cellular automata rules (4-5-rule or similar)
- Smooth edges, ensure connectivity
- Sizes: compact (10×10) to sprawling (40×30)

**Placement**:
- Between room clusters (organic "connective tissue")
- Alternative to corridor networks
- Can contain pools, chasms, or small rooms

### Implementation Tasks

- [ ] Cellular automata algorithm (4-5 rule or blob generation)
- [ ] Lake generation and placement
- [ ] Chasm generation and placement
- [ ] Bridge tile type and destruction mechanics
- [ ] Organic cavern section generation
- [ ] Blend algorithm: structured rooms + organic features
- [ ] Ensure navigability (don't block critical paths with impassable terrain)

---

## Phase 4: Room Feature Mix ("Buildings on Organic Ground")

**Goal**: Blend structured and organic for unique feel.

### Blending Algorithm

```
1. Place structured rooms (rectangles, circles, crosses)
   - Use Angband's placement algorithm
   - 10-20 rooms scattered across map
   
2. Identify open spaces between rooms
   - Flood fill to find large empty areas
   
3. Fill some gaps with organic features
   - 30-50% of gaps: lakes, chasms, rough caverns
   - Remaining gaps: leave for corridors
   
4. Add bridges over water/chasms where needed
   - Connect isolated areas
   - Create alternate routes
```

### Balance Controls

- [ ] Configurable structured vs. organic ratio
  - Early depths: More structured (80% rooms, 20% organic)
  - Deep depths: More organic (50% rooms, 50% organic)
- [ ] Per-level theme settings
  - "Constructed dungeon" levels: 90% structured
  - "Natural cave" levels: 70% organic
  - "Ruins" levels: 50/50 mix with destroyed bridges

### Navigability Checks

- [ ] Pathfinding: All rooms must be reachable
- [ ] Detect isolated areas, add connections (corridors or bridges)
- [ ] Ensure critical paths aren't blocked by impassable terrain
- [ ] Player must be able to reach stairs up/down

---

## Phase 5: Corridors & Connectivity

**Goal**: Connect rooms with interesting, organic-feeling corridors.

### Brogue-inspired Organic Corridors

**Not just straight lines or L-shapes:**
- Width variation: 1-2 tiles wide, occasionally 3
- Twists and turns using controlled randomness
- Occasional wider junctions where multiple corridors meet
- Small alcoves and dead-ends (exploration interest)

### Algorithm

```
For each unconnected room:
    1. Pick nearest unconnected room or corridor
    2. Start pathfinding (A* or similar)
    3. Add random "wobble" to path:
       - Occasionally step perpendicular to direct path
       - Vary corridor width (1-3 tiles)
       - Add small alcoves at random points
    4. Carve corridor, mark both rooms as connected
```

### Features

- [ ] Pathfinding algorithm (A* with random perturbation)
- [ ] Width variation (1-3 tiles)
- [ ] Occasional junctions (where corridors meet)
- [ ] Small alcoves and dead-ends
- [ ] Ensure all rooms are reachable from each other

---

## Phase 6: Special Rooms (Vaults & Monster Pits)

**Goal**: Place handcrafted special rooms loaded from templates.

### Vault Templates

- [ ] Load from `vaults.yaml` (ASCII art format)
- [ ] Filter by depth: `min_depth <= current_depth <= max_depth`
- [ ] Roll for spawn using `spawn_frequency` (0.0-1.0)
  - 0.0 = never spawns
  - 1.0 = always spawns (guaranteed)
  - Configurable per vault template
- [ ] Apply rarity weighting for multiple valid vaults
- [ ] Place with spacing (minimum distance from other vaults)

### Monster Pits

- [ ] Load from `monster_pits.yaml`
- [ ] Thematic encounter groups (orc pit, dragon nest, undead crypt)
- [ ] Similar placement rules as vaults
- [ ] Appropriate spacing from vaults and other pits

### Placement Priority

1. Huge rooms (if applicable)
2. Vaults (high priority, need lots of space)
3. Monster pits
4. Regular rooms
5. Organic features (fill gaps)
6. Corridors (connect everything)

---

## Implementation Notes

**Dungeon Generation Order:**
1. Initialize grid (all walls)
2. Place huge room (rare, first only)
3. Place vaults (if any spawn this level)
4. Place monster pits (if any spawn)
5. Place regular rooms (rectangles, circles, etc.)
6. Add organic features (lakes, chasms, caverns) in gaps
7. Add bridges over water/chasms
8. Connect rooms with corridors
9. Place doors at room entrances
10. Add stairs (up/down)
11. Place bonfires (1-2 per level)
12. Spawn monsters and loot

**Wizard Mode Testing:**
- Regenerate levels quickly to test variety
- Reveal map to inspect layouts
- Jump to specific depths to test depth-based generation
- Force spawn specific room types or features

---

## Future Considerations (Post-MVP)

- [ ] Multi-level vaults (span multiple dungeon levels)
- [ ] Vault keys (purchased in town, guarantee vault access)
- [ ] Destructible terrain (walls can be destroyed)
- [ ] Secret rooms (hidden behind walls, require searching)
- [ ] Themed levels (all-water level, all-lava level, ice caves, etc.)
- [ ] Dynamic terrain changes (flooding, cave-ins, earthquakes)
