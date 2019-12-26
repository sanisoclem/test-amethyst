use amethyst::{
  ecs::*,
  shrev::{EventChannel, ReaderId},
  ui::UiEvent,
};
use log::info;

#[derive(Default)]
pub struct UiEventHandlerSystem {
  reader_id: Option<ReaderId<UiEvent>>,
}
impl<'a> System<'a> for UiEventHandlerSystem {
  type SystemData = Write<'a, EventChannel<UiEvent>>;

  fn run(&mut self, mut events: Self::SystemData) {
    let reader_id = self
      .reader_id
      .get_or_insert_with(|| events.register_reader());

    // Reader id was just initialized above if empty
    for ev in events.read(reader_id) {
      info!("[SYSTEM] You just interacted with a ui element: {:?}", ev);
    }
  }
}
