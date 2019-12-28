use crate::{resources::prefabs::PrefabRegistry, utils::hierarchy_util};
use amethyst::{ecs::Entity, prelude::*, ui::UiPrefab};

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
      .read_resource::<PrefabRegistry<UiPrefab>>()
      .get_prefab("main_menu") // todo: move ids to config file
      .expect("main menu prefab not found")
      .clone();

    self.ui_root = Some(data.world.create_entity().with(menu_prefab.clone()).build());
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
