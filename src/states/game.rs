use crate::{
    components::level::LevelPrefabData, resources::prefabs::PrefabRegistry, utils::hierarchy_util,
};
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, Prefab},
    controls::HideCursor,
    core::Transform,
    ecs::Entity,
    input::{is_key_down, is_mouse_button_down},
    prelude::*,
    renderer::rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
    renderer::{
        mtl::MaterialDefaults,
        palette::LinSrgba,
        rendy::texture::palette::load_from_linear_rgba,
        shape::FromShape,
        types::{Mesh, MeshData},
        Material, Texture,
    },
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
pub struct Hax {
    pub the_material: Option<Handle<Material>>,
}

impl Default for Hax {
    fn default() -> Self {
        Self { the_material: None }
    }
}

impl SimpleState for MainGameState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;
        let scene_handle = world
            .read_resource::<PrefabRegistry<Prefab<LevelPrefabData>>>()
            .get_prefab("default_level")
            .expect("level prefab not found")
            .clone();

        let menu_prefab = world
            .read_resource::<PrefabRegistry<UiPrefab>>()
            .get_prefab("fps_widget") // todo: move ids to config file
            .expect("fps prefab not found")
            .clone();

        self.scene = Some(world.create_entity().with(scene_handle).build());
        self.fps_display = Some(world.create_entity().with(menu_prefab.clone()).build());

        {
            let default_mat = world.read_resource::<MaterialDefaults>().0.clone();

            let mesh = world.exec(|loader: AssetLoaderSystemData<Mesh>| {
                loader.load_from_data(
                    MeshData(
                        MeshBuilder::new()
                            .with_vertices(vec![
                                Position([0.0, 0.0, 0.0]),
                                Position([0.0, 1.0, 0.0]),
                                Position([1.0, 0.0, 0.0]),
                                Position([1.0, 1.0, 0.0]),
                                Position([0.0, 0.0, 1.0]),
                                Position([0.0, 1.0, 1.0]),
                                Position([1.0, 0.0, 1.0]),
                                Position([1.0, 1.0, 1.0]),
                            ])
                            .with_vertices(vec![
                                Normal([0.0, 0.0, 1.0]),
                                Normal([0.0, 0.0, 1.0]),
                                Normal([0.0, 0.0, 1.0]),
                                Normal([0.0, 0.0, 1.0]),
                                Normal([0.0, 0.0, -1.0]),
                                Normal([0.0, 0.0, -1.0]),
                                Normal([0.0, 0.0, -1.0]),
                                Normal([0.0, 0.0, -1.0]),
                            ])
                            .with_vertices(vec![
                                TexCoord([0.0, 0.0]),
                                TexCoord([0.0, 1.0]),
                                TexCoord([1.0, 1.0]),
                                TexCoord([1.0, 1.0]),
                                TexCoord([0.0, 0.0]),
                                TexCoord([0.0, 1.0]),
                                TexCoord([1.0, 1.0]),
                                TexCoord([1.0, 1.0]),
                            ])
                            .with_indices(Indices::U16(
                                vec![
                                    0, 1, 2, // front
                                    1, 3, 2, 1, 5, 3, // top
                                    5, 7, 3, 0, 4, 1, // right
                                    4, 5, 1, 0, 2, 4, // bottom
                                    2, 6, 4, 3, 7, 2, // left
                                    7, 6, 2, 6, 5, 4, 5, 6, 7,
                                ]
                                .into(),
                            )),
                    ),
                    (),
                )
            });

            let albedo = world.exec(|loader: AssetLoaderSystemData<Texture>| {
                loader.load_from_data(
                    load_from_linear_rgba(LinSrgba::new(1.0, 0.0, 0.0, 1.0)).into(),
                    (),
                )
            });

            let mat = world.exec(|loader: AssetLoaderSystemData<Material>| {
                loader.load_from_data(
                    Material {
                        albedo,
                        ..default_mat.clone()
                    },
                    (),
                )
            });
            {
                let mut hax = world.write_resource::<Hax>();
                hax.the_material = Some(mat.clone());
            }

            let transform = Transform::default();

            world
                .create_entity()
                .with(mesh)
                .with(mat)
                .with(transform)
                .build();
        }
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
