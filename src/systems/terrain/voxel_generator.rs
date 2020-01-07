use crate::components::terrain::{Chunk, Voxel, VoxelData};
use amethyst::core::math::*;
use amethyst::ecs::prelude::*;
use noise::*;

pub struct VoxelGeneratorSystem {}

impl Default for VoxelGeneratorSystem {
    fn default() -> Self {
        Self {}
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

        let hybrid_multi = HybridMulti::new();
        let billow = Billow::new();
        let basic_multi = BasicMulti::new();
        let ridged_multi = RidgedMulti::new();
        let control = ScalePoint::new(Checkerboard::new()).set_all_scales(2.0, 2.0, 2.0, 2.0);
        let control_big = ScalePoint::new(Checkerboard::new()).set_all_scales(0.5, 0.5, 0.5, 0.5);

        let select1 = Select::new(&hybrid_multi, &basic_multi, &control)
            .set_bounds(0.0, 1.0)
            .set_falloff(0.0);
        let select2 = Select::new(&ridged_multi, &billow, &control)
            .set_bounds(0.0, 1.0)
            .set_falloff(0.0);

        let select = Select::new(&select1, &select2, &control_big)
            .set_bounds(0.0, 1.0)
            .set_falloff(0.0);

        let g = ScaleBias::new(&select).set_scale(0.5).set_bias(0.5);
        let generator = Clamp::new(&g).set_lower_bound(0.0).set_upper_bound(1.);

        let entities_to_modify = (&entities, &chunks, !&voxel_data)
            .join()
            .map(|(entity, chunk, _)| (entity, chunk))
            .collect::<Vec<_>>();

        for (entity, chunk) in entities_to_modify {
            let mut voxels = std::collections::HashMap::new();
            for x in 0..(settings.chunk_size + 1) {
                for z in 0..(settings.chunk_size + 1) {
                    let (abs_x, abs_z) = get_abs((x, z), chunk, &settings);
                    let value = (generator.get([abs_x as f64 / 3000., abs_z as f64 / 3000.]) * 20.)
                        .floor() as i32;

                    for y in 0..51 {
                        let abs_y = y as f32 * settings.voxel_size;

                        voxels.insert(
                            (x, y, z),
                            Voxel {
                                value: if y == 0 || (y < value && y < 50) {
                                    1
                                } else {
                                    0
                                },
                            },
                        );
                    }
                }
            }

            voxel_data
                .insert(entity, VoxelData { voxels: voxels })
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
