use std::collections::HashMap;
use std::fs::read_dir;

use crate::components::{critter::CritterPrefabData, level::LevelPrefabData, NamedPrefab};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
};
use serde::Deserialize;

// evoli's UI registry (should we use paths as names instead?)
#[derive(Default)]
pub struct UiPrefabRegistry {
    pub prefabs: Vec<Handle<UiPrefab>>,
}

impl UiPrefabRegistry {
    pub fn find(&self, world: &World, name: &str) -> Option<Handle<UiPrefab>> {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs.iter().find_map(|handle| {
            if storage
                .get(handle)?
                .entities()
                .next()?
                .data()?
                .0 // transform is 0th element of UiPrefab tuple
                .as_ref()?
                .id
                == name
            {
                Some(handle.clone())
            } else {
                None
            }
        })
    }
}

#[derive(Default)]
pub struct PrefabRegistry<T>
where
    T: Send + Sync,
{
    prefabs: HashMap<String, Handle<Prefab<T>>>,
}

impl<T> PrefabRegistry<T>
where
    T: Send + Sync,
{
    pub fn insert(&mut self, name: String, prefab_handle: Handle<Prefab<T>>) {
        self.prefabs.insert(name, prefab_handle);
    }

    pub fn get_prefab(&self, name: &str) -> Option<&Handle<Prefab<T>>> {
        self.prefabs.get(name)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<Prefab<T>>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<Prefab<T>>>) {
        self.prefabs = prefabs;
    }
}

fn make_name(subdirectory: &str, entry: &std::fs::DirEntry) -> Option<String> {
    let path_buffer = entry.path();
    let filename = path_buffer.file_name()?;
    Some(format!("{}{}", subdirectory, filename.to_str()?))
}

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();

    // load ui prefabs
    {
        let mut ui_prefab_registry = UiPrefabRegistry::default();
        let prefab_dir_path = application_root_dir()
            .expect("to have an app root dir")
            .join("assets/prefabs/ui");
        let prefab_iter = read_dir(prefab_dir_path).expect("to be able enumerate UI prefabs");
        ui_prefab_registry.prefabs = prefab_iter
            .map(|prefab_dir_entry| {
                world.exec(|loader: UiLoader<'_>| {
                    loader.load(
                        make_name(
                            "prefabs/ui/",
                            &prefab_dir_entry.expect("to read prefab path"),
                        )
                        .expect("to be able to name UI prefabs"),
                        &mut progress_counter,
                    )
                })
            })
            .collect::<Vec<Handle<UiPrefab>>>();
        world.add_resource(ui_prefab_registry);
    }

    // load critter prefabs
    my_load_prefab::<CritterPrefabData<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>>(
        "critters",
        world,
        &mut progress_counter,
    );

    // load level prefabs
    my_load_prefab::<LevelPrefabData<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>>(
        "levels",
        world,
        &mut progress_counter,
    );

    progress_counter
}

pub fn update_prefab_names(world: &mut World) {
    my_update_prefab_names::<CritterPrefabData<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>>(world);
}

fn my_load_prefab<T>(path: &str, world: &mut World, pc: &mut ProgressCounter)
where
    T: for<'a> Deserialize<'a> + Send + Sync + Default + 'static,
{
    let prefab_iter = {
        let prefab_dir_path = application_root_dir()
            .expect("to have an app root dir")
            .join("assets/prefabs")
            .join(path);
        let prefab_iter = read_dir(prefab_dir_path).expect("to enumerate prefab files");
        prefab_iter.map(|prefab_dir_entry| {
            world.exec(|loader: PrefabLoader<'_, T>| {
                let name = make_name(path, &prefab_dir_entry.expect("to read prefab file path"))
                    .expect("to generate prefab name");
                (name.to_owned(), loader.load(name, RonFormat, &mut *pc))
            })
        })
    };

    let mut registry = PrefabRegistry::<T>::default();
    for (_count, (name, prefab)) in prefab_iter.enumerate() {
        registry.insert(name, prefab);
    }
    world.add_resource(registry);
}

fn my_update_prefab_names<T>(world: &mut World)
where
    T: Send + Sync + NamedPrefab + 'static,
{
    let updated_prefabs = {
        let creature_prefabs = world.read_resource::<PrefabRegistry<T>>();
        let prefabs = creature_prefabs.get_prefabs();
        let mut prefab_resource = world.write_resource::<AssetStorage<Prefab<T>>>();
        let mut new_prefabs = HashMap::new();
        let g: Option<i32>;

        for (_key, handle) in prefabs.iter() {
            if let Some(name) = prefab_resource
                .get_mut(handle)
                .and_then(|prefab| prefab.entity(0))
                .and_then(|entity| entity.data())
                .and_then(|data| data.name())
            {
                new_prefabs.insert(name.to_owned(), handle.clone());
            }
        }
        new_prefabs
    };
    world
        .write_resource::<PrefabRegistry<T>>()
        .set_prefabs(updated_prefabs);
}
