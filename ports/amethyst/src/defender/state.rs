use amethyst::assets::Loader;
use amethyst::ecs::prelude::{ Entity };
use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;
use amethyst::input::is_key_down;
use amethyst::ui::{
    Anchor,
    TtfFormat,
    UiText,
    UiTransform,
};

use crate::defender::{
    data::DefenderData,
    entity::{ CurrentPlayerState, PlayerState },
    initialize_bullet,
    initialize_camera,
    initialize_enemies,
    initialize_player,
    initialize_score,
};

/// Game is running.
pub struct RunningState;

/// Handle running state
impl<'a, 'b> State<DefenderData<'a, 'b>, StateEvent> for RunningState {
    fn on_start(&mut self, data: StateData<DefenderData<'a, 'b>>) {
        let world = data.world;

        // Initialize entities that exist at the beginning.
        initialize_camera(world);
        initialize_enemies(world);
        initialize_player(world);
        // Initialize resources
        initialize_bullet(world);
        initialize_score(world);
    }

    fn update(&mut self, data: StateData<DefenderData>) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        // Make sure the player is still alive
        let player_state = data.world.write_resource::<PlayerState>();
        match player_state.current {
            CurrentPlayerState::DEAD => {
                return Trans::Push(
                    Box::new(DeadState {
                        death_reason: "You're dead! You're not a very good pilot.",
                        status_text: None
                    })
                );
            },
            _ => ()
        }

        Trans::None
    }

    fn handle_event(&mut self, _: StateData<DefenderData<'a, 'b>>, event: StateEvent) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState::default()));
            }
        }

        Trans::None
    }
}

/// Game is paused.
#[derive(Default)]
pub struct PausedState {
    status_text: Option<Entity>,
}

/// Handle paused state
impl<'a, 'b> State<DefenderData<'a, 'b>, StateEvent> for PausedState {
    fn on_start(&mut self, data: StateData<DefenderData<'a, 'b>>) {
        let world = data.world;

        let font = world.read_resource::<Loader>().load(
            "resources/fonts/PxPlus_IBM_VGA8.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = UiTransform::new(
            "PAUSED".to_string(),
            Anchor::TopMiddle,
            // x, y, z
            0.0, -20.0, 1.0,
            // width, height
            400.0, 40.0,
            // Tab order
            0
        );

        let entity = world.create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "Paused: Press ENTER to Resume".to_string(),
                [1., 1., 1., 1.],
                25.,
            )).build();

        self.status_text = Some(entity);
    }

    fn on_stop(&mut self, data: StateData<DefenderData<'a, 'b>>) {
        let world = data.world;
        if let Some(entity) = self.status_text {
            world.delete_entity(entity)
                .expect("unable to remove status text");
        }
    }

    fn update(&mut self, data: StateData<DefenderData<'a, 'b>>) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }

    fn handle_event(&mut self, _: StateData<DefenderData<'a, 'b>>, event: StateEvent) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}

pub struct DeadState {
    death_reason: &'static str,
    status_text: Option<Entity>,
}

// Handle dead state
impl<'a, 'b> State<DefenderData<'a, 'b>, StateEvent> for DeadState {
    fn on_start(&mut self, data: StateData<DefenderData<'a, 'b>>) {
        let world = data.world;

        let font = world.read_resource::<Loader>().load(
            "resources/fonts/PxPlus_IBM_VGA8.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = UiTransform::new(
            "DEAD".to_string(),
            Anchor::Middle,
            // x, y, z
            0.0, -20.0, 1.0,
            // width, height
            1000.0, 40.0,
            // Tab order
            0
        );

        let entity = world.create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                self.death_reason.to_string(),
                [1., 1., 1., 1.],
                25.,
            )).build();

        self.status_text = Some(entity);
    }

    fn on_stop(&mut self, data: StateData<DefenderData<'a, 'b>>) {
        let world = data.world;
        if let Some(entity) = self.status_text {
            world.delete_entity(entity)
                .expect("unable to remove status text");
        }
    }

    fn update(&mut self, data: StateData<DefenderData<'a, 'b>>) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);
        Trans::None
    }

    fn handle_event(&mut self, _: StateData<DefenderData<'a, 'b>>, event: StateEvent) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Return) {
                // Reset game.
                return Trans::Pop;
            }
        }

        Trans::None
    }
}