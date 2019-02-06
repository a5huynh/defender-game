/// Handles spawning a new player bullet in the direction the player is facing
/// whenever the "fire" button is pressed.
use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage,
    },
};
use std::f32::consts::PI;
use crate::defender::{
    config::BulletConfig,
    entity::Bullet,
};

/// Move a bullet
pub struct MoveBulletSystem;

impl<'s> System<'s> for MoveBulletSystem {
    type SystemData = (
        // List of bullets in the system
        ReadStorage<'s, Bullet>,
        ReadExpect<'s, BulletConfig>,
        // Associated transform in the system
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (bullets, config, mut transforms, time): Self::SystemData) {
        for (bullet, transform) in (&bullets, &mut transforms).join() {
            // The direction is stored as a polar coordinate. Convert to
            // a direction vector and add to the current position.
            // Add PI / 2.0
            let x = (bullet.direction + PI / 2.0).cos();
            let y = (bullet.direction + PI / 2.0).sin();

            // Continue moving the bullet in the current direction.
            let new_x = x * config.velocity * time.delta_seconds();
            let new_y = y * config.velocity * time.delta_seconds();
            transform.translate_x(new_x);
            transform.translate_y(new_y);
        }
    }
}