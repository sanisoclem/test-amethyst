use crate::{
  resources::prefabs::{initialize_prefabs, update_prefab_names},
  states::{game::MainGameState, menu::MenuState},
  utils::hierarchy_util,
};
use amethyst::{
  assets::ProgressCounter, audio::output::init_output, ecs::Entity, prelude::*, ui::UiCreator,
};
use log::info;

pub struct LoadingState {
  scene_root: Option<Entity>,
  ui_root: Option<Entity>,
  counter: i32,
  loading_progress: Option<ProgressCounter>,
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
  fn on_start(&mut self, data: StateData<GameData>) {
    let StateData { mut world, .. } = data;

    // create UI
    self.ui_root =
      Some(world.exec(|mut creator: UiCreator<'_>| creator.create("loader/ui.ron", ())));

    // start loading all the things
    init_output(&mut world.res);

    self.loading_progress = Some(initialize_prefabs(&mut world));
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

    if let Some(counter) = &self.loading_progress {
      if counter.is_complete() && self.counter >= 100 {
        info!("counter complete!");
        self.loading_progress = None;
        update_prefab_names(&mut data.world);
        return Trans::Switch(Box::new(MainGameState::default()));
      } else if counter.num_failed() > 0 {
        //info!("some assets failed loading {}", counter.num_failed());
      } else if counter.num_loading() > 0 {
        //info!("still loading {} assets", counter.num_loading());
      }
    }
    Trans::None
  }
}
