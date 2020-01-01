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

        // TODO: optimize (duplicate calcs)
        for (entity, chunk) in entities_to_modify {
            let voxels = (0..(settings.chunk_size * settings.chunk_size))
                .map(|i| {
                    let x = i % settings.chunk_size;
                    let y = i / settings.chunk_size;
                    let (abs_x, abs_y) = get_abs((x, y), chunk, &settings);
                    let heights = [
                        (x, y),
                        (x + 1, y),
                        (x - 1, y),
                        (x, y + 1),
                        (x, y - 1),
                        (x + 1, y + 1),
                        (x - 1, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y + 1),
                    ]
                    .iter()
                    .map(|&xy| get_abs(xy, chunk, &settings))
                    .map(|(abs_x, abs_y)| self.get_height(abs_x, abs_y))
                    .collect();

                    Voxel::new(x, y, abs_x, abs_y, heights)
                })
                .collect::<Vec<_>>();

            voxel_data.insert(entity, VoxelData::new(voxels)).unwrap();
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
    fn get_height(&self, x: f32, y: f32) -> f32 {
        (self.noise_generator.get([x as f64 / 100., y as f64 / 100.]) * 30.) as f32
    }
}
