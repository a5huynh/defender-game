use amethyst::prelude::*;
use amethyst::renderer::VirtualKeyCode;
use amethyst::input::is_key_down;

use crate::defender::data::DefenderData;

pub struct PausedMenuState;

impl<'a, 'b> State<DefenderData<'a, 'b>, StateEvent> for PausedMenuState {
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