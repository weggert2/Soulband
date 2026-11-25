#![allow(dead_code)] // TODO: remove

use rand::rngs::StdRng;
use rand::RngExt;

/// type of terrain for a tile
#[derive(Clone, Copy)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,

    // Not all tiles are in rooms, if there is no id, it's a hallway (or other)
    pub room_id: Option<u32>,
}

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Circle {
    pub radius: u32,
}

#[derive(Debug)]
pub struct Cross {
    pub horizontal_width: u32,
    pub horizontal_height: u32,
    pub vertical_width: u32,
    pub vertical_height: u32,
}

#[derive(Debug)]
pub enum GeometricShape {
    Rectangle(Rectangle),
    Circle(Circle),
    Cross(Cross),
}

#[derive(Debug)]
pub enum RoomShape {
    Geometric(GeometricShape),
    Template(Vec<String>),
    Organic,
}

// The purpose of the room, e.g. Vault, MonsterPit, Shrine etc.
#[derive(Debug)]
pub enum RoomTheme {
    Normal,
    Vault,
    MonsterPit,
    Boss,
    Shrine,
    Lake,
    Chasm,
}

#[derive(Debug)]
pub struct BoundingBox {
    pub origin: Point,
    pub rect: Rectangle,
}

#[derive(Debug)]
pub struct Room {
    pub id: u32,
    pub shape: RoomShape,
    pub theme: RoomTheme,
    pub bounding_box: BoundingBox,
}

pub struct Level {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Room>,
}

impl Level {
    // TODO: make these configurable.
    const DEFAULT_WIDTH: u32 = 80;
    const DEFAULT_HEIGHT: u32 = 30;
    const MAX_ROOMS: u32 = 20;

    // Max size for height and width of a rectangular room
    const MIN_RECT_ROOM_SIZE: u32 = 4;
    const MAX_RECT_ROOM_SIZE: u32 = 12;

    pub fn new(width: u32, height: u32) -> Self {
        Level {
            width,
            height,
            tiles: Vec::<Tile>::new(),
            rooms: Vec::<Room>::new(),
        }
    }

    pub fn default_size() -> Self {
        Level::new(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT)
    }

    /// Generates the level's layout: fills the grid with walls, then carves rooms.
    ///
    /// Existing tiles and rooms are cleared first, so this can be called
    /// repeatedly to regenerate a level in place.
    ///
    /// # Arguments
    ///
    /// * `rng` - Seeded random number generator; the same seed produces the
    ///   same layout, which is what makes generation reproducible for testing.
    pub fn generate(&mut self, rng: &mut StdRng) {
        // Check invariants: the largest room plus a 1-tile wall border on each side must fit.
        assert!(Self::MAX_RECT_ROOM_SIZE + 2 <= self.width);
        assert!(Self::MAX_RECT_ROOM_SIZE + 2 <= self.height);

        // Blow away all the tiles, replace with walls. We'll carve out rooms and hallways.
        self.tiles = vec![
            Tile {
                tile_type: TileType::Wall,
                room_id: None,
            };
            (self.width * self.height) as usize
        ];

        self.rooms.clear();

        let rooms_placed: u32 = 0;
        let mut tries: u32 = 0;
        let max_tries: u32 = 5;
        let room_id: u32 = 0;

        while rooms_placed < Self::MAX_ROOMS && tries < max_tries {
            let height: u32 = rng.random_range(Self::MIN_RECT_ROOM_SIZE..=Self::MAX_RECT_ROOM_SIZE);
            let width: u32 = rng.random_range(Self::MIN_RECT_ROOM_SIZE..=Self::MAX_RECT_ROOM_SIZE);

            let room: Room = Room {
                id: room_id,
                shape: RoomShape::Geometric(GeometricShape::Rectangle(Rectangle { height, width })),
                theme: RoomTheme::Normal,
                bounding_box: BoundingBox {
                    origin: Point {
                        // Keep a 1-tile wall border around the map edge. The room spans
                        // origin..origin + size - 1, so the origin must be in 1..=level_size - size - 1
                        x: rng.random_range(1..self.width - width),
                        y: rng.random_range(1..self.height - height),
                    },
                    rect: Rectangle { height, width },
                },
            };

            dbg!(&room);

            // debug: exit immediately
            tries = 999;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;

    const DEFAULT_SEED: u64 = 0;

    #[test]
    fn test_default_size() {
        let level: Level = Level::default_size();
        assert_eq!(level.width, Level::DEFAULT_WIDTH);
        assert_eq!(level.height, Level::DEFAULT_HEIGHT);
        assert_eq!(level.tiles.len(), 0);
        assert_eq!(level.rooms.len(), 0);
    }

    #[test]
    fn test_generate() {
        let mut level: Level = Level::default_size();
        let mut rng: StdRng = StdRng::seed_from_u64(DEFAULT_SEED);
        level.generate(&mut rng);
    }
}
