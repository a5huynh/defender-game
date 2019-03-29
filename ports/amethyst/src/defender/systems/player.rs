use amethyst::{
    core::{
        nalgebra::{
            geometry::UnitQuaternion,
            Vector3,
        },
        timing::Time,
        transform::Transform
    },
    ecs::prelude::{
        Entities,
        Join,
        LazyUpdate,
        Read,
        ReadExpect,
        System,
        WriteStorage
    },
    input::InputHandler,
};
use crate::defender::{
    config::consts::{
        FRAC_WIN_HEIGHT_2,
        FRAC_WIN_WIDTH_2,
    },
    config::PlayerConfig,
    entity::{ Bullet, BulletResource, Player },
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        // What this system mutates
        WriteStorage<'s, Transform>,
        // Let's us set the direction the player is facing.
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        // Entities in case we want to add a new bullet
        Entities<'s>,
        ReadExpect<'s, BulletResource>,
        ReadExpect<'s, LazyUpdate>,
        // Also has access to the input handler
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut transforms, mut players, player_config, entities, bullet_resource, lazy_update, input, time): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let movement_x = input.axis_value("player_x");
            let movement_y = input.axis_value("player_y");

            let fire_action = input.action_is_down("fire");

            if player.weapon_cooldown >= 0.0 {
                player.weapon_cooldown -= time.delta_seconds();
            }

            // Add a new bullet entity
            if let Some(fired) = fire_action {
                if fired && player.weapon_cooldown <= 0.0 {
                    // Setup cooldown
                    player.weapon_cooldown = player_config.weapon_cooldown;

                    // Add bullet
                    let entity = entities.create();
                    let bullet = Bullet::new(player);
                    let pos = transform.clone();

                    // Add new bullet to scene using LazyUpdate which queues up
                    // new entities.
                    lazy_update.insert(entity, bullet_resource.material.clone());
                    lazy_update.insert(entity, bullet_resource.mesh.clone());
                    lazy_update.insert(entity, bullet);
                    lazy_update.insert(entity, pos);
                }
            }

            // Move the player
            if let Some(mv_x) = movement_x {
                if let Some(mv_y) = movement_y {
                    // Determine direction the player is facing by
                    // calculating the angle between the up/down,
                    // left/right vector.
                    let mut rot_angle = player.direction;
                    // If the direction has change since we last moved,
                    // recalculate the angle and update our direction.
                    if mv_x.abs() > 0.0 || mv_y.abs() > 0.0 {
                        rot_angle = -1.0 * mv_x.atan2(mv_y) as f32;
                        player.direction = rot_angle;
                    }

                    let new_rotation = UnitQuaternion::from_axis_angle(
                        &Vector3::z_axis(),
                        rot_angle
                    );

                    // Scale movement by some factor to make the motion seem
                    // smoother
                    let scaled_x = 1.2 * mv_x as f32;
                    let scaled_y = 1.2 * mv_y as f32;

                    let player_x = transform.translation().x;
                    let player_y = transform.translation().y;

                    let new_x = (player_x + scaled_x)
                        .min(FRAC_WIN_WIDTH_2)
                        .max(-FRAC_WIN_WIDTH_2);

                    let new_y = (player_y + scaled_y)
                        // Limit the player to the window
                        .min(FRAC_WIN_HEIGHT_2)
                        .max(-FRAC_WIN_HEIGHT_2);

                    transform.set_x(new_x);
                    transform.set_y(new_y);
                    transform.set_rotation(new_rotation);
                }
            }
        }
    }
}