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

pub struct Point {
    pub x: u32,
    pub y: u32,
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

pub struct BoundingBox {
    pub origin: Point,
    pub rect: Rectangle,
}

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
