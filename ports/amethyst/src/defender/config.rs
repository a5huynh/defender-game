use serde::{ Deserialize, Serialize };

pub mod consts {
    pub const WIN_HEIGHT: f32 = 768.0;
    pub const FRAC_WIN_HEIGHT_2: f32 = WIN_HEIGHT / 2.0;
    pub const WIN_WIDTH: f32 = 960.0;
    pub const FRAC_WIN_WIDTH_2: f32 = WIN_WIDTH / 2.0;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulletConfig {
    pub color: [f32; 4],
    /// width x height in pixels, of the bullet.
    pub dimensions: [f32; 2],
    /// How long the bullet lasts
    pub ttl: f32,
    pub velocity: f32,
}

impl Default for BulletConfig {
    fn default() -> Self {
        BulletConfig {
            color: [1.0, 1.0, 1.0, 1.0],
            dimensions: [10.0, 10.0],
            ttl: 10.0,
            velocity: 10.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnemyConfig {
    pub dimensions: [f32; 2],
    pub ttc: f32,
    pub velocity: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        EnemyConfig {
            dimensions: [0.0, 0.0],
            ttc: 0.0,
            velocity: 0.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConfig {
    /// width x height in pixels, of the player icon.
    pub dimensions: [f32; 2],
    /// Which way the player is initially facing.
    /// 0 = north, pi / 4 = east, etc.
    pub rotation: f32,
    pub color: [f32; 4],
    pub weapon_cooldown: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            dimensions: [25.0, 25.0],
            rotation: 0.0,
            color: [0.0, 1.0, 0.0, 1.0],
            weapon_cooldown: 10.0,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DefenderConfig {
    pub bullet: BulletConfig,
    pub enemy: EnemyConfig,
    pub player: PlayerConfig,
}