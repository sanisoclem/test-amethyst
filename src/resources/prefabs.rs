use std::collections::HashMap;
use std::fs::read_dir;

use crate::{
    components::{critter::CritterPrefabData, level::LevelPrefabData, NamedPrefab},
    utils::assets::enumerate_assets,
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
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

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();

    // load ui prefabs
    {
        let mut ui_prefab_registry = UiPrefabRegistry::default();
        ui_prefab_registry.prefabs = enumerate_assets("prefabs/ui")
            .expect("to be able enumerate UI prefabs")
            .map(|(_, asset_path)| {
                world.exec(|loader: UiLoader<'_>| loader.load(asset_path, &mut progress_counter))
            })
            .collect::<Vec<Handle<UiPrefab>>>();
        world.add_resource(ui_prefab_registry);
    }

    // load critter prefabs
    my_load_prefab::<CritterPrefabData>("prefabs/critters", world, &mut progress_counter);

    // load level prefabs
    my_load_prefab::<LevelPrefabData>("prefabs/levels", world, &mut progress_counter);

    progress_counter
}

pub fn update_prefab_names(world: &mut World) {
    my_update_prefab_names::<CritterPrefabData>(world);
    my_update_prefab_names::<LevelPrefabData>(world);
}

fn my_load_prefab<T>(path: &str, world: &mut World, pc: &mut ProgressCounter)
where
    T: for<'a> Deserialize<'a> + Send + Sync + Default + 'static,
{
    let prefab_iter = {
        enumerate_assets(path)
            .expect("assets can be enumerated")
            .map(|(_, asset_path)| {
                world.exec(|loader: PrefabLoader<'_, T>| {
                    (
                        asset_path.to_owned(),
                        loader.load(asset_path, RonFormat, &mut *pc),
                    )
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

        for (_, handle) in prefabs.iter() {
            let name = prefab_resource
                .get_mut(handle)
                .and_then(|prefab| prefab.entity(0))
                .and_then(|entity| entity.data())
                .and_then(|data| data.name())
                .expect("Failed to retrieve prefab name");
            log::info!("Found named prefab {}", name);
            new_prefabs.insert(name.to_owned(), handle.clone());
        }
        new_prefabs
    };
    world
        .write_resource::<PrefabRegistry<T>>()
        .set_prefabs(updated_prefabs);
}
