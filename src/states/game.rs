use crate::{
  components::level::LevelPrefabData, resources::prefabs::PrefabRegistry, utils::hierarchy_util,
};
use amethyst::{
  assets::{PrefabLoader, PrefabLoaderSystem, Processor, ProgressCounter, RonFormat},
  audio::{output::init_output, Source},
  ecs::Entity,
  prelude::*,
  ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
};

pub struct MainGameState {
  level: Option<Entity>,
}

impl Default for MainGameState {
  fn default() -> Self {
    Self { level: None }
  }
}

impl SimpleState for MainGameState {
  fn on_start(&mut self, data: StateData<GameData>) {
    let level = {
      let registry = data
        .world
        .read_resource::<PrefabRegistry<LevelPrefabData>>();
      registry
        .get_prefab("default_level")
        .expect("level prefab not found")
        .clone()
    };

    self.level = Some(data.world.create_entity().with(level).build());
  }
}
