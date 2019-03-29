
use amethyst::{
    core::{
        nalgebra::{
            geometry::UnitQuaternion,
            Vector3
        },
        transform::Transform,
    },
    ecs::prelude::{
        Component,
        DenseVecStorage,
        Join
    },
    prelude::*
};
use crate::defender::{
    config::{ PlayerConfig },
    render::{
        create_mesh,
        create_material,
        generate_triangle_vertices,
    },
};

#[derive(Debug, Default)]
pub struct Player {
    /// The direction the player is facing in radians.
    pub direction: f32,
    /// Current weapon cooldown timer
    pub weapon_cooldown: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn initialize(world: &mut World) {
        let mut player_transform = Transform::default();
        player_transform.set_xyz(0.0, 0.0, 0.0);

        let (dimensions, color) = {
            let config = &world.read_resource::<PlayerConfig>();
            (config.dimensions, config.color)
        };

        let player_mesh = create_mesh(
            world,
            generate_triangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
        );

        let player_material = create_material(world, color);

        // Create player triangle
        world.create_entity()
            .with(player_mesh)
            .with(player_material)
            .with(Player::default())
            .with(player_transform)
            .build();
    }

    pub fn reset(world: &mut World) {
        let mut players = world.write_storage::<Player>();
        let mut transforms = world.write_storage::<Transform>();
        for (player, transform) in (&mut players, &mut transforms).join() {
            player.weapon_cooldown = 0.0;
            player.direction = 0.0;

            transform.set_x(0.0);
            transform.set_y(0.0);
            transform.set_rotation(
                UnitQuaternion::from_axis_angle(
                    &Vector3::z_axis(),
                    0.0
                )
            );
        }
    }
}