use crate::defender::{
    entity::{ Player },
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

use amethyst::{
    core::{
        nalgebra::{
            geometry::UnitQuaternion,
            Vector3,
        },
        transform::Transform
    },
    ecs::prelude::{ Join, Read, ReadStorage, System, WriteStorage },
    input::InputHandler,
};
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        // What this system mutates
        WriteStorage<'s, Transform>,
        // Let's us set the direction the player is facing.
        WriteStorage<'s, Player>,
        // Also has access to the input handler
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, mut players, input): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let movement_x = input.axis_value("player_x");
            let movement_y = input.axis_value("player_y");

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

                    let width_half = WINDOW_WIDTH * 0.5;
                    let height_half = WINDOW_HEIGHT * 0.5;

                    let new_x = (player_x + scaled_x)
                        .min(width_half)
                        .max(-width_half);

                    let new_y = (player_y + scaled_y)
                        // Limit the player to the window
                        .min(height_half)
                        .max(-height_half);

                    transform.set_x(new_x);
                    transform.set_y(new_y);
                    transform.set_rotation(new_rotation);
                }
            }
        }
    }
}