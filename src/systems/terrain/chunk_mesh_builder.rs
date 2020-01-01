use crate::components::terrain::{Chunk, VoxelData};

use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::math::*,
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::{types::Mesh, visibility::BoundingSphere, Material},
};

// generates meshes for chunks
#[derive(Default)]
pub struct ChunkMeshBuilderSystem;

impl<'a> System<'a> for ChunkMeshBuilderSystem {
    type SystemData = (
        Read<'a, super::TerrainSettings>,
        Entities<'a>,
        ReadStorage<'a, Chunk>,
        ReadStorage<'a, VoxelData>, // convert to read id
        AssetLoaderSystemData<'a, Mesh>,
        WriteStorage<'a, Handle<Mesh>>,
        WriteStorage<'a, Handle<Material>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, BoundingSphere>,
        Read<'a, crate::states::game::Hax>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            settings,
            entities,
            chunks,
            voxel_data,
            mesh_loader,
            mut meshes,
            mut materials,
            mut transforms,
            mut bounds,
            hax,
        ) = data;

        if let Some(material) = hax.the_material.as_ref() {
            let to_create = (&*entities, &chunks, &voxel_data, !&transforms)
                .join()
                .map(|(entity, chunk, voxel, _)| (entity, chunk, voxel))
                .collect::<Vec<_>>();

            let chunk_size = settings.chunk_size as f32 * settings.voxel_size;
            let offset = chunk_size / 2.;

            for (entity, chunk, voxel) in to_create.into_iter() {
                log::info!("Creating mesh for {:?}", chunk);
                let origin = Vector3::new(chunk.x, 0., chunk.y);

                transforms
                    .insert(entity, Transform::from(origin))
                    .expect("transform insert failed");

                // todo: generate mesh based on LOD
                meshes
                    .insert(
                        entity,
                        mesh_loader.load_from_data(
                            crate::utils::mesh::create_voxel_mesh2(
                                voxel,
                                settings.chunk_size,
                                settings.voxel_size,
                                offset,
                                chunk,
                            ),
                            (),
                        ),
                    )
                    .expect("mesh insert failed");
                materials
                    .insert(entity, material.clone())
                    .expect("material insertion failed");

                bounds
                    .insert(
                        entity,
                        BoundingSphere::origin(((chunk_size * chunk_size) * 2.).sqrt() / 2.),
                    )
                    .expect("bounding sphere insert failed");
            }
        }
    }
}
