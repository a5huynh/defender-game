pub enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH
}

pub struct Position {
    pub x: f64,
    pub y: f64
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        return Position { x, y };
    }
}