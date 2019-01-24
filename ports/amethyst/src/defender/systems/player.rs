use crate::defender::{
    entity::{ Player },
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

use amethyst::{
    core::{ transform::Transform },
    ecs::prelude::{ Join, Read, ReadStorage, System, WriteStorage },
    input::InputHandler,
};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        // What this system mutates
        WriteStorage<'s, Transform>,
        // Reads the player components
        ReadStorage<'s, Player>,
        // Also has access to the input handler
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (_player, transform) in (&players, &mut transforms).join() {
            let movement_x = input.axis_value("player_x");
            let movement_y = input.axis_value("player_y");

            // Move the player
            if let Some(mv_x) = movement_x {
                if let Some(mv_y) = movement_y {
                    // Scale movement by some factor to make the motion seem
                    // smoother
                    let scaled_x = 1.2 * mv_x as f32;
                    let scaled_y = 1.2 * mv_y as f32;

                    let player_x = transform.translation().x;
                    let player_y = transform.translation().y;

                    let width_half = WINDOW_WIDTH * 0.5;
                    let height_half = WINDOW_HEIGHT * 0.5;

                    let new_x = (player_x - scaled_x)
                        .min(width_half)
                        .max(-width_half);

                    let new_y = (player_y + scaled_y)
                        // Limit the player to the window
                        .min(height_half)
                        .max(-height_half);

                    transform.set_x(new_x);
                    transform.set_y(new_y);
                }
            }
        }
    }
}