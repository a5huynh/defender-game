use amethyst:: {
    core::transform::Transform,
    ecs::prelude::{
        Join,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::defender::{
    config::{ EnemyConfig, PlayerConfig },
    entity::{ Enemy, Player },
    math::{ rect_in_rect },
};

pub struct EnemyCollision;

impl<'s> System<'s> for EnemyCollision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
    );

    fn run(&mut self, (
        transforms,
        mut enemies,
        enemy_config,
        mut players,
        player_config
    ): Self::SystemData) {
        // Loop through each enemy and each player location
        for (enemy_transform, _enemy) in (&transforms, &mut enemies).join() {

            // Determine whether we've collided
            let enemy_x = enemy_transform.translation().x;
            let enemy_y = enemy_transform.translation().y;

            for (player_transform, _player) in (&transforms, &mut players).join() {

                let player_x = player_transform.translation().x;
                let player_y = player_transform.translation().y;

                if rect_in_rect(
                    enemy_x - enemy_config.dimensions[0],
                    enemy_y - enemy_config.dimensions[1],
                    enemy_x + enemy_config.dimensions[0],
                    enemy_y + enemy_config.dimensions[1],
                    player_x - player_config.dimensions[0],
                    player_y - player_config.dimensions[1],
                    player_x + player_config.dimensions[0],
                    player_y + player_config.dimensions[1],
                ) {
                    println!(
                        "COLLISION! {} {} {} {}",
                        enemy_x, enemy_y,
                        player_x, player_y
                    );
                }
            }
        }
    }
}