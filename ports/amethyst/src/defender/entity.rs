use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage },
};

#[derive(Debug)]
pub struct Player {
    /// Height in pixels, of the player icon.
    pub height: f32,
    /// Width in pixels, of the player icon.
    pub width: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            height: 25.0,
            width: 25.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}