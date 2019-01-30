use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    /// Height in pixels, of the player icon.
    pub height: f32,
    /// Width in pixels, of the player icon.
    pub width: f32,
    /// Which way the player is initially facing.
    /// 0 = north, pi / 4 = east, etc.
    pub rotation: f32,
    pub color: [f32; 4],
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            height: 25.0,
            width: 25.0,
            rotation: 0.0,
            color: [0.0, 1.0, 0.0, 1.0]
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DefenderConfig {
    pub player: PlayerConfig,
}