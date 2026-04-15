# Soulband - Design Document

A roguelike combining the dungeon crawling of Angband with the risk/reward mechanics of Dark Souls.

## Core Concept
Merge the deep dungeon exploration and mythology of Angband with the deliberate combat and resource management of Souls games.

---

## Confirmed Features

### Town & Economy
- **Top-level town** serves as safe hub
- **Traditional Angband shops**: General Store, Armory, Weaponsmith, Temple, Alchemy, Magic Shop, Black Market
- **Unified currency (Gold)** used for:
  - Purchasing items from shops
  - Leveling up stats at bonfire/rest point
- Currency drops when you die and must be retrieved

**Late-Game Gold Sinks:**
- **High-tier consumables** - powerful potions/elixirs that scale with character power
  - Greater healing potions, mana elixirs, temporary stat buffs
  - Prices scale to remain relevant at high levels
- **Enchantment services** - guaranteed stat bonuses or special effects added to equipment
  - Elemental damage types (fire, ice, lightning, poison)
  - Stat increases (strength, dexterity, etc.)
  - Special properties (life steal, critical chance, magic find)
  - **Exponential cost scaling**: Each enhancement level costs dramatically more
    - Early levels (+1 to +5): affordable
    - Mid levels (+10 to +15): expensive
    - High levels (+20 to +21): ludicrously expensive
  - Forces strategic choice: spread enchantments across items or max out one piece?
- **Vault keys** - expensive keys purchased in town
  - Guarantee access to artifact vaults in the dungeon
  - High gold cost but reliable way to find powerful items
- **Black Market ultra-rares** - absurdly expensive unique items
  - Legendary artifacts at 50K+ gold prices
  - Creates tension: save for safety or splurge on power?
  - Always optional but very tempting

### World & Mythology
- **Mixed mythology** monsters:
  - Tolkien creatures (orcs, trolls, balrogs, dragons, etc.)
  - Greek mythology (hydras, minotaurs, medusa, etc.)
  - Alice in Wonderland creatures
  - Angels & Demons
  - Other legendary monsters from various sources

### Dungeon Design

**Level Generation System (Angband-inspired):**
- **Randomly generated dungeons** for each run
- **Multi-phase generation algorithm**:
  1. **Basic room placement**: Place rooms of varying sizes throughout the level
  2. **Corridor connections**: Connect rooms with hallways
  3. **Special room placement**: Place vaults, monster pits, and themed rooms
  4. **Feature population**: Add doors, stairs, bonfires, and other features
  5. **Monster spawning**: Populate rooms and corridors with enemies
  6. **Loot placement**: Distribute items, gold, and artifacts

**Room Types:**
- **Standard rooms**: Rectangular rooms of various sizes (small, medium, large)
- **Irregular rooms**: Non-rectangular shapes for variety
- **Corridors**: Connecting passages between rooms
  - Not just straight lines or L-shapes
  - Minor width variations (1-2 tiles wide, occasionally 3)
  - Twists and turns for organic feel
  - Occasional wider junctions or small alcoves
- **Monster rooms**: Large chambers with significant enemy encounters
- **Treasure rooms**: Small rooms with loot-heavy spawns
- **Empty rooms**: Atmospheric but sparse (not every room has content)

**Special Room Types:**

**Vaults (Angband Tradition):**
- **Handcrafted layouts** defined in ASCII art text files
- **Tiered by difficulty**:
  - Lesser vaults: small, moderate danger, decent loot
  - Greater vaults: medium-large, high danger, excellent loot
  - Artifact vaults: rare, very dangerous, guaranteed artifacts
- **Vault characteristics**:
  - Usually have thick walls or special perimeter walls
  - May require vault keys (purchased in town) to access
  - Dense monster population
  - High-value treasure and equipment
  - Often contain unique or rare enemies
  - Can have internal structure (pillars, sub-rooms, mazes)
- **ASCII art format example**:
  ```
  ############
  #..........#
  #.##....##.#
  #.#......#.#
  #.#..$$..#.#    Legend:
  #.#......#.#    # = wall
  #.##....##.#    . = floor
  #..........#    $ = treasure
  #+########+#    + = door
  ```
- **Vault templates contain**:
  - Visual layout (walls, floors, doors, features)
  - Treasure density markers
  - Monster spawn markers (specific or themed)
  - Special terrain (water, lava, chasms)
  - Metadata: minimum depth, rarity, required keys

**Monster Pits (Themed Encounters):**
- **Thematic groups of enemies** in a contained area
- **ASCII art layouts** similar to vaults but typically:
  - Less structured/fortified (no thick walls)
  - More organic shapes
  - Focused on enemy composition over loot
  - Can be open pits or enclosed chambers
- **Pit themes examples**:
  - **Orc pit**: Various orc types (warriors, shamans, chieftains)
  - **Undead pit**: Skeletons, zombies, wraiths
  - **Dragon nest**: Multiple dragons and drakes
  - **Demon pit**: Various demonic entities
  - **Troll cave**: Different troll variants
  - **Spider den**: Spiders of all sizes
  - **Hydra pool**: Water-themed with hydras
- **Pit characteristics**:
  - Enemies all share a theme or creature type
  - Higher monster density than normal rooms
  - Loot appropriate to enemy type
  - Often positioned in high-risk areas
  - May have environmental hazards matching theme
- **Pit template format**:
  ```
  ##########
  #........####
  #o.o......o.#
  ##o....o....#
  #..oo.o.....#    o = orc spawn marker
  #.o.........#    Other metadata:
  ##.........##    - theme: orcs
  ###.....###      - min_depth: 5
    #######        - spawn_count: 8-12
  ```

**Template System Implementation:**
- **File format**: TOML or YAML with embedded ASCII art
- **Template structure**:
  ```toml
  [vault]
  name = "Small Treasury"
  type = "lesser_vault"
  min_depth = 10
  max_depth = 30
  rarity = 15  # 1-100 scale
  requires_key = false
  
  layout = """
  ############
  #..........#
  #.$$$$$$$$.$#
  #+########+#
  """
  
  [vault.legend]
  '#' = "wall"
  '.' = "floor"
  '$' = "treasure_high"
  '+' = "door"
  
  [vault.monsters]
  count = "4-8"
  theme = "treasure_guardians"
  ```
- **Tile legend mapping**: Map ASCII characters to game tiles
- **Spawn markers**: Special characters indicate where to place entities
- **Rotation and mirroring**: Templates can be randomly rotated/mirrored for variety
- **Parameterization**: Templates can have variables (monster count, treasure amount)

**Placement & Configuration:**
- **Configurable spawn frequency**: Per-vault-type setting (0.0 to 1.0 scale)
  - 0.0 = never spawns
  - 1.0 = always spawns (guaranteed per level)
  - Adjustable per vault/pit template in config files
  - **Separate from depth**: Frequency controls if it spawns, depth controls which tier/rarity
  - Addresses Angband's problem of too-rare special rooms
- **Depth effects**:
  - Shallow depths: simple treasure rooms, lesser vaults, weak monster pits
  - Deep depths: artifact vaults, greater vaults, dangerous themed pits
  - Depth affects "specialness" and difficulty, not just spawn chance
- **Spacing**: Maintain minimum distance between special rooms
- **Accessibility**: Ensure all special rooms are reachable from main dungeon path
- **Typical frequency**: 0-2 vaults and 0-3 monster pits per level (configurable)

**Dungeon Persistence & Corpse Runs:**
- **Level persistence**: Up to 2 dungeon levels cached in memory
  - **Last bonfire level**: The level containing the last bonfire you rested at
  - **Death level**: The level where you died (contains your corpse)
  - If both are the same level, only one level needs to be cached
- **After death**, you respawn in town with 0 gold
- **Corpse location**: Your gold remains at the exact spot you died
- **Enemies respawn** on cached levels (full repopulation)
- **Corpse run procedure**:
  1. Use recall to return to your last dungeon bonfire
  2. Navigate through respawned enemies to reach the level with your corpse
  3. Retrieve corpse to recover all gold
  4. High stakes: you're at 0 gold, one death away from run-over
- **Level layouts persist** until corpse is retrieved or run ends
- **New levels beyond cached ones** are generated normally if you descend further

### Loot & Equipment
- **Artifacts** - unique powerful items with fixed properties (Angband style)
- **Random armor/weapons** - procedurally generated with varying properties
  - Random stat requirements (STR, DEX, INT, FTH)
  - Random stat scaling bonuses (how much bonus damage from your stats)
  - Possible status effect buildup (bleed, poison, frost, etc.)
  - Elemental damage types (fire, ice, lightning, holy, dark)
  - Special properties and effects
- **Catalysts for magic**:
  - Staves for casting spells (INT-based)
  - Talismans/Idols for casting prayers (FTH-based)
  - May have their own stat requirements and scaling
- **Equipment determines build viability** - finding a great STR weapon early might push you toward a STR build

**Loot Economy & Gold Sources:**
- **Frequent item drops** - enemies drop equipment and consumables regularly (Angband feel)
- **Low vendor value for most items** - equipment sells for very little or can't be sold at all
  - Prevents "hoover everything and sell for safety net" gameplay
  - Keeps loot exciting for build purposes without trivializing gold management
- **Carrying capacity limits** - limited weight or inventory slots
  - Forces strategic choices: "Do I take this sword to try, or keep room for valuable finds?"
  - Prevents collecting everything to sell
  - Makes town trips more deliberate
- **Primary gold sources**:
  - **Gems** - valuable items that drop occasionally, exist purely to sell (various sizes/values)
  - **Direct gold drops** - rare but meaningful
  - **Vault treasures** - high-value items found in dangerous vaults
  - **Quest rewards** (if quests exist in the design)
- **Depth-based vendor value** - vendor prices scale with dungeon depth
  - Early dungeon items (levels 1-10): sell for 5-20 gold (almost worthless)
  - Mid dungeon items (levels 20-30): sell for 100-500 gold
  - Deep dungeon items (levels 40+): sell for 1,000+ gold
  - Scaling matches the gold threshold progression
  - Rewards risk-taking: going deep yields meaningful vendor income
  - Early grinding can't easily maintain late-game thresholds

### Combat & Resources

**Combat System:**
- **Turn-based combat** (traditional roguelike style)
- **Multi-tile entities** (large monsters):
  - Entities can occupy multiple grid cells (width × height)
  - Player is always 1×1
  - Large creatures (2×1, 2×2, etc.) physically occupy multiple spaces
  - **Movement restrictions**: creatures can only move through spaces large enough to fit them
    - Large monsters blocked by narrow hallways (tactical advantage for player)
    - Creates defensive positioning opportunities
  - **Frenzy mechanics**: when large monsters can't path to player normally
    - Enter frenzy state and begin destroying walls/terrain to reach player
    - Creates tension: narrow spaces are temporary safety, not permanent
    - Player must keep moving as monster tears down defenses
- **Speed/Energy system** (Angband-style):
  - Creatures accumulate energy based on speed stat
  - Act when they have enough energy
  - Faster creatures get multiple actions before slower ones
  - Speed modified by stats, equipment, and effects
- **Armor Class (AC) system**:
  - All armor pieces (including shields) provide AC value
  - **AC reduces damage taken** (not chance to hit)
  - **Diminishing returns curve** (exact formula TBD)
  - High AC provides meaningful protection but can't make you invincible
  - More predictable than dodge-based systems, better for permadeath stakes
- **Equipment choices**:
  - Shield in offhand: more AC/defense
  - Weapon/wand/catalyst in offhand: more damage/magic output
  - Creates tactical tradeoff between offense and defense
- **No stamina system** - actions are free, limited only by turn economy and speed
- **Blocking mechanics** (future consideration, not MVP):
  - Potential "blocking mode" stance: increases AC but reduces speed
  - Tactical choice: turtle up for defense vs. maintain mobility
  - Could be shield-specific ability

**Resources:**
- **Limited health potions**
- **Limited mana potions**
- **Bonfire mechanics**:
  - Rest at bonfire to restore health/mana
  - Resting respawns monsters
  - Used for leveling up stats (spending gold)
  - **Bonfires exist in town and throughout dungeon levels**
  - **1-2 bonfires per dungeon level**, randomly placed
  - (TBD: Are bonfires pre-placed but inactive until area is cleared, or always active?)
- **Recall mechanics** (scroll or ability):
  - **From dungeon → town**: Returns you to town bonfire
  - **From town → dungeon**: Returns you to last bonfire rested at in dungeon
  - Enables strategic back-and-forth: delve deep, recall to town to shop/bank, recall back to continue
- Strategic resource management required

**Status Effects & Buildup System** (inspired by Dark Souls):
- **Buildup mechanics**: Multiple hits accumulate status until threshold is reached, then proc occurs
- **Status types**:
  - **Bleed** - builds up, then procs for large burst damage (% of max HP)
  - **Poison** - builds up, then procs for damage over time (moderate)
  - **Rot** - builds up, then procs for heavy damage over time (stronger than poison)
  - **Frost** - builds up, then procs for burst damage + stamina damage/slow effect
  - **Burn/Fire** - damage over time, possibly from fire-infused weapons
  - **Wet** - applied by water terrain, rain, water spells
    - Increases susceptibility to lightning damage (damage multiplier)
    - If hit by cold/frost while wet: chance to be frozen solid
    - Frozen creatures lose multiple turns (stunned/immobilized)
    - Wet status can extinguish burning status
  - Other status effects possible (sleep, madness, curse, etc.)
- **Weapons and spells** can have innate status buildup values
- **Light/fast weapons** build status quickly but deal less raw damage
- **Heavy/slow weapons** build status slowly but deal more raw damage
- Creates tactical choice: raw damage vs. status application
- **Environmental interaction**: Some statuses interact with terrain and each other (wet + lightning, wet + cold = freeze)

### Character Progression

**Classless System:**
- **No fixed classes** - build emerges from stat allocation
- **Free stat allocation** at bonfires (spend gold per level)
- **Respec possible** at any time (cost TBD)
- Your effectiveness with equipment/magic depends entirely on your stats

**Core Stats** (exact list TBD, inspired by Dark Souls):
- **Strength (STR)** - required for heavy weapons, increases physical damage
- **Dexterity (DEX)** - required for finesse weapons, increases attack speed/crit
- **Intelligence (INT)** - required for sorceries, increases spell power
- **Faith (FTH)** - required for prayers/miracles, increases prayer effectiveness
- **Vitality (VIT)** - increases HP
- **Endurance (END)** - increases stamina/equipment load
- **Attunement (ATN)** - increases spell/prayer slots
- Additional stats possible: Luck, Arcane, etc.

**Equipment Requirements:**
- **Weapons have stat requirements** - e.g., "Requires 24 STR, 12 DEX"
- Can't equip or use effectively without meeting requirements
- **Scaling bonuses** - weapons get damage bonuses from matching stats
  - Greatswords: high STR scaling
  - Rapiers: high DEX scaling
  - Magic weapons: INT scaling
  - Blessed weapons: FTH scaling

**Magic System:**
- **Spells (INT-based)** require:
  - Staff equipped (catalyst)
  - Sufficient INT stat
  - Spell learned from scroll/tome
  - Attunement slot available
- **Prayers/Miracles (FTH-based)** require:
  - Talisman/Idol equipped (catalyst)
  - Sufficient FTH stat
  - Prayer learned from prayer book
  - Attunement slot available

**Leveling Costs:**
- **Exponential scaling** (inspired by Dark Souls)
  - Early levels: relatively cheap (hundreds of gold)
  - Later levels: extremely expensive (tens of thousands of gold)
  - Each level grants 1 stat point to allocate
- **The Level-Up Dilemma**:
  - Each level makes you stronger
  - But raises the cost for future levels
  - AND increases the gold threshold required to survive death
  - Over-leveling becomes a strategic risk

### Death & Run Termination

**Normal Death (gold >= threshold):**
- **Respawn in town** with 0 gold
- **Keep all equipment** - armor, weapons, items in inventory
- **Drop ALL gold** at death location (corpse)
- Must retrieve corpse to recover gold
- Corpse run is dangerous - you're at 0 gold, one death from game over

**Run-Over Death (gold < threshold):**
- **Game Over** - the run ends
- **Lose everything** - all equipment, items, and character progress
- Start a completely new run from scratch

**Gold Threshold System:**
- **Exponential scaling** tied to character level (exact formula TBD)
- Threshold scales similarly to leveling costs
  - Low levels: manageable threshold (e.g., 500-1000 gold)
  - High levels: dangerous threshold (e.g., 20,000+ gold)
- Creates multiple layers of tension:
  - Spend gold on items/enchantments to get stronger (but lower your safety net)
  - Hoard gold for safety (but stay weaker and miss opportunities)
  - Level up for stats (but increase the threshold you must maintain)
  - Splurge on vault keys or black market items (high risk, high reward)
- After any death, you're at 0 gold until you recover your corpse

**Banking & Storage:**
- **Cannot bank gold** - all gold must be carried
- **Home storage in town** - chest/stash in your home
  - Limited to **50-75 item slots** (exact number TBD)
  - Free to deposit and withdraw
  - Can store anything: artifacts, equipment, consumables, gems
  - Accessible anytime you're in town
- **Storage is per-run only** - completely wiped on run-over death
  - Does NOT provide insurance against run-over
  - Purely for inventory management within a single run
  - Useful for stashing artifacts you're not using yet, backup equipment, valuable gems before risky dives
- **Items have weight values** (for potential future weight-based mechanics)
  - Current storage limits by slot count, not weight
  - Weight may be used for carrying capacity or other mechanics later

**Strategic Implications:**
- Every death puts you at immediate risk (0 gold = one death from game over)
- Corpse runs are high-stakes: recover everything or potentially lose the run
- **Leveling up is a double-edged sword**: stronger stats vs. higher death threshold
- Over-leveling can price you out of safety - maintaining 50K gold is much harder than 1K
- Low-level challenge runs become viable strategy (keep threshold manageable)
- Equipment and artifacts become relatively more valuable (they don't raise threshold)
- Converting gold into banked items/equipment is a form of risk mitigation
- **Late-game gold sinks create constant temptation**: vault key now or save for safety?
- Black market ultra-rares are always optional but potentially game-changing
- Grinding lower levels for safety still works, but you can't out-level the risk entirely

---

## Open Questions

### Leveling & Threshold Formulas
- **Confirmed**: Both leveling costs and gold threshold use exponential scaling
- What are the exact formulas?
  - Leveling cost formula? (inspired by Dark Souls curve)
  - Death threshold formula? (should scale similarly to leveling costs)
- Should threshold be a percentage of leveling cost? Or independent curve?
- Should threshold account for dungeon depth as well as character level?
- Example target values needed:
  - Level 1-10: threshold range?
  - Level 20-30: threshold range?
  - Level 50+: threshold range?

### Enchantment System
- **Confirmed**: Exponential cost scaling per enhancement level
- What is the exact cost formula?
  - Example: +1 costs 500g, +5 costs 5K, +10 costs 50K, +20 costs ???
- Is there a maximum enhancement level cap?
- Can items lose enhancement levels or break if they fail? (or always succeed?)
- Do different item types have different base costs?
- Can enchantments stack with inherent item properties?

### Meta-Progression
- Does anything unlock permanently across runs?
- Town upgrades?
- Shop inventory expansions?
- Recipe/crafting unlocks?
- Bestiary knowledge?

### Stats & Build System
- What is the exact stat list?
- Starting stats for a new character?
- Soft caps and hard caps on stats? (diminishing returns like Dark Souls?)
- Can you respec? If so, what's the cost?
- How does stat scaling work exactly on weapons?
  - Letter grades (S/A/B/C/D/E)?
  - Percentage bonuses?

### Enemy Stats & Combat
- **Confirmed**: Turn-based combat with speed/energy system, AC reduces damage with diminishing returns
- Do enemies have stats similar to the player?
- How do enemies scale as you go deeper in dungeons?
- Resistances and weaknesses based on enemy type?
- What is the exact AC damage reduction formula (diminishing returns curve)?

**Future Consideration - Enemy Equipment (not MVP):**
- Enemies could have visible equipment that explains their stats
- "Orc wearing mithril shirt" telegraphs high AC
- Could drop their equipped items (or just increase drop chance)
- Might enable targeted farming
- Could be gated behind Intelligence/Perception stat
- **Implementation note**: Keep enemy stat system flexible to potentially add equipment layer later

**Future Consideration - Multi-Tile Targeting (not MVP):**
- For multi-tile entities, allow targeting specific tiles (regions) of the creature
- **Head/upper tile targeting**:
  - Higher damage multiplier
  - Lower hit chance
  - Possible status effects (stun, blind)
- **Body/lower tile targeting**:
  - More reliable hit chance
  - Lower damage (more armor)
  - Possible movement-impairing effects if damaged
- **Tactical depth**:
  - Risk/reward decision: reliable hits vs. high damage
  - Positional advantages (elevation) could affect targeting
  - Different weapon types might favor different targeting styles
- **Implementation considerations**:
  - Could be as simple as hit chance/damage modifiers per tile
  - Or as complex as per-tile health and armor values
  - Start simple, expand if it proves fun

**Future Consideration - Environmental Interactions (not MVP):**
Inspired by Brogue's emergent environmental gameplay:
- **Fire mechanics**:
  - Fire spreads through flammable terrain (grass, foliage, wooden structures)
  - Burns creatures standing in it (damage over time)
  - Creates smoke/obscurement
  - Fire spells and attacks can ignite terrain
  - Water extinguishes fire
- **Terrain transformation**:
  - Lava can be cooled (ice spells, water) to create walkable ground
  - Water can be frozen (ice spells) to create temporary walkable bridges
  - Ice can be melted back into water
  - Temporary bridges/platforms that may expire after time
- **Water terrain**:
  - Deep water is impassable without swimming/levitation
  - Shallow water slows movement
  - Fish or aquatic creatures inhabit water
  - Electric damage spreads through water (chain effects)
  - Can extinguish burning terrain or creatures
  - Can be frozen into walkable ice bridges
- **Gas/vapor clouds**:
  - Poison gas clouds deal damage and obscure vision
  - Flammable gases can be ignited for explosions
  - Clouds dissipate over time or blown by wind
  - Can be created by spells, items, or environment
- **Chasms and pits**:
  - **Falling into a chasm transports you to the next dungeon level** (forced descent)
  - Creates risk: unplanned descent may skip loot/bonfires
  - Can push/knock enemies into them (instant kill/removal)
  - Some creatures can fly over them
  - Bridges can be created or destroyed
  - Strategic implications: escape route vs. accidental descent
- **Tactical implications**:
  - Environmental damage affects both player and enemies
  - Creates emergent tactics (lure enemies into water, then zap with lightning)
  - Terrain can be weaponized (ignite grass between you and pursuing enemies)
  - Risk of friendly fire from environmental effects
  - Adds positioning strategy beyond simple melee range
- **Implementation note**: Start with basic floor/wall tiles, add environmental layers later

### Status Effects System
- **Confirmed**: Buildup system with proc effects (bleed, poison, rot, frost, burn)
- What are the exact buildup thresholds for each enemy type?
- How much does buildup decay over time?
- Can enemies apply status effects to the player?
- Are some enemies immune to certain status effects?
  - Undead immune to bleed/poison?
  - Fire creatures immune to burn?
- How does status buildup scale with weapon speed?
- Exact proc effects and damage values for each status?

### Progression Balance
- How to balance random loot variance with stat-based progression?
- Prevent overpowered RNG combinations?
- Ensure minimum viability (no unwinnable situations)?
- How to prevent stat-checking (where you just need X stat to win)?

---

## Debug/Wizard Mode

**Purpose**: Enable rapid testing and iteration without playing through the full game.

**Activation**: Toggle via command-line flag, config setting, or in-game key combo (e.g., Ctrl+Shift+D)

**Core Debug Commands**:

**Level & World Manipulation:**
- `wizard_goto_depth <N>` - Jump to specific dungeon depth
- `wizard_regen_level` - Regenerate current dungeon level with new layout
- `wizard_reveal_map` - Reveal entire current level map
- `wizard_spawn_bonfire` - Place bonfire at current location or cursor
- `wizard_spawn_stairs` - Place up/down stairs
- `wizard_spawn_room <type>` - Generate specific room type (vault, monster pit, etc.)

**Entity Spawning:**
- `wizard_spawn_monster <name/id> [count]` - Spawn specific monster(s) at cursor
- `wizard_spawn_monster_random <depth>` - Spawn random monster for given depth
- `wizard_kill_monsters` - Kill all monsters on current level
- `wizard_kill_target` - Kill monster under cursor

**Item & Loot:**
- `wizard_spawn_item <name/id>` - Spawn specific item at cursor
- `wizard_spawn_artifact <name/id>` - Spawn specific artifact
- `wizard_spawn_gem <value>` - Spawn gem worth X gold
- `wizard_identify_all` - Identify all items in inventory
- `wizard_max_inventory` - Fill inventory with useful items

**Player Modification:**
- `wizard_give_gold <amount>` - Add gold to player
- `wizard_set_stat <stat> <value>` - Set specific stat (STR, DEX, INT, etc.)
- `wizard_set_level <N>` - Set player level
- `wizard_set_speed <N>` - Modify player speed stat
- `wizard_heal_full` - Restore full HP and mana
- `wizard_kill_player` - Instantly kill the player (test death mechanics)
- `wizard_set_threshold <amount>` - Manually set death gold threshold
- `wizard_toggle_immortal` - Toggle invulnerability

**Status Effects & Combat:**
- `wizard_apply_status <status> [target]` - Apply status effect to player or target
- `wizard_clear_status` - Clear all status effects from player
- `wizard_set_hp <amount>` - Set player HP to specific value
- `wizard_set_mana <amount>` - Set player mana to specific value

**Equipment & Build Testing:**
- `wizard_spawn_weapon <type> [+enhancement]` - Generate weapon with specific enhancement level
- `wizard_spawn_armor <type> [+enhancement]` - Generate armor piece
- `wizard_give_spell <name>` - Learn specific spell/prayer
- `wizard_respec` - Free stat respec

**Visualization & Diagnostics:**
- `wizard_show_pathfinding` - Visualize monster pathfinding on screen
- `wizard_show_ai_state` - Display AI state for all monsters
- `wizard_show_fov` - Show field-of-view calculations
- `wizard_log_combat` - Enable detailed combat calculation logging
- `wizard_show_stats` - Display detailed stats panel
- `wizard_show_spawns` - Highlight valid spawn locations for entities

**Game Speed & Time:**
- `wizard_set_gamespeed <multiplier>` - Speed up or slow down game time
- `wizard_step_turn` - Advance exactly one game turn (for precise testing)
- `wizard_pause_ai` - Freeze all monster AI

**Corpse Run & Death Testing:**
- `wizard_spawn_corpse <gold>` - Place test corpse with specific gold amount
- `wizard_force_death` - Trigger death sequence with current gold
- `wizard_clear_corpse` - Remove existing corpse
- `wizard_goto_last_bonfire` - Teleport to last rested bonfire

**Teleportation:**
- `wizard_teleport <x> <y>` - Teleport to specific coordinates
- `wizard_teleport_cursor` - Teleport to cursor location
- `wizard_goto_town` - Return to town

**Implementation Notes:**
- Commands can be accessed via:
  - Console/command line interface (press `:` or similar)
  - Quick-access menu (press `?` in wizard mode)
  - Keybindings for frequently used commands
- All commands should be clearly marked as debug/wizard mode in UI
- Consider logging all wizard commands to a debug log file
- Wizard mode should be disabled in release builds or gated behind flag
- Some commands might have shortened aliases (e.g., `wiz_goto`, `wiz_spawn`)

**Safety & Best Practices:**
- Wizard mode saves should be clearly marked (separate save slot?)
- Option to disable wizard mode saves from counting toward achievements (if meta-progression exists)
- Clear visual indicator when wizard mode is active (colored border, status text)

---

## Content Configuration & Implementation Approach

### Data-Driven Design (inspired by Angband)

**Philosophy**: All game content should be configurable via text files (YAML/TOML format).

**Required Data Files** (for implementation):
- `monsters.yaml` - All monster stats, abilities, drops, AI behavior
- `artifacts.yaml` - Unique items with fixed properties
- `item_templates.yaml` - Weapon/armor types, base stats, possible properties, scaling
- `vaults.yaml` - ASCII art vault layouts with treasure/monster markers
- `monster_pits.yaml` - Themed encounter definitions (orc pits, dragon nests, etc.)
- `room_templates.yaml` - Standard dungeon room patterns
- `spells.yaml` - Sorceries (INT-based) with costs, effects, requirements
- `prayers.yaml` - Miracles (FTH-based) with costs, effects, requirements
- `status_effects.yaml` - Buildup rates, proc effects, durations
- `dungeon_profiles.yaml` - Per-depth generation settings (room frequency, vault probability, etc.)

**ASCII Art Template Format** (Angband-inspired):
- Visual layouts embedded in YAML/TOML files
- Rich symbol legend for dungeon features:
  - `#` = wall, `.` = floor, `+` = door (secret or normal)
  - `$` = gold/treasure, `&` = treasure or trap
  - `^` = trap, `:` = rubble, `` ` `` = lava, `/` = water
  - Numbers `0-9` = monster/loot markers with depth scaling
    - `1` = standard depth monster/item
    - `5` = 7 levels out-of-depth (OOD) treasure
    - `8` = 40 levels OOD monster + 20 levels OOD treasure (ultra rare)
  - Letters `a-z`, `A-Z` = specific monster types by symbol
- Metadata per template: `name`, `type`, `min_depth`, `max_depth`, `rarity`, `spawn_frequency` (0.0-1.0)
- Rotation/mirroring: Templates can be randomly transformed for variety

**Benefits**:
- Content addition without code changes
- Rapid balance iteration
- Community modding support
- Clear separation of data and logic
- Easy testing with wizard mode

**Implementation Priority**: Design content systems to load from config files from the start.

---

### Level Generation Implementation (Angband + Brogue Hybrid)

**Core Algorithm** (Angband approach):
1. Place rooms of various sizes
2. Connect rooms with corridors
3. Place special rooms (vaults, pits)
4. Add features (doors, stairs, bonfires)
5. Spawn monsters and loot

**Corridor Generation** (Brogue-inspired organic feel):
- Not just straight lines or L-shapes
- **Width variation**: 1-2 tiles wide, occasionally 3
- **Twists and turns**: Meandering paths using noise/randomness
- **Occasional features**:
  - Wider junctions where multiple corridors meet
  - Small alcoves (dead-ends, hiding spots)
  - Irregular shapes (less "grid-aligned" feel)
- **Algorithm considerations**:
  - Use pathfinding but add controlled randomness
  - Avoid perfect Manhattan distance paths
  - Add slight curves and bends
  - Occasionally branch corridors before reaching destination

**Special Room Placement**:
- Load vault/pit templates from `vaults.yaml` and `monster_pits.yaml`
- Filter by depth: `min_depth <= current_depth <= max_depth`
- Roll for spawn using `spawn_frequency` (0.0 = never, 1.0 = always)
- Apply rarity weighting for multiple valid templates
- Ensure spacing between special rooms
- Guarantee accessibility from main dungeon path

---

### Environmental Systems (Future/Non-MVP - Brogue-inspired)

**Purpose**: Add emergent tactical depth and environmental storytelling.

**Fire & Spreading** (inspired by Brogue):
- Fire spreads through flammable terrain (grass, wooden structures, carpets)
- Burns creatures standing in fire (damage over time)
- Creates smoke/obscurement (blocks vision)
- Fire spells and attacks can ignite terrain
- Water extinguishes fire
- **Tactical use**: Weaponize environment, block enemy paths, create diversions

**Flooding & Water Systems**:
- Flood-fill algorithms for water spreading
- Deep water is impassable without swimming/levitation
- Shallow water slows movement
- Electric damage spreads through water (chain effects)
- Water can extinguish burning terrain/creatures
- Ice spells freeze water into walkable bridges (temporary)
- **Reference**: See `BrogueCE/src/brogue/Architect.c` for flood-fill implementation

**Gas & Vapor Clouds**:
- Poison gas clouds: damage over time, obscure vision
- Flammable gases: can be ignited for explosions
- Clouds dissipate over time or with wind effects
- Created by spells, items, traps, or environmental sources
- **Visibility**: Gases should be clearly visible (animated sprites, particles)

**Terrain Transformation**:
- Lava → solid ground (via ice/water cooling)
- Water → ice bridges (via frost spells)
- Ice → water (via fire/heat)
- Walls → rubble (via monster frenzy or destruction spells)
- **Temporary effects**: Bridges/platforms may expire after time or turns

**Chasms & Vertical Traversal**:
- Falling into chasm = forced descent to next level
- Risk: unplanned descent, skip loot/bonfires
- Push/knock enemies into chasms (instant kill)
- Flying creatures can pass over chasms
- Bridges can be created or destroyed

**Tactical Implications**:
- Environmental damage affects player AND enemies (no friendly fire immunity)
- Emergent combos: Wet + lightning, fire spreading to cut off pursuit
- Positioning matters beyond melee range
- Risk of backfire (your fire spreads to you, your explosion hits you)

**Implementation Notes**:
- Start with basic floor/wall tiles (MVP)
- Add environmental layers incrementally post-MVP
- Use cellular automata for spreading effects (fire, water, gas)
- Requires per-tile state tracking (burning, wet, frozen, etc.)
- Animation system for visible spreading (important for player feedback)
- **Reference implementations**: 
  - Brogue's `Architect.c` for organic generation and flooding
  - Brogue's fire/gas spreading mechanics

---

## Design Philosophy
- **Tactical combat** - every encounter matters
- **Risk vs. reward** - push deeper or return to safety?
- **Gold as dual currency** - simultaneously your progression path and your lifeline
- **Exponential consequences** - leveling up makes you stronger but raises the stakes
- **No safe over-leveling** - you can't grind your way to safety, threshold keeps scaling
- **Classless freedom** - your build emerges organically from stat choices and equipment found
- **Stat requirements create specialization** - can't use everything, must commit to a build
- **Vulnerable corpse runs** - the most dangerous moment is right after death
- **Resource scarcity** - potions and rests are precious
- **Build variety** - stats, equipment, and artifacts create unique playstyles
- **Spending vs. hoarding tension** - every purchase decision affects your safety net
- **Learning through death** - knowledge persists even if progress doesn't
