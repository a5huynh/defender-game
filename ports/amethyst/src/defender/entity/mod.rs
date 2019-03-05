use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage, Entity },
    renderer::{
        Material,
        MeshHandle,
    }
};

mod enemy;
pub use enemy::*;
mod player;
pub use player::*;

#[derive(Debug, Default)]
pub struct Bullet {
    /// Direction the bullet is going.
    pub direction: f32,
    /// Time-to-live for the bullet.
    pub ttl: f32,
}

impl Bullet {
    pub fn new(shooter: &Player) -> Self {
        Bullet {
            direction: shooter.direction,
            ttl: 0.0
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct BulletResource {
    pub material: Material,
    pub mesh: MeshHandle
}

impl Component for BulletResource {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct ScoreBoard {
    pub score: i32,
}

pub struct ScoreText {
    pub text: Entity,
}

#[derive(Clone)]
pub enum CurrentPlayerState {
    ALIVE,
    DEAD,
    RESET,
}

pub struct PlayerState {
    pub current: CurrentPlayerState,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            current: CurrentPlayerState::ALIVE
        }
    }
}
