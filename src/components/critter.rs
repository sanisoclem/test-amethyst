use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::{Named, Transform},
    derive::PrefabData,
    ecs::Entity,
    renderer::{formats::GraphicsPrefab, rendy::mesh::MeshBuilder, shape::FromShape},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CritterPrefabData<V>
where
    V: FromShape + Into<MeshBuilder<'static>>,
{
    pub name: Option<Named>,
    graphics: Option<GraphicsPrefab<V>>,
    transform: Option<Transform>,
}

impl<T> crate::components::NamedPrefab for CritterPrefabData<T>
where
    T: FromShape + Into<MeshBuilder<'static>>,
{
    fn name(&self) -> Option<&str> {
        if let Some(name) = &self.name {
            return Some(&name.name);
        }
        None
    }
}
