use amethyst::{
  assets::{PrefabData, ProgressCounter},
  derive::PrefabData,
  ecs::{Component, DenseVecStorage, Entity, WriteStorage},
  renderer::rendy::mesh::{Normal, Position, TexCoord},
  utils::scene::BasicScenePrefab,
  Error,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MyScenePrefab {
  basic: Option<BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>>,
  rotate_me: Option<RotateMe>,
}

impl Default for MyScenePrefab {
  fn default() -> Self {
    MyScenePrefab {
      basic: Some(BasicScenePrefab::<(
        Vec<Position>,
        Vec<Normal>,
        Vec<TexCoord>,
      )>::default()),
      rotate_me: None,
    }
  }
}

#[derive(Clone, Deserialize, Serialize, Debug, PrefabData)]
#[prefab(Component)]
pub struct RotateMe {
  pub angular_velocity: f32,
}

impl Component for RotateMe {
  type Storage = DenseVecStorage<Self>;
}
