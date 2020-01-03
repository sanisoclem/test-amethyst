use amethyst::{
    core::{ecs::prelude::*, SystemBundle},
    Error,
};

mod chunk_lod;
mod chunk_mesh_builder;
mod chunk_spawner;
mod voxel_generator;

pub use chunk_lod::ChunkLodSystem;
pub use chunk_mesh_builder::ChunkMeshBuilderSystem;
pub use chunk_spawner::ChunkSpawnerSystem;
pub use voxel_generator::VoxelGeneratorSystem;

pub struct TerrainSettings {
    pub chunk_size: i32, // voxels per side
    pub voxel_size: f32, // length of voxel side
}

impl Default for TerrainSettings {
    fn default() -> Self {
        Self {
            chunk_size: 50,
            voxel_size: 5.,
        }
    }
}

#[derive(Default, Debug)]
pub struct TerrainBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TerrainBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.insert(TerrainSettings::default());
        builder.add(ChunkSpawnerSystem::default(), "terrain_chunk_spawner", &[]);
        builder.add(ChunkLodSystem, "terrain_lod", &["terrain_chunk_spawner"]);
        builder.add(
            VoxelGeneratorSystem::default(),
            "terrain_voxel_generator",
            &["terrain_chunk_spawner"],
        );
        builder.add(
            ChunkMeshBuilderSystem::default(),
            "terrain_mesh_builder",
            &["terrain_voxel_generator", "terrain_lod"],
        );

        Ok(())
    }
}
