use amethyst::assets::{ Loader };
use amethyst::core::nalgebra::{
    Vector2,
    Vector3
};
use amethyst::prelude::*;
use amethyst::renderer::{
    PosTex,
    Material,
    MaterialDefaults,
    MeshHandle,
};

pub fn create_material(world: &World, color: [f32; 4]) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(color.into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

pub fn create_mesh(world: &World, vertices: Vec<PosTex>) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

pub fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: Vector3::new(left, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
    ]
}

/// Creates a triangle centered at (x, y)
pub fn generate_triangle_vertices(x: f32, y: f32, width: f32, height: f32) -> Vec<PosTex> {
    let half_height = height / 2.0;
    let half_width = width / 2.0;

    vec![
        PosTex {
            position: Vector3::new(x - half_width, y - half_height, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(x + half_width, y - half_height, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(0.0, y + half_height, 0.0),
            tex_coord: Vector2::new(0.5, 1.0),
        },
    ]
}