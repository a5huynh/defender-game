use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    /// Height in pixels, of the player icon.
    pub height: f32,
    /// Width in pixels, of the player icon.
    pub width: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            height: 10.0,
            width: 10.0,
        }
    }
}