#![allow(dead_code)] // TODO: remove

// use egui_macroquad::egui::Rect;

/// type of terrain for a tile
pub enum TileType {
    Wall,
    Floor,
}

pub struct Tile {
    pub tile_type: TileType,

    // Not all tiles are in rooms, if there is no id, it's a hallway (or other)
    pub room_id: Option<u32>,
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub struct Circle {
    pub radius: u32,
}

pub struct Cross {
    pub horizontal_width: u32,
    pub horizontal_height: u32,
    pub vertical_width: u32,
    pub vertical_height: u32,
}

pub enum GeometricShape {
    Rectangle(Rectangle),
    Circle(Circle),
    Cross(Cross),
}
pub enum RoomShape {
    Geometric(GeometricShape),
    Template(Vec<String>),
    Organic,
}

// The purpose of the room, e.g. Vault, MonsterPit, Shrine etc.
pub enum RoomTheme {
    Normal,
    Vault,
    MonsterPit,
    Boss,
    Shrine,
    Lake,
    Chasm,
}

pub struct Room {
    pub id: u32,

    // top left of the bounding box
    pub x: u32,
    pub y: u32,

    // width, height of the bounding box
    pub width: u32,
    pub height: u32,
}

impl Room {
    fn get_bounding_box() -> bool {
        true
    }
}
