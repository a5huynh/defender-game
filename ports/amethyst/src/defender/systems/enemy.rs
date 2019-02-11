use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{
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
    config::EnemyConfig,
    entity::Enemy,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        // What this system mutates
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut enemies, enemy_config, time): Self::SystemData) {
        let mut rng = rand::thread_rng();

        let width_half = WINDOW_WIDTH * 0.5;
        let height_half = WINDOW_HEIGHT * 0.5;

        for (enemy, transform) in (&mut enemies, &mut transforms).join() {
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


            let x = (enemy.direction + PI / 2.0).cos();
            let x = x * enemy_config.velocity * time.delta_seconds();

            let y = (enemy.direction + PI / 2.0).sin();
            let y = y * enemy_config.velocity * time.delta_seconds();

            // TODO: Detect hitting the boundary and automatically change to the
            // opposite direction.
            let new_x = (enemy_x + x)
                .min(width_half - enemy_config.dimensions[0])
                .max(-width_half);

            let new_y = (enemy_y + y)
                .min(height_half - enemy_config.dimensions[1])
                .max(-height_half);

            transform.set_x(new_x);
            transform.set_y(new_y);
        }
    }
}