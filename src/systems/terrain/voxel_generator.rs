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

        for (entity, chunk) in entities_to_modify {
            let mut adjacent_voxels = Vec::new();
            let mut voxels = Vec::new();
            for x in -2..(settings.chunk_size + 1) {
                for z in -2..(settings.chunk_size + 1) {
                    let (abs_x, abs_z) = get_abs((x, z), chunk, &settings);
                    let height = self.get_height(abs_x, abs_z);
                    // todo: get height for adjacent voxels from adjacent chunks if exists (in case they have been modified)

                    let vector_to_push_to =
                        if z < 0 || z >= settings.chunk_size || x < 0 || x >= settings.chunk_size {
                            &mut adjacent_voxels
                        } else {
                            &mut voxels
                        };

                    for y in 0..height {
                        let abs_y = y as f32 * settings.voxel_size;
                        vector_to_push_to.push(Voxel::new(x, y, z, abs_x, abs_y, abs_z))
                    }
                }
            }

            voxel_data
                .insert(entity, VoxelData::new(voxels, adjacent_voxels))
                .unwrap();
        }
    }
}

fn get_abs((x, y): (i32, i32), chunk: &Chunk, settings: &super::TerrainSettings) -> (f32, f32) {
    let offset = (settings.chunk_size as f32 * settings.voxel_size) / 2.;
    (
        chunk.x - offset + (x as f32 * settings.voxel_size),
        chunk.y - offset + (y as f32 * settings.voxel_size),
    )
}

impl VoxelGeneratorSystem {
    fn get_height(&self, x: f32, y: f32) -> i32 {
        (self.noise_generator.get([x as f64 / 100., y as f64 / 100.]) * 100.).floor() as i32
    }
}
