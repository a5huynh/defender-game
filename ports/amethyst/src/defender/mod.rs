use amethyst::assets::Loader;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera,
    Projection,
};
use amethyst::ui::{
    Anchor,
    TtfFormat,
    UiText,
    UiTransform
};

pub mod config;
pub mod data;
pub mod entity;
mod math;
pub mod state;
mod render;
pub mod systems;

use config::{
    consts::{
        FRAC_WIN_HEIGHT_2,
        FRAC_WIN_WIDTH_2,
    },
    BulletConfig,
    PlayerConfig,
};

use entity::{
    Bullet,
    BulletResource,
    Player,
    ScoreText
};

use render::{
    create_mesh,
    create_material,
    generate_rectangle_vertices,
    generate_triangle_vertices,
};

pub fn initialize_bullet(world: &mut World) {
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

pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -FRAC_WIN_WIDTH_2,
            FRAC_WIN_WIDTH_2,
            -FRAC_WIN_HEIGHT_2,
            FRAC_WIN_HEIGHT_2,
        )))
        .with(transform)
        .build();
}

pub fn initialize_player(world: &mut World) {
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

pub fn initialize_score(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "resources/fonts/PxPlus_IBM_VGA8.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    let transform = UiTransform::new(
        "Score".to_string(),
        Anchor::TopLeft,
        // x, y, z
        85.0, -20.0, 1.0,
        // width, height
        400.0, 40.0,
        // Tab order
        0
    );

    let text = world.create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            "Score: 00000".to_string(),
            [1., 1., 1., 1.],
            25.,
        )).build();

    world.add_resource(ScoreText { text } );
}