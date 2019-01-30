pub struct Defender;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera,
    Event,
    KeyboardInput,
    Projection,
    VirtualKeyCode,
    WindowEvent,
};

mod entity;
use entity::{ Player };

mod render;
use render::{
    create_mesh,
    create_material,
    generate_rectangle_vertices,
    generate_triangle_vertices,
};

pub mod systems;

pub const WINDOW_HEIGHT: f32 = 768.0;
pub const WINDOW_WIDTH: f32 = 960.0;

impl SimpleState for Defender {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
        initialize_player(world);
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => Trans::Quit,
                        WindowEvent::CloseRequested => Trans::Quit,
                        _ => Trans::None,
                    }
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    let width_half = WINDOW_WIDTH * 0.5;
    let height_half = WINDOW_HEIGHT * 0.5;

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -width_half,
            width_half,
            -height_half,
            height_half,
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
        generate_triangle_vertices(0.0, 0.0, player.width, player.height)
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