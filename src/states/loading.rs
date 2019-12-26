use crate::{components::MyScenePrefab, states::menu::MenuState, utils::hierarchy_util};
use amethyst::{
  assets::{PrefabLoader, PrefabLoaderSystem, Processor, ProgressCounter, RonFormat},
  audio::{output::init_output, Source},
  ecs::Entity,
  prelude::*,
  ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
};

pub struct LoadingState {
  scene_root: Option<Entity>,
  ui_root: Option<Entity>,
  loading_progress: Option<ProgressCounter>,
  counter: i32,
}

impl Default for LoadingState {
  fn default() -> Self {
    Self {
      scene_root: None,
      loading_progress: None,
      ui_root: None,
      counter: 0,
    }
  }
}

impl SimpleState for LoadingState {
  fn on_start(&mut self, mut data: StateData<GameData>) {
    let StateData { world, .. } = data;

    // create loader scene
    let handle = world.exec(|loader: PrefabLoader<'_, MyScenePrefab>| {
      loader.load("loader/scene.ron", RonFormat, ())
    });
    self.scene_root = Some(world.create_entity().with(handle).build());

    // create UI
    self.ui_root =
      Some(world.exec(|mut creator: UiCreator<'_>| creator.create("loader/ui.ron", ())));

    // start loading all the things
    init_output(&mut world.res);
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    // delete the loader ui and scene
    if let Some(root) = self.ui_root {
      hierarchy_util::delete_hierarchy(root, data.world).expect("failed to delete loading ui");
    }
    if let Some(root) = self.scene_root {
      hierarchy_util::delete_hierarchy(root, data.world).expect("failed to delete loader scene");
    }

    self.ui_root = None;
    self.scene_root = None;
  }

  fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
    data.data.update(&data.world);

    self.counter += 1;
    if self.counter >= 500 {
      return Trans::Switch(Box::new(MenuState::default()));
    }
    Trans::None
  }
}
