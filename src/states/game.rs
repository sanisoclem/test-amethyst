use crate::{
  components::level::LevelPrefabData, resources::prefabs::PrefabRegistry, utils::hierarchy_util,
};
use amethyst::{
  assets::Prefab,
  controls::HideCursor,
  ecs::Entity,
  input::{is_key_down, is_mouse_button_down},
  prelude::*,
  ui::UiPrefab,
  winit::{MouseButton, VirtualKeyCode},
};

pub struct MainGameState {
  scene: Option<Entity>,
  fps_display: Option<Entity>,
}

impl Default for MainGameState {
  fn default() -> Self {
    Self {
      scene: None,
      fps_display: None,
    }
  }
}

impl SimpleState for MainGameState {
  fn on_start(&mut self, data: StateData<GameData>) {
    let scene_handle = data
      .world
      .read_resource::<PrefabRegistry<Prefab<LevelPrefabData>>>()
      .get_prefab("default_level")
      .expect("level prefab not found")
      .clone();

    let menu_prefab = data
      .world
      .read_resource::<PrefabRegistry<UiPrefab>>()
      .get_prefab("fps_widget") // todo: move ids to config file
      .expect("fps prefab not found")
      .clone();

    self.scene = Some(data.world.create_entity().with(scene_handle).build());
    self.fps_display = Some(data.world.create_entity().with(menu_prefab.clone()).build());
  }

  fn on_stop(&mut self, data: StateData<GameData>) {
    // delete the ui and scene
    if let Some(root) = self.scene {
      hierarchy_util::delete_hierarchy(root, data.world).expect("failed to delete scene");
    }

    self.scene = None;
  }

  fn handle_event(
    &mut self,
    data: StateData<'_, GameData<'_, '_>>,
    event: StateEvent,
  ) -> SimpleTrans {
    let StateData { world, .. } = data;
    if let StateEvent::Window(event) = &event {
      if is_key_down(&event, VirtualKeyCode::Escape) {
        let mut hide_cursor = world.write_resource::<HideCursor>();
        hide_cursor.hide = false;
      } else if is_mouse_button_down(&event, MouseButton::Left) {
        let mut hide_cursor = world.write_resource::<HideCursor>();
        hide_cursor.hide = true;
      }
    }
    Trans::None
  }
}
