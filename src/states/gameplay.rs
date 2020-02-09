use crate::components::GamePrefabHandles;

use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

#[derive(Default)]
pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;

        let camera_handle = world.read_resource::<GamePrefabHandles>().camera.clone();
        let player_handle = world.read_resource::<GamePrefabHandles>().player.clone();

        world.create_entity().with(camera_handle).build();
        world.create_entity().with(player_handle).build();
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}
