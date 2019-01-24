pub struct Defender;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera,
    Projection,
};

mod entity;
use entity::{ Player };

mod render;
use render::{
    create_mesh,
    create_material,
    generate_rectangle_vertices,
};

pub const WINDOW_HEIGHT: f32 = 768.0;
pub const WINDOW_WIDTH: f32 = 960.0;

impl SimpleState for Defender {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
        initialize_player(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -WINDOW_WIDTH / 2.0,
            WINDOW_WIDTH / 2.0,
            -WINDOW_HEIGHT / 2.0,
            WINDOW_HEIGHT / 2.0,
        )))
        .with(transform)
        .build();
}

fn initialize_player(world: &mut World) {
    let mut player_transform = Transform::default();
    player_transform.set_xyz(0.0, 0.0, 0.0);

    let player = Player::default();

    let player_mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, player.width, player.height)
    );

    let player_material = create_material(world, [0.0, 1.0, 0.0, 1.0]);

    // Create player triangle
    world.create_entity()
        .with(player_mesh)
        .with(player_material)
        .with(player)
        .with(player_transform)
        .build();
}