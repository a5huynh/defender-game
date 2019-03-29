use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{
        Entities,
        Join,
        Read,
        ReadExpect,
        System,
        WriteStorage,
    },
};
use std::f32::consts::PI;
use crate::defender::{
    config::BulletConfig,
    entity::Bullet,
};

/// Handles moving a bullet continually in the direction it was fired.
pub struct MoveBulletSystem;

impl<'s> System<'s> for MoveBulletSystem {
    type SystemData = (
        Entities<'s>,
        // List of bullets in the system
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, BulletConfig>,
        // Associated transform in the system
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut bullets, config, mut transforms, time): Self::SystemData) {
        for (bullet_entity, bullet, transform) in
            (&entities, &mut bullets, &mut transforms).join() {
            if bullet.ttl < config.ttl {
                bullet.ttl += time.delta_seconds();
            } else {
                // Remove the bullet if the ttl is <= 0.0
                entities.delete(bullet_entity)
                    .expect("Unable to delete bullet entity");
                continue;
            }

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