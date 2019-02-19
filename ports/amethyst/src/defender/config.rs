use serde::{ Deserialize, Serialize };

pub mod consts {
    pub const WIN_HEIGHT: f32 = 768.0;
    pub const FRAC_WIN_HEIGHT_2: f32 = WIN_HEIGHT / 2.0;
    pub const WIN_WIDTH: f32 = 960.0;
    pub const FRAC_WIN_WIDTH_2: f32 = WIN_WIDTH / 2.0;
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BulletConfig {
    pub color: [f32; 4],
    /// width x height in pixels, of the bullet.
    pub dimensions: [f32; 2],
    /// How long the bullet lasts
    pub ttl: f32,
    pub velocity: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct EnemyConfig {
    pub dimensions: [f32; 2],
    pub ttc: f32,
    pub velocity: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub enemy_count: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PlayerConfig {
    /// width x height in pixels, of the player icon.
    pub dimensions: [f32; 2],
    /// Which way the player is initially facing.
    /// 0 = north, pi / 4 = east, etc.
    pub rotation: f32,
    pub color: [f32; 4],
    pub weapon_cooldown: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DefenderConfig {
    pub bullet: BulletConfig,
    pub enemy: EnemyConfig,
    pub game: GameConfig,
    pub player: PlayerConfig,
}