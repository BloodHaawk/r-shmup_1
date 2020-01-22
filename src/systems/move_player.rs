use crate::components::Player;
use precompile::bindings::{AxisBinding, MovementBindings};

use amethyst::{
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct MovePlayer {
    pub speed: f32,
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
            if let Some(movement) = input.axis_value(&AxisBinding::Vertical) {
                player_transform
                    .set_translation_y(player_transform.translation().y + self.speed * time.delta_seconds() * movement);
            }
            if let Some(movement) = input.axis_value(&AxisBinding::Horizontal) {
                player_transform
                    .set_translation_x(player_transform.translation().x + self.speed * time.delta_seconds() * movement);
            }
        }
    }
}
