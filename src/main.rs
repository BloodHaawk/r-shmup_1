mod components;
mod states;
mod systems;

use states::LoadingState;

use amethyst::{assets::PrefabLoaderSystemDesc, prelude::*};

use components::{CameraPrefabData, PlayerPrefabData};

use precompile::PrecompiledBundle;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Off)
        .start();

    let game_data = GameDataBuilder::new()
        .with_bundle(PrecompiledBundle {
            display_config_path: String::from("config/display.ron"),
            bindings_config_path: String::from("config/bindings.ron"),
        })?
        .with_system_desc(PrefabLoaderSystemDesc::<CameraPrefabData>::default(), "", &[])
        .with_system_desc(PrefabLoaderSystemDesc::<PlayerPrefabData>::default(), "", &[])
        // System is thread local because it must execute after InputSystem to reduce lag by 1 frame
        .with_thread_local(systems::MovePlayer { base_speed: 1000.0 });

    Application::build("assets", LoadingState::default())?
        .build(game_data)?
        .run();

    Ok(())
}
