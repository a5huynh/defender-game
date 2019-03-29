use amethyst:: {
    core::transform::Transform,
    ecs::prelude::{
        Join,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteStorage,
    },
};

use crate::defender::{
    config::{ EnemyConfig, PlayerConfig },
    entity::{
        CurrentPlayerState,
        Enemy,
        Player,
        PlayerState,
    },
    math::{ rect_in_rect },
};

pub struct EnemyCollision;

impl<'s> System<'s> for EnemyCollision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        Write<'s, PlayerState>,
    );

    fn run(&mut self, (
        transforms,
        enemies,
        enemy_config,
        mut players,
        player_config,
        mut player_state,
    ): Self::SystemData) {
        // Loop through each enemy and each player location
        for (enemy_transform, _enemy) in (&transforms, &enemies).join() {
            // Determine whether we've collided
            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            for (player_transform, _player) in (&transforms, &mut players).join() {

                let player_x = player_transform.translation().x;
                let player_y = player_transform.translation().y;

                if rect_in_rect(
                    enemy_x,
                    enemy_y,
                    enemy_config.dimensions,
                    player_x,
                    player_y,
                    player_config.dimensions,
                ) {
                    // Set the dead flag on the player.
                    player_state.current = CurrentPlayerState::DEAD;
                }
            }
        }
    }
}