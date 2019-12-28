use crate::{
    components::{critter::CritterPrefabData, level::LevelPrefabData, NamedPrefab},
    utils::assets::enumerate_assets,
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::{World, WorldExt},
    ui::{UiLoader, UiPrefab},
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Default)]
pub struct PrefabRegistry<T> {
    prefabs: HashMap<String, Handle<T>>,
}

impl<T> PrefabRegistry<T> {
    pub fn insert(&mut self, name: String, prefab_handle: Handle<T>) {
        self.prefabs.insert(name, prefab_handle);
    }

    pub fn get_prefab(&self, name: &str) -> Option<&Handle<T>> {
        self.prefabs.get(name)
    }

    pub fn get_prefabs(&self) -> &HashMap<String, Handle<T>> {
        &self.prefabs
    }

    pub fn set_prefabs(&mut self, prefabs: HashMap<String, Handle<T>>) {
        self.prefabs = prefabs;
    }
}

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut progress_counter = ProgressCounter::new();

    // load ui prefabs
    {
        let mut registry = PrefabRegistry::<UiPrefab>::default();
        registry.prefabs = enumerate_assets("prefabs/ui")
            .expect("to be able enumerate UI prefabs")
            .map(|(_, asset_path)| {
                (
                    asset_path.to_owned(),
                    world.exec(|loader: UiLoader<'_>| {
                        loader.load(asset_path, &mut progress_counter)
                    }),
                )
            })
            .collect::<HashMap<String, Handle<UiPrefab>>>();
        world.insert(registry);
    }

    // load critter prefabs
    load_prefabs::<CritterPrefabData>("prefabs/critters", world, &mut progress_counter);

    // load level prefabs
    load_prefabs::<LevelPrefabData>("prefabs/levels", world, &mut progress_counter);

    progress_counter
}

pub fn update_prefab_names(world: &mut World) {
    {
        let updated_prefabs = {
            let registry = world.read_resource::<PrefabRegistry<UiPrefab>>();
            let prefabs = registry.get_prefabs();
            let mut prefab_resource = world.write_resource::<AssetStorage<UiPrefab>>();
            let mut new_prefabs = HashMap::new();

            for (_, handle) in prefabs.iter() {
                let name = prefab_resource
                    .get_mut(handle)
                    .and_then(|prefab| prefab.entity(0))
                    .and_then(|entity| entity.data())
                    .and_then(|data| data.0.as_ref())
                    .map(|transform| &transform.id)
                    .expect("Failed to retrieve prefab name");
                new_prefabs.insert(name.to_owned(), handle.clone());
            }
            new_prefabs
        };
        world
            .write_resource::<PrefabRegistry<UiPrefab>>()
            .set_prefabs(updated_prefabs);
    }

    my_update_prefab_names::<CritterPrefabData>(world);
    my_update_prefab_names::<LevelPrefabData>(world);
}

fn load_prefabs<T>(path: &str, world: &mut World, pc: &mut ProgressCounter)
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

    let mut registry = PrefabRegistry::<Prefab<T>>::default();
    for (_count, (name, prefab)) in prefab_iter.enumerate() {
        registry.insert(name, prefab);
    }
    world.insert(registry);
}

fn my_update_prefab_names<T>(world: &mut World)
where
    T: Send + Sync + NamedPrefab + 'static,
{
    let updated_prefabs = {
        let creature_prefabs = world.read_resource::<PrefabRegistry<Prefab<T>>>();
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
            new_prefabs.insert(name.to_owned(), handle.clone());
        }
        new_prefabs
    };
    world
        .write_resource::<PrefabRegistry<Prefab<T>>>()
        .set_prefabs(updated_prefabs);
}
