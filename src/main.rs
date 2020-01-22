mod components;
mod systems;

use amethyst::{
    assets::{Handle, Loader},
    core::Transform,
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat},
};

use components::Player;

use precompile::PrecompiledBundle;

struct MyState {
    spritesheet: Option<Handle<SpriteSheet>>,
}

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.spritesheet = Some(load_sprite_sheet(world, "characters.png", "characters.ron"));

        initialise_camera(world);

        let sprite_render = SpriteRender {
            sprite_sheet: self.spritesheet.clone().unwrap(),
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(sprite_render)
            .with(Transform::default())
            .with(Player)
            .build();
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", amethyst::LogLevelFilter::Off)
        .start();

    let game_data = GameDataBuilder::default()
        .with_bundle(PrecompiledBundle {
            display_config_path: String::from("config/display.ron"),
            bindings_config_path: String::from("config/bindings.ron"),
        })?
        .with(
            systems::MovePlayer { speed: 200.0 },
            "move_player_system",
            &["input_system"],
        );

    let mut game = Application::new("assets", MyState { spritesheet: None }, game_data)?;
    game.run();

    Ok(())
}

fn initialise_camera(world: &mut World) {
    let mut camera_transform = Transform::default();
    camera_transform.set_translation_xyz(250.0, 250.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(500.0, 500.0))
        .with(camera_transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, texture_path: &str, spritesheet_path: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(texture_path, ImageFormat::default(), (), &world.read_resource());

    loader.load(
        spritesheet_path,
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource(),
    )
}
