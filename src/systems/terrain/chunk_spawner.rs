use crate::components::terrain::Chunk;

use amethyst::{
    controls::FlyControlTag,
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
};

// controls chunk lifetime
#[derive(SystemDesc)]
pub struct ChunkSpawnerSystem {
    #[system_desc(skip)]
    registry: std::collections::HashMap<(i32, i32), Entity>, // move to resource ?
}

impl Default for ChunkSpawnerSystem {
    fn default() -> Self {
        ChunkSpawnerSystem {
            registry: std::collections::HashMap::new(),
        }
    }
}

impl<'a> System<'a> for ChunkSpawnerSystem {
    type SystemData = (
        Read<'a, super::TerrainSettings>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, FlyControlTag>,
        WriteStorage<'a, Chunk>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (settings, transform, control_tag, mut chunks, entities) = data;

        // find position of current camera (if no camera is found, then do nothing)
        if let Some((position, _)) = (&transform, &control_tag).join().next() {
            // find which chunk the camera is at
            let translation = position.translation();

            let chunk_size = settings.chunk_size as f32 * settings.voxel_size;
            let half_chunk_size = chunk_size / 2.;
            let x = ((translation.x + half_chunk_size) / chunk_size).floor() as i32;
            let y = ((translation.z + half_chunk_size) / chunk_size).floor() as i32;

            // -- determine which chunks to create
            let chunks_to_create = vec![
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

            //create the chunks
            for (x, y) in chunks_to_create.into_iter() {
                if let None = self.registry.get(&(x, y)) {
                    log::info!("Created chunk {:?}", (x, y));
                    let entity = entities
                        .build_entity()
                        // todo: add lod component
                        .with(
                            Chunk::new(x as f32 * chunk_size, y as f32 * chunk_size),
                            &mut chunks,
                        )
                        .build();
                    self.registry.insert((x, y), entity);
                }
            }
        }
    }
}
