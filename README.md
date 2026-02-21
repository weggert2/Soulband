# Soulband

A roguelike combining the deep dungeon exploration of Angband with the risk/reward mechanics of Dark Souls.

## About

Soulband is a turn-based roguelike where every decision matters. Navigate procedurally generated dungeons filled with creatures from mixed mythologies (Tolkien, Greek myths, and more), manage scarce resources, and face the ultimate tension: spend your gold to get stronger, or hoard it to survive death.

### Key Features

- **Classless character system** - Build emerges from stat allocation and equipment found
- **Gold as dual currency** - Level up your stats *and* survive death penalties with the same resource
- **Corpse run mechanics** - Die with enough gold and you can retrieve it from your corpse. Die broke and lose everything.
- **Tactical turn-based combat** - Multi-tile entities, status effects, and deliberate positioning
- **Risk vs. reward** - Push deeper for better loot, or return to town and safety?

### Inspirations

- **Angband** - Dungeon structure, vaults, artifacts, town hub with traditional shops
- **Dark Souls** - Bonfire system, corpse runs, gold-based leveling with exponential costs
- **Traditional roguelikes** - Permadeath, procedural generation, turn-based combat

## Dependencies (Debian/Ubuntu)

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Install System Libraries
```bash
sudo apt-get update
sudo apt-get install libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev pkg-config
```

## Building and Running

```bash
cargo run --release

# Run the tile selector utility
cargo run --bin tile_selector
```
