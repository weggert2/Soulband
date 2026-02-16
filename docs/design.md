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

### World & Mythology
- **Mixed mythology** monsters:
  - Tolkien creatures (orcs, trolls, balrogs, dragons, etc.)
  - Greek mythology (hydras, minotaurs, medusa, etc.)
  - Alice in Wonderland creatures
  - Angels & Demons
  - Other legendary monsters from various sources

### Dungeon Design
- **Randomly generated dungeons** for each run
- **Special room types**:
  - Monster rooms (large encounters)
  - Monster pits (themed groups of enemies)
  - Vaults (from Angband tradition)

### Loot & Equipment
- **Artifacts** - unique powerful items (Angband style)
- **Random armor/weapons** - procedurally generated with varying stats
- Rarity tiers and special properties

### Combat & Resources
- **Limited health potions**
- **Limited mana potions**
- **Bonfire mechanics**:
  - Rest at bonfire to restore health/mana
  - Resting respawns monsters
  - Used for leveling up stats (spending souls)
- Strategic resource management required

### Death & Progression
- **Respawn on death** - death does NOT end the run
- **Drop all currency** on death
- Need to retrieve dropped souls (location/mechanism TBD)

---

## Open Questions

### Run Termination
**What causes a run to end?**
- Permadeath condition? (e.g., die while carrying no souls?)
- Voluntary exit after success?
- Boss defeat?
- Time limit?
- Other failure state?

### Death Mechanics
1. **Where do you respawn?**
   - Option A: In town (must traverse back down)
   - Option B: Last bonfire
   - Option C: Special respawn rooms on each level

2. **How do you retrieve dropped souls?**
   - Traditional Souls: bloodstain at death location (one chance)
   - Challenge: dungeons are randomly generated
   - Does the dungeon layout persist until souls are retrieved?
   - Or do souls appear at dungeon entrance/specific location?

3. **Permadeath integration**
   - What (if anything) persists between runs?
   - Character level?
   - Equipment?
   - Unlocked shops/town upgrades?
   - Knowledge/bestiary?

### Progression Balance
- How to balance random loot variance with stat-based progression?
- Prevent overpowered RNG combinations?
- Ensure minimum viability (no unwinnable situations)?

---

## Design Philosophy
- **Tactical combat** - every encounter matters
- **Risk vs. reward** - push deeper or return to safety?
- **Resource scarcity** - potions and rests are precious
- **Build variety** - stats, equipment, and artifacts create unique playstyles
- **Learning through death** - knowledge persists even if progress doesn't
