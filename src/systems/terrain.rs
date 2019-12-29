use crate::components::Biome;

use amethyst::{
    assets::{AssetLoaderSystemData, AssetStorage, Handle},
    controls::FlyControlTag,
    core::math::{Translation3, Vector3},
    core::{SystemDesc, Time, Transform},
    derive::SystemDesc,
    ecs::prelude::{Entity, Read, ReadStorage, System, SystemData, World, Write, WriteStorage},
    ecs::{Entities, Join},
    renderer::rendy::mesh::{Indices, MeshBuilder, Normal, Position, TexCoord},
    renderer::{
        mtl::MaterialDefaults,
        palette::LinSrgba,
        rendy::texture::palette::load_from_linear_rgba,
        types::{Mesh, MeshData},
        Material, Texture,
    },
};

#[derive(SystemDesc)]
pub struct BiomeSystem {
    #[system_desc(skip)]
    registry: std::collections::HashMap<(i32, i32), Entity>,
    #[system_desc(skip)]
    size: f32,
}

impl Default for BiomeSystem {
    fn default() -> Self {
        BiomeSystem {
            size: 30.,
            registry: std::collections::HashMap::new(),
        }
    }
}

impl<'a> System<'a> for BiomeSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FlyControlTag>,
        WriteStorage<'a, Biome>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (transform, control_tag, mut biomes, entities) = data;

        // find position of current camera (if no camera is found, then do nothing)
        if let Some((position, _)) = (&transform, &control_tag).join().next() {
            // find which biome the camera is at
            let translation = position.translation();
            let x = (translation.x / self.size).floor() as i32;
            let y = (translation.z / self.size).floor() as i32;

            // -- determine which biomes to create
            let biomes_to_create = vec![
                (x, y),
                (x + 1, y),
                (x - 1, y),
                (x, y + 1),
                (x, y - 1),
                (x + 1, y + 1),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
            ];

            //create the biomes
            for key in biomes_to_create.into_iter() {
                if let None = self.registry.get(&key) {
                    log::info!("Created biome {:?}", key);
                    let entity = entities
                        .build_entity()
                        .with(Biome::new(key), &mut biomes)
                        .build();
                    self.registry.insert(key, entity);
                }
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct BiomeMeshBuilderSystem {
    #[system_desc(skip)]
    size: f32,
}

impl Default for BiomeMeshBuilderSystem {
    fn default() -> Self {
        BiomeMeshBuilderSystem { size: 30. }
    }
}

impl<'a> System<'a> for BiomeMeshBuilderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Biome>,
        AssetLoaderSystemData<'a, Mesh>,
        WriteStorage<'a, Handle<Mesh>>,
        WriteStorage<'a, Handle<Material>>,
        WriteStorage<'a, Transform>,
        Read<'a, crate::states::game::Hax>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, biomes, mesh_loader, mut meshes, mut materials, mut transforms, hax) = data;
        let mut transforms_to_insert = vec![];

        if let Some(material) = hax.the_material.as_ref() {
            for (entity, biome, _) in (&*entities, &biomes, !&transforms).join() {
                log::info!("Creating mesh for {:?}", biome);
                transforms_to_insert.push((
                    entity,
                    Transform::from(Vector3::new(
                        biome.x as f32 * self.size,
                        (biome.x + biome.y) as f32 - 50.,
                        biome.y as f32 * self.size,
                    )),
                ));

                meshes
                    .insert(
                        entity,
                        mesh_loader
                            .load_from_data(crate::utils::mesh::create_cube_mesh(self.size), ()),
                    )
                    .expect("mesh insert failed");
                materials
                    .insert(entity, material.clone())
                    .expect("material insertion failed");
            }

            for (entity, to_insert) in transforms_to_insert.into_iter() {
                transforms
                    .insert(entity, to_insert)
                    .expect("transform insert failed");
            }
        }
    }
}

// #[derive(SystemDesc)]
// pub struct TerrainSystem {
//     #[system_desc(skip)]
//     plate_size: f32,
// }

// impl Default for TerrainSystem {
//     fn default() -> Self {
//         TerrainSystem { plate_size: 50. }
//     }
// }

// impl<'a> System<'a> for TerrainSystem {
//     type SystemData = (
//         Read<'a, Time>,
//         ReadStorage<'a, Transform>,
//         ReadStorage<'a, FlyControlTag>,
//         WriteStorage<'a, TectonicPlate>,
//     );

//     fn run(&mut self, data: Self::SystemData) {
//         let (time, transform, controlTag, mut plates) = data;

//         // find position of current camera (if no camera is found, then do nothing)
//         if let Some((position, _)) = (&transform, &controlTag).join().next() {
//             // determine which plate it is in
//         }
//     }
// }
