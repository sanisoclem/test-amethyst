use crate::components::terrain::{Chunk, Voxel, VoxelData};
use amethyst::core::math::*;
use amethyst::ecs::prelude::*;
use noise::*;

pub struct VoxelGeneratorSystem {
    noise_generator: Perlin,
}

impl Default for VoxelGeneratorSystem {
    fn default() -> Self {
        Self {
            noise_generator: Perlin::new().set_seed(20),
        }
    }
}

impl<'a> System<'a> for VoxelGeneratorSystem {
    type SystemData = (
        Read<'a, super::TerrainSettings>,
        ReadStorage<'a, Chunk>,
        WriteStorage<'a, VoxelData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (settings, chunks, mut voxel_data, entities) = data;

        let entities_to_modify = (&entities, &chunks, !&voxel_data)
            .join()
            .map(|(entity, chunk, _)| (entity, chunk))
            .collect::<Vec<_>>();

        let offset = (settings.chunk_size as f32 * settings.voxel_size) / 2.;

        for (entity, chunk) in entities_to_modify {
            let voxels = (0..(settings.chunk_size * settings.chunk_size))
                .map(|i| {
                    let x = i % settings.chunk_size;
                    let y = i / settings.chunk_size;
                    let abs_x = chunk.x - offset + x as f32;
                    let abs_y = chunk.y - offset + y as f32;
                    let height = self
                        .noise_generator
                        .get([abs_x as f64 * 100., abs_y as f64 * 100.])
                        * 100.;
                    log::info!(
                        "noise {:?}:{}",
                        (abs_x as f64, abs_y as f64),
                        self.noise_generator.get([abs_x as f64, abs_y as f64])
                    );
                    Voxel::new(x, y, abs_x, abs_y, height as f32)
                })
                .collect::<Vec<_>>();

            // log::info!("generated voxel data for chunk {:?} : {:?}", chunk, voxels);

            voxel_data.insert(entity, VoxelData::new(voxels)).unwrap();
        }
    }
}
