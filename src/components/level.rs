use amethyst::{
    assets::{PrefabData, ProgressCounter},
    controls::ControlTagPrefab,
    core::{Named, Transform},
    derive::PrefabData,
    ecs::Entity,
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    renderer::{
        camera::CameraPrefab, formats::GraphicsPrefab, light::LightPrefab,
        rendy::mesh::MeshBuilder, shape::FromShape,
    },
    utils::removal::Removal,
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, PrefabData)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct LevelPrefabData<V = (Vec<Position>, Vec<Normal>, Vec<TexCoord>)>
where
    V: FromShape + Into<MeshBuilder<'static>>,
{
    pub name: Option<Named>,
    graphics: Option<GraphicsPrefab<V>>,
    transform: Option<Transform>,
    light: Option<LightPrefab>,
    camera: Option<CameraPrefab>,
    control_tag: Option<ControlTagPrefab>,
    removal: Option<Removal<()>>,
}

impl<T> crate::components::NamedPrefab for LevelPrefabData<T>
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
