use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{
        Entities,
        Join,
        Read,
        ReadExpect,
        System,
        WriteStorage
    },
};
use rand::prelude::*;
use std::f32::consts::PI;

use crate::defender::{
    config::{
        consts::{
            FRAC_WIN_HEIGHT_2,
            FRAC_WIN_WIDTH_2,
        },
        EnemyConfig
    },
    entity::Enemy,
};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut transforms, mut enemies, enemy_config, time): Self::SystemData) {
        let mut rng = rand::thread_rng();

        for (enemy, transform, entity) in (&mut enemies, &mut transforms, &entities).join() {
            if enemy.is_destroyed {
                entities.delete(entity)
                    .expect("Unable to delete enemy");
                // No need to do anything else to this entity.
                continue;
            }

            // Knock down enemy direction counter.
            enemy.ttc -= time.delta_seconds();

            // Enemies change to a random direction whenver their direction
            // counter goes to 0.0
            if enemy.ttc < 0.0 {
                enemy.direction = (rng.gen::<f32>() * 2.0 * PI) - PI;
                enemy.ttc = enemy_config.ttc;
            }

            let enemy_x = transform.translation().x;
            let enemy_y = transform.translation().y;

            // Rotate by 90 degrees so that angle is aligned with 0 degrees
            // at the top.
            let x = (enemy.direction + PI / 2.0).cos();
            let x = x * enemy_config.velocity * time.delta_seconds();

            let y = (enemy.direction + PI / 2.0).sin();
            let y = y * enemy_config.velocity * time.delta_seconds();

            let scaled_x = enemy_x + x;
            let scaled_y = enemy_y + y;

            let new_x = scaled_x.min(FRAC_WIN_WIDTH_2 - enemy_config.dimensions[0])
                .max(-FRAC_WIN_WIDTH_2);

            let new_y = scaled_y.min(FRAC_WIN_HEIGHT_2 - enemy_config.dimensions[1])
                .max(-FRAC_WIN_HEIGHT_2);

            // If new_x/new_y differ we've hit the bounds.
            if new_x != scaled_x || new_y != scaled_y {
                // If the enemy hits the edge of the screen, reflect the
                // direction they're going by 180 and continue.
                enemy.direction -= PI;
                if enemy.direction <= -PI {
                    enemy.direction += 2.0 * PI;
                }
            }

            transform.set_x(new_x);
            transform.set_y(new_y);
        }
    }
}