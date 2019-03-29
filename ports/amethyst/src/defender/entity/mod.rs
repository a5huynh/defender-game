use amethyst::{
    ecs::prelude::Entity,
};

mod bullet;
pub use bullet::*;
mod enemy;
pub use enemy::*;
mod player;
pub use player::*;

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
