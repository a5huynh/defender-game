use amethyst::{
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
    config::{ BulletConfig, EnemyConfig },
    entity::{ Bullet, Enemy },
};

pub struct BulletCollision;

impl<'s> System<'s> for BulletCollision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bullet>,
        ReadExpect<'s, BulletConfig>,
        WriteStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
    );

    fn run(&mut self, (transforms, mut bullets, _bullet_config, mut enemies, enemy_config ): Self::SystemData) {
        // Loop through each bullet location
        for (bullet_transform, bullet) in (&transforms, &mut bullets).join() {
            let bullet_x = bullet_transform.translation().x;
            let bullet_y = bullet_transform.translation().y;
            // Loop through each enemy location
            for (enemy_transform, enemy) in (&transforms, &mut enemies).join() {

                let enemy_x = enemy_transform.translation().x;
                let enemy_y = enemy_transform.translation().y;

                // Check to see if a bullet is within the enemy bounds
                if point_in_rect(
                    bullet_x,
                    bullet_y,
                    enemy_x - enemy_config.dimensions[0],
                    enemy_y - enemy_config.dimensions[1],
                    enemy_x + enemy_config.dimensions[0],
                    enemy_y + enemy_config.dimensions[1],
                ) {
                    // Destroy bullet & enemy if it's a hit.
                    bullet.ttl = 0.0;
                    enemy.is_destroyed = true;
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}