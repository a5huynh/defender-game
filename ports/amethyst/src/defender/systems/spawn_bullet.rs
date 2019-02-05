/// Handles spawning a new player bullet in the direction the player is facing
/// whenever the "fire" button is pressed.
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{
        Entity,
        Entities,
        LazyUpdate,
        Read,
        ReadExpect,
        System,
    },
    input::InputHandler,
};

use crate::defender::entity::{ Bullet, BulletResource };

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    /// Expected system data:
    ///     Entities:           The list of entities in the world.
    ///     BulletResource:     BulletResource we want to add.
    ///     LazyUpdate:         LazyUpdate system that queues up the entity creation.
    ///     InputHandler:       Input handler system.
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, BulletResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (entities, bullet_resource, lazy_update, input): Self::SystemData) {
        let bullet_action = input.action_is_down("fire");
        if let Some(bullet_fired) = bullet_action {
            if !bullet_fired {
                return;
            }

            let bullet:Entity = entities.create();
            // Starting position
            let pos = Transform::default();

            // Add new bullet to scene using LazyUpdate which queues up
            // new entities.
            lazy_update.insert(bullet, bullet_resource.material.clone());
            lazy_update.insert(bullet, bullet_resource.mesh.clone());
            lazy_update.insert(bullet, Bullet::default());
            lazy_update.insert(bullet, pos);
        }
    }
}