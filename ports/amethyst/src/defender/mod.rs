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
use rand::prelude::*;

pub mod config;
use config::{
    BulletConfig,
    PlayerConfig
};

mod entity;
use entity::{
    Bullet,
    BulletResource,
    Enemy,
    EnemyResource,
    Player
};

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

pub struct Defender;

impl SimpleState for Defender {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Initialize entities that exist at the beginning.
        initialize_camera(world);
        initialize_enemies(world);
        initialize_player(world);
        // Initialize resources
        initialize_bullet(world);
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

fn initialize_bullet(world: &mut World) {
    let (dimensions, color) = {
        let config = &world.read_resource::<BulletConfig>();
        (config.dimensions, config.color)
    };

    let bullet_mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
    );

    let bullet_material = create_material(world, color);
    let bullet_resource = BulletResource {
        material: bullet_material,
        mesh: bullet_mesh
    };

    // Register bullet entity & add resource so we can use it later.
    world.register::<Bullet>();
    world.add_resource(bullet_resource.clone());
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

fn initialize_enemies(world: &mut World) {
    let dimensions = [30.0, 30.0];

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
    );

    let material = create_material(world, [1.0, 0.0, 0.0, 1.0]);
    // let resource = EnemyResource { material, mesh };

    let mut rng = rand::thread_rng();
    let max_x = WINDOW_WIDTH / 2.0;
    let max_y = WINDOW_HEIGHT / 2.0;

    world.register::<Enemy>();
    for _ in 0..5 {
        let mut transform = Transform::default();
        let x = (rng.gen::<f32>() * WINDOW_WIDTH - max_x)
            .min(max_x)
            .max(-max_x);

        let y: f32 = (rng.gen::<f32>() * WINDOW_HEIGHT - max_y)
            .min(max_y)
            .max(-max_y);

        transform.set_xyz(x, y, 0.0);

        world.create_entity()
            .with(mesh.clone())
            .with(material.clone())
            .with(Enemy::default())
            .with(transform)
            .build();
    }
}

fn initialize_player(world: &mut World) {
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
        .with(Player {
            direction: 0.0,
            weapon_cooldown: 0.0
        })
        .with(player_transform)
        .build();
}