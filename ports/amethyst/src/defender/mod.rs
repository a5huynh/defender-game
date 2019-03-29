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
};
use entity::{
    ScoreText
};

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