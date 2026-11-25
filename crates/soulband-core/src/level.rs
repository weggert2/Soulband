use rand::rngs::StdRng;
use rand::RngExt;

use crate::pos::Pos;

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
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Circle {
    pub radius: i32,
}

#[derive(Debug)]
pub struct Cross {
    pub horizontal_width: i32,
    pub horizontal_height: i32,
    pub vertical_width: i32,
    pub vertical_height: i32,
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
    pub origin: Pos,
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
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub rooms: Vec<Room>,
}

impl Level {
    // TODO: make these configurable.
    const DEFAULT_WIDTH: i32 = 80;
    const DEFAULT_HEIGHT: i32 = 30;
    const MAX_ROOMS: u32 = 20;

    // Max size for height and width of a rectangular room
    const MIN_RECT_ROOM_SIZE: i32 = 4;
    const MAX_RECT_ROOM_SIZE: i32 = 12;

    /// Creates an empty level of the given size, with no tiles or rooms until `generate` is called
    ///
    /// * `width` (`i32`) - Level width in tiles
    /// * `height` (`i32`) - Level height in tiles
    pub fn new(width: i32, height: i32) -> Self {
        Level {
            width,
            height,
            tiles: Vec::<Tile>::new(),
            rooms: Vec::<Room>::new(),
        }
    }

    /// Creates an empty level using the default dimensions (`DEFAULT_WIDTH` × `DEFAULT_HEIGHT`)
    pub fn default_size() -> Self {
        Level::new(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT)
    }

    /// Returns true if `pos` is on the map.
    ///
    /// * `pos` (`Pos`) - The position to check
    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    /// Converts a grid position into an index into `tiles`, or `None` if it's off the map.
    ///
    /// Tiles are stored row by row in one flat `Vec`: skip `y` full rows, then step `x` along.
    /// This is the only place that formula should appear.
    ///
    /// * `pos` (`Pos`) - The position to convert
    pub fn idx(&self, pos: Pos) -> Option<usize> {
        if self.in_bounds(pos) {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }

    /// Generates the level's layout: fills the grid with walls, then carves rooms.
    ///
    /// Existing tiles and rooms are cleared first, so this can be called
    /// repeatedly to regenerate a level in place.
    ///
    /// * `rng` (`&mut StdRng`) - Seeded random number generator; the same seed produces the
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
            let height: i32 = rng.random_range(Self::MIN_RECT_ROOM_SIZE..=Self::MAX_RECT_ROOM_SIZE);
            let width: i32 = rng.random_range(Self::MIN_RECT_ROOM_SIZE..=Self::MAX_RECT_ROOM_SIZE);

            let room: Room = Room {
                id: room_id,
                shape: RoomShape::Geometric(GeometricShape::Rectangle(Rectangle { height, width })),
                theme: RoomTheme::Normal,
                bounding_box: BoundingBox {
                    origin: Pos {
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

    /// A default-size level has the default dimensions and no tiles or rooms before generation
    #[test]
    fn test_default_size() {
        let level: Level = Level::default_size();
        assert_eq!(level.width, Level::DEFAULT_WIDTH);
        assert_eq!(level.height, Level::DEFAULT_HEIGHT);
        assert_eq!(level.tiles.len(), 0);
        assert_eq!(level.rooms.len(), 0);
    }

    /// Generating a default-size level with a fixed seed doesn't panic
    #[test]
    fn test_generate() {
        let mut level: Level = Level::default_size();
        let mut rng: StdRng = StdRng::seed_from_u64(DEFAULT_SEED);
        level.generate(&mut rng);
    }

    /// Corners are in bounds; one step past any edge is out of bounds
    #[test]
    fn test_in_bounds() {
        let level = Level::new(10, 5);

        // Corners are on the map.
        assert!(level.in_bounds(Pos::new(0, 0)));
        assert!(level.in_bounds(Pos::new(9, 4)));

        // One step past any edge is off the map.
        assert!(!level.in_bounds(Pos::new(-1, 0)));
        assert!(!level.in_bounds(Pos::new(0, -1)));
        assert!(!level.in_bounds(Pos::new(10, 0)));
        assert!(!level.in_bounds(Pos::new(0, 5)));
    }

    /// Positions map to row-major indices; off-map positions give `None`
    #[test]
    fn test_idx() {
        let level = Level::new(10, 5);

        assert_eq!(level.idx(Pos::new(0, 0)), Some(0));
        assert_eq!(level.idx(Pos::new(9, 0)), Some(9)); // end of first row
        assert_eq!(level.idx(Pos::new(0, 1)), Some(10)); // start of second row
        assert_eq!(level.idx(Pos::new(3, 2)), Some(23)); // 2 * 10 + 3
        assert_eq!(level.idx(Pos::new(9, 4)), Some(49)); // last tile

        assert_eq!(level.idx(Pos::new(-1, 0)), None);
        assert_eq!(level.idx(Pos::new(10, 0)), None);
    }
}
