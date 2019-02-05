use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage },
    renderer::{
        Material,
        MeshHandle,
    }
};

#[derive(Debug)]
pub struct Player {
    // The direction the player is facing in radians.
    pub direction: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Bullet {
    /// Direction the bullet is going.
    pub direction: f32,
    /// Time-to-live for the bullet.
    pub ttl: f32
}

impl Default for Bullet {
    fn default() -> Self {
        Bullet {
            direction: 0.0,
            ttl: 10.0,
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