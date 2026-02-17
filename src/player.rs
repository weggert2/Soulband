pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        Player { x: 0, y: 0 }
    }
}
