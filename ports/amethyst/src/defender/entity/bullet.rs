use amethyst::{
    ecs::prelude::{
        Component,
        DenseVecStorage ,
    },
    prelude::*,
    renderer::{
        Material,
        MeshHandle,
    }
};
use crate::defender::{
    config::BulletConfig,
    entity::Player,
    render::{
        create_material,
        create_mesh,
        generate_rectangle_vertices,
    },
};

#[derive(Debug, Default)]
pub struct Bullet {
    /// Direction the bullet is going.
    pub direction: f32,
    /// Time-to-live for the bullet.
    pub ttl: f32,
}

impl Bullet {
    pub fn initialize(world: &mut World) {
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

    pub fn new(shooter: &Player) -> Self {
        Bullet {
            direction: shooter.direction,
            ttl: 0.0
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct BulletResource {
    pub material: Material,
    pub mesh: MeshHandle
}

impl Component for BulletResource {
    type Storage = DenseVecStorage<Self>;
}
