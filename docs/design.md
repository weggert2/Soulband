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
  - Used for leveling up stats (spending gold)
- Strategic resource management required

### Character Progression
- **Exponential leveling costs** (inspired by Dark Souls)
  - Early levels: relatively cheap (hundreds of gold)
  - Later levels: extremely expensive (tens of thousands of gold)
  - Each level provides stat increases (HP, damage, etc.)
- **Allocate points to stats** at bonfires:
  - Strength, Dexterity, Intelligence, Faith, Vitality, etc. (exact stats TBD)
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
- **Can bank items** in town storage (specific mechanics TBD)
- Banked items provide a form of insurance against run-over

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

### Item Banking System
- How does item storage in town work?
- Is there a cost to store/retrieve items?
- Storage capacity limits?
- Can you retrieve banked items mid-run or only at start?
- Which items can be banked? (artifacts, equipment, consumables, or all?)

### Dungeon Persistence
- Does dungeon layout persist after death until gold is retrieved?
- Or is it regenerated each time you descend?
- If regenerated, where does the corpse/gold appear?
  - At entrance to the level where you died?
  - Randomly placed in that level?
  - Exact death location if layout persists?

### Meta-Progression
- Does anything unlock permanently across runs?
- Town upgrades?
- Shop inventory expansions?
- Recipe/crafting unlocks?
- Bestiary knowledge?

### Progression Balance
- How to balance random loot variance with stat-based progression?
- Prevent overpowered RNG combinations?
- Ensure minimum viability (no unwinnable situations)?
- Should equipment have level requirements?

---

## Design Philosophy
- **Tactical combat** - every encounter matters
- **Risk vs. reward** - push deeper or return to safety?
- **Gold as dual currency** - simultaneously your progression path and your lifeline
- **Exponential consequences** - leveling up makes you stronger but raises the stakes
- **No safe over-leveling** - you can't grind your way to safety, threshold keeps scaling
- **Vulnerable corpse runs** - the most dangerous moment is right after death
- **Resource scarcity** - potions and rests are precious
- **Build variety** - stats, equipment, and artifacts create unique playstyles
- **Spending vs. hoarding tension** - every purchase decision affects your safety net
- **Learning through death** - knowledge persists even if progress doesn't
