use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Chunk {
    pub x: f32,
    pub y: f32,
}
impl Component for Chunk {
    type Storage = DenseVecStorage<Self>;
}

impl Chunk {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug)]
pub struct Voxel {
    pub value: i32,
}

#[derive(Debug)]
pub struct VoxelData {
    pub voxels: std::collections::HashMap<(i32, i32, i32), Voxel>,
}

impl Component for VoxelData {
    type Storage = DenseVecStorage<Self>;
}
