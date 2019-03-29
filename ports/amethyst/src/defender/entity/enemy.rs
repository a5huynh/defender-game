use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Component,
        DenseVecStorage,
        Join
    },
    prelude::*
};
use rand::prelude::*;
use crate::defender::{
    config::{
        consts::{
            FRAC_WIN_HEIGHT_2,
            FRAC_WIN_WIDTH_2,
            WIN_HEIGHT,
            WIN_WIDTH,
        },
        EnemyConfig,
        GameConfig,
    },
    render::{
        create_material,
        create_mesh,
        generate_rectangle_vertices,
    }
};


#[derive(Debug, Default)]
pub struct Enemy {
    /// Direction this enemy is going.
    pub direction: f32,
    /// Was this enemy destroyed? If so, remove from game.
    pub is_destroyed: bool,
    /// Time left til the enemy decides to change direction.
    pub ttc: f32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

impl Enemy {
    /// Initialize the enemies in the game. Assumes a completely empty
    /// world with no existing enemies.
    pub fn initialize(world: &mut World) {
        let mut rng = rand::thread_rng();

        let dimensions = {
            let config = &world.read_resource::<EnemyConfig>();
            config.dimensions
        };

        let num_enemies = {
            let config = &world.read_resource::<GameConfig>();
            config.enemy_count
        };

        let mesh = create_mesh(
            world,
            generate_rectangle_vertices(0.0, 0.0, dimensions[0], dimensions[1])
        );

        let material = create_material(world, [1.0, 0.0, 0.0, 1.0]);
        // let resource = EnemyResource { material, mesh };

        world.register::<Enemy>();
        for _ in 0..num_enemies {
            let mut transform = Transform::default();
            let x = (rng.gen::<f32>() * WIN_WIDTH - FRAC_WIN_WIDTH_2)
                .min(FRAC_WIN_WIDTH_2)
                .max(-FRAC_WIN_WIDTH_2);

            let y: f32 = (rng.gen::<f32>() * WIN_HEIGHT - FRAC_WIN_HEIGHT_2)
                .min(FRAC_WIN_HEIGHT_2)
                .max(-FRAC_WIN_HEIGHT_2);

            transform.set_xyz(x, y, 0.0);

            world.create_entity()
                .with(mesh.clone())
                .with(material.clone())
                .with(Enemy::default())
                .with(transform)
                .build();
        }
    }

    /// Remove all existing enemies from the state.
    pub fn remove_all(world: &mut World) {
        let enemies = world.read_storage::<Enemy>();
        let entities = world.entities();
        for (_enemy, entity) in (&enemies, &entities).join() {
            entities.delete(entity)
                .expect("unable to delete enemy entity");
        }
    }

    /// Utility function that removes all existing enemies from the game
    /// and re-initializes them
    pub fn reset(world: &mut World) {
        Enemy::remove_all(world);
        Enemy::initialize(world);
    }
}

// pub struct EnemyResource {
//     pub material: Material,
//     pub mesh: MeshHandle,
// }
//
// impl Component for EnemyResource {
//     type Storage = DenseVecStorage<Self>;
// }
