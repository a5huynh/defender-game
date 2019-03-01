use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Join,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteStorage,
    },
    ui::UiText,
};

use crate::defender::{
    config::{ EnemyConfig },
    entity::{ Bullet, Enemy, ScoreBoard, ScoreText },
    math::{ point_in_rect },
};

pub struct BulletCollision;

impl<'s> System<'s> for BulletCollision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Bullet>,
        WriteStorage<'s, Enemy>,
        ReadExpect<'s, EnemyConfig>,
        // Used to update the score.
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (
        transforms,
        mut bullets,
        mut enemies,
        enemy_config,
        mut ui_text,
        mut scoreboard,
        score_text,
    ): Self::SystemData) {
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
                    // Update the score
                    scoreboard.score += 100;
                    if let Some(text) = ui_text.get_mut(score_text.text) {
                        text.text = format!(
                            "Score: {:05}",
                            scoreboard.score
                        );
                    }
                }
            }
        }
    }
}
