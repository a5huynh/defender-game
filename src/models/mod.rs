
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub size: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, size: f64) -> Player {
        return Player { x, y, size, rotation: 0.0 };
    }
}