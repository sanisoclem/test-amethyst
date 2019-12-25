use amethyst::{
    assets::{PrefabLoader,PrefabLoaderSystem,RonFormat},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir,scene::BasicScenePrefab},
};

type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let handle = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load("sphere.ron", RonFormat, ())
        });
        data.world.create_entity().with(handle).build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");

    let display_config_path = config_dir.join("display.ron");


    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(),"",&[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderShaded3D::default()),
        )?
        ;

    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}
