use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage },
    renderer::{
        Material,
        MeshHandle,
    }
};

#[derive(Debug, Default)]
pub struct Enemy {
    /// Direction this enemy is going.
    pub direction: f32,
    /// Was this enemy destroyed? If so, remove from game.
    pub is_destroyed: bool,
    /// Time left til the enemy decides to change direction.
    pub ttc: f32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

pub struct EnemyResource {
    pub material: Material,
    pub mesh: MeshHandle,
}

impl Component for EnemyResource {
    type Storage = DenseVecStorage<Self>;
}

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
pub struct Player {
    /// The direction the player is facing in radians.
    pub direction: f32,
    /// Current weapon cooldown timer
    pub weapon_cooldown: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct ScoreBoard {
    pub score: i32,
}