use crate::components::Player;
use precompile::bindings::{ActionBinding, AxisBinding, MovementBindings};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct MovePlayer {
    pub base_speed: f32,
}

impl<'s> System<'s> for MovePlayer {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<MovementBindings>>,
    );

    fn run(&mut self, (players, mut transforms, time, input): Self::SystemData) {
        for (_player, player_transform) in (&players, &mut transforms).join() {
            let mut speed = self.base_speed;

            let focus = input.action_is_down(&ActionBinding::Focus).unwrap_or(false);
            if focus {
                speed = self.base_speed / 2.0;
            }

            if let Some(movement) = input.axis_value(&AxisBinding::Vertical) {
                player_transform
                    .set_translation_y(player_transform.translation().y + speed * time.delta_seconds() * movement);
            }
            if let Some(movement) = input.axis_value(&AxisBinding::Horizontal) {
                player_transform
                    .set_translation_x(player_transform.translation().x + speed * time.delta_seconds() * movement);
            }
        }
    }
}
