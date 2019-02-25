use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;
use amethyst::input::is_key_down;

use crate::defender::{
    data::DefenderData,
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
        Trans::None
    }

    fn handle_event(&mut self, _: StateData<DefenderData<'a, 'b>>, event: StateEvent) -> Trans<DefenderData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PausedState));
            }
        }

        Trans::None
    }
}

/// Game is paused.
pub struct PausedState;

/// Handle paused state
impl<'a, 'b> State<DefenderData<'a, 'b>, StateEvent> for PausedState {
    fn on_start(&mut self, _data: StateData<DefenderData<'a, 'b>>) {
        println!("Pausing game.");
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