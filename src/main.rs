use amethyst::{
    assets::{PrefabLoaderSystemDesc, Processor},
    audio::Source,
    controls::FlyControlBundle,
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::{application_root_dir, auto_fov::AutoFovSystem, fps_counter::FpsCounterBundle},
};
use components::{critter::CritterPrefabData, level::LevelPrefabData};

mod bindings;
mod components;
mod resources;
mod states;
mod systems;
mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");

    let display_config_path = config_dir.join("display.ron");
    let input_bindings_path = config_dir.join("input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<CritterPrefabData>::default(),
            "",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<LevelPrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(
            FlyControlBundle::<bindings::GameBindings>::new(
                Some(bindings::AxisBinding::XAxis),
                Some(bindings::AxisBinding::YAxis),
                Some(bindings::AxisBinding::ZAxis),
            )
            .with_sensitivity(0.1, 0.1)
            .with_speed(250.),
        )?
        .with(AutoFovSystem::new(), "auto_fov", &[])
        .with_bundle(TransformBundle::new().with_dep(&["fly_movement", "free_rotation"]))?
        .with_bundle(UiBundle::<bindings::GameBindings>::new())?
        .with(Processor::<Source>::new(), "source_processor", &[])
        .with(
            systems::ui_event_handler::UiEventHandlerSystem::default(),
            "ui_event_handler",
            &[],
        )
        .with(systems::debug::DebugSystem::default(), "debug_system", &[])
        .with_bundle(systems::terrain::TerrainBundle::default())?
        .with_bundle(FpsCounterBundle::default())?
        .with_bundle(
            InputBundle::<bindings::GameBindings>::new()
                .with_bindings_from_file(&input_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderUi::default()),
        )?;

    let mut game = Application::build(assets_dir, states::loading::LoadingState::default())?
        .build(game_data)?;

    game.run();
    Ok(())
}
