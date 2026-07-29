/// type of terrain for a tyle
pub enum TileType {
    Wall,
    Floor,
}

pub struct Tile {
    pub tile_type: TileType,
    pub room_id: u32,
}
