use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage },
};

#[derive(Debug)]
pub struct Player {
    // The direction the player is facing in radians.
    pub direction: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}