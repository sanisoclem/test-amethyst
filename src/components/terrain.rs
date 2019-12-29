use amethyst::ecs::{Component, DenseVecStorage};
pub struct TerrainChunk;
impl Component for TerrainChunk {
    type Storage = DenseVecStorage<Self>;
}

pub struct TerrainChunkActive;
impl Component for TerrainChunkActive {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Biome {
    pub x: i32,
    pub y: i32,
}
impl Component for Biome {
    type Storage = DenseVecStorage<Self>;
}

impl Biome {
    pub fn new((x, y): (i32, i32)) -> Self {
        Self { x: x, y: y }
    }
}
