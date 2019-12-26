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

mod components;
mod states;
mod systems;
mod utils;

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
        .with(
            systems::ui_event_handler::UiEventHandlerSystem::default(),
            "ui_event_handler",
            &[],
        )
        .with(systems::rotator::RotatorSystem, "rotator_system", &[])
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

    let mut game = Application::build(assets_dir, states::loading::LoadingState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Unlimited, 9999)
        .build(game_data)?;

    game.run();
    Ok(())
}
