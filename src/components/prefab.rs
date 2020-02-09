use crate::components::*;

use amethyst::assets::{Handle, Prefab};

#[derive(Clone)]
pub struct GamePrefabHandles {
    pub camera: Handle<Prefab<CameraPrefabData>>,
    pub player: Handle<Prefab<PlayerPrefabData>>,
}
