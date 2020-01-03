use crate::components::terrain::{Chunk, VoxelData};

use amethyst::{
    assets::{AssetLoaderSystemData, Handle},
    core::math::*,
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::{
        palette::LinSrgba, rendy::texture::palette::load_from_linear_rgba, types::Mesh,
        visibility::BoundingSphere, Material, Texture,
    },
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
        AssetLoaderSystemData<'a, Texture>,
        AssetLoaderSystemData<'a, Material>,
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
            tex_loader,
            mat_loader,
            mut meshes,
            mut materials,
            mut transforms,
            mut bounds,
            hax,
        ) = data;

        if let Some(material) = hax.blah.as_ref() {
            let to_create = (&*entities, &chunks, &voxel_data, !&transforms)
                .join()
                .map(|(entity, chunk, voxel, _)| (entity, chunk, voxel))
                .collect::<Vec<_>>();

            let chunk_size = settings.chunk_size as f32 * settings.voxel_size;
            let offset = chunk_size / 2.;

            for (entity, chunk, voxel) in to_create.into_iter() {
                log::info!("Creating mesh for {:?}", chunk);
                let origin = Vector3::new(chunk.x, -50., chunk.y);

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

                // let albedo = tex_loader.load_from_data(
                //     load_from_linear_rgba(LinSrgba::new(
                //         0.2 + origin.x / 10000.,
                //         (origin.x + origin.z) / 20000.,
                //         (origin.z) / 10000.,
                //         1.0,
                //     ))
                //     .into(),
                //     (),
                // );

                // let mat = mat_loader.load_from_data(
                //     Material {
                //         albedo,
                //         ..material.clone()
                //     },
                //     (),
                // );

                materials
                    .insert(entity, material.clone())
                    .expect("material insertion failed");

                bounds
                    .insert(
                        entity,
                        BoundingSphere::origin(((chunk_size * chunk_size) * 2.).sqrt()), // 2.),
                    )
                    .expect("bounding sphere insert failed");
            }
        }
    }
}
