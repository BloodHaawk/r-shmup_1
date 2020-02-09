use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    renderer::sprite::prefab::SpriteScenePrefab,
    Error,
};

use serde::{Deserialize, Serialize};

#[derive(Component, Deserialize, Serialize, PrefabData, Debug, Clone)]
#[prefab(Component)]
pub struct Player;

#[derive(Debug, Clone, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct PlayerPrefabData {
    player: Player,
    sprite_scene: SpriteScenePrefab,
}
