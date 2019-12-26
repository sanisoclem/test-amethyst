use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystem, Processor, RonFormat},
    audio::{output::init_output, Source},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, Time},
    ecs::prelude::{Entity, System, Write},
    input::{is_close_requested, is_key_down, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    ui::{RenderUi, UiBundle, UiCreator, UiEvent, UiFinder, UiText},
    utils::{
        application_root_dir,
        fps_counter::{FpsCounter, FpsCounterBundle},
    },
    winit::VirtualKeyCode,
};
use log::info;

mod components;
mod systems;

#[derive(Default)]
struct GameState {
    fps_display: Option<Entity>,
}

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // Create sample scene
        let handle = world.exec(|loader: PrefabLoader<'_, components::MyScenePrefab>| {
            loader.load("scene.ron", RonFormat, ())
        });
        world.create_entity().with(handle).build();

        // initialize audio?? (Should this be in game state start?)
        init_output(&mut world.res);

        // create UI
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(e) => {
                if is_close_requested(&e) || is_key_down(&e, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                info!(
                    "[HANDLE_EVENT] You just interacted with a ui element: {:?}",
                    ui_event
                );
                Trans::None
            }
            StateEvent::Input(input) => {
                info!("Input Event detected: {:?}.", input);
                Trans::None
            }
        }
    }
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = state_data;

        if self.fps_display.is_none() {
            world.exec(|finder: UiFinder<'_>| {
                if let Some(entity) = finder.find("fps") {
                    self.fps_display = Some(entity);
                }
            });
        }

        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(fps_display) = self.fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 {
                    let fps = world.read_resource::<FpsCounter>().sampled_fps();
                    fps_display.text = format!("{:.*} FPS", 2, fps);
                }
            }
        }

        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");

    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<components::MyScenePrefab>::default(),
            "",
            &[],
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(UiEventHandlerSystem::new(), "ui_event_handler", &[])
        .with(systems::RotatorSystem, "rotator_system", &[])
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, GameState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();
    Ok(())
}

/// This shows how to handle UI events.
#[derive(Default)]
pub struct UiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl UiEventHandlerSystem {
    pub fn new() -> Self {
        Self::default()
    }
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
