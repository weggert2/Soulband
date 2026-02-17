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
  - **Biased towards dangerous locations**:
    - Near cleared monster rooms
    - Near monster pits
    - In or near vaults
    - Far from stairs (deep in the level)
    - Dead ends and isolated areas
  - Placement rewards exploration and clearing dangerous encounters
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
  - Other status effects possible (sleep, madness, curse, etc.)
- **Weapons and spells** can have innate status buildup values
- **Light/fast weapons** build status quickly but deal less raw damage
- **Heavy/slow weapons** build status slowly but deal more raw damage
- Creates tactical choice: raw damage vs. status application

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

## Content Configuration

**Data-Driven Design** (inspired by Angband):
- **All game content should be configurable via text files**
- Content types to be data-driven:
  - **Monsters**: stats, abilities, drops, AI behavior
  - **Artifacts**: unique items with fixed properties
  - **Item templates**: weapon/armor types, base stats, possible properties
  - **Vault templates**: special room layouts and contents
  - **Room templates**: dungeon room patterns and features
  - **Spells/Prayers**: magic abilities, costs, effects
  - **Status effects**: buildup rates, proc effects, durations
- **Benefits**:
  - Easy content addition without code changes
  - Simple balance iteration
  - Community modding support
  - Clear separation of data and logic
- **Format**:
  - **General content** (monsters, items, artifacts, spells): Structured text with comments (YAML/TOML)
  - **Room/Vault templates**: ASCII art format (Angband-style)
    - Visual layout using ASCII characters
    - Can be embedded in TOML/YAML files
    - Requires custom parsing system for layout
    - Example: Draw walls as `#`, floors as `.`, doors as `+`, etc.
    - Allows intuitive visual design of dungeon features
- **Implementation note**: Design content systems to load from config files from the start

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
