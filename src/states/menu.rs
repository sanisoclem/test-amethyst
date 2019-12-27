use crate::{
  resources::prefabs::{PrefabRegistry, UiPrefabRegistry},
  utils::hierarchy_util,
};
use amethyst::{
  assets::{PrefabLoader, PrefabLoaderSystem, Processor, ProgressCounter, RonFormat},
  audio::{output::init_output, Source},
  ecs::Entity,
  prelude::*,
  ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
};

pub struct MenuState {
  scene_root: Option<Entity>,
  ui_root: Option<Entity>,
}

impl Default for MenuState {
  fn default() -> Self {
    Self {
      scene_root: None,
      ui_root: None,
    }
  }
}

impl SimpleState for MenuState {
  fn on_start(&mut self, data: StateData<GameData>) {
    let menu_prefab = data
      .world
      .read_resource::<UiPrefabRegistry>()
      .find(data.world, "main_menu"); // todo: move ids to config file
    if let Some(menu_prefab) = menu_prefab {
      self.ui_root = Some(data.world.create_entity().with(menu_prefab).build());
    }
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    // delete the ui and scene
    if let Some(root) = self.ui_root {
      hierarchy_util::delete_hierarchy(root, data.world).expect("failed to delete loading ui");
    }
    if let Some(root) = self.scene_root {
      hierarchy_util::delete_hierarchy(root, data.world).expect("failed to delete loader scene");
    }

    self.ui_root = None;
    self.scene_root = None;
  }
}
