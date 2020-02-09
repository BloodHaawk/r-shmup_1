use super::GameplayState;
use crate::components::{CameraPrefabData, GamePrefabHandles, PlayerPrefabData};

use amethyst::{
    assets::{PrefabLoader, ProgressCounter, RonFormat},
    prelude::*,
};

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<GameData>) {
        type SystemData<'s> = (PrefabLoader<'s, CameraPrefabData>, PrefabLoader<'s, PlayerPrefabData>);

        let world = data.world;
        let prefab_handles = world.exec(|(camera_loader, player_loader): SystemData| GamePrefabHandles {
            camera: camera_loader.load("prefabs/camera.ron", RonFormat, &mut self.progress_counter),
            player: player_loader.load("prefabs/player.ron", RonFormat, &mut self.progress_counter),
        });

        world.insert(prefab_handles);
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        println!(
            "Loading: {}%",
            100 * self.progress_counter.num_finished() / self.progress_counter.num_assets()
        );

        if self.progress_counter.num_failed() > 0 {
            println!("Error when loading assets.");
            return Trans::Quit;
        }

        if self.progress_counter.is_complete() {
            println!("Loading complete.");
            return Trans::Switch(Box::new(GameplayState::default()));
        }
        Trans::None
    }
}
