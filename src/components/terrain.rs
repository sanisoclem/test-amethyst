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
    pub x: i32,
    pub y: i32,
    pub abs_x: f32,
    pub abs_y: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct VoxelData {
    pub voxels: Vec<Voxel>, // when const generics is stable, probably can be `voxels: [T; N]` (need more than 32 voxels maybe)
}

impl Component for VoxelData {
    type Storage = DenseVecStorage<Self>;
}
impl Voxel {
    pub fn new(x: i32, y: i32, abs_x: f32, abs_y: f32, height: f32) -> Self {
        Self {
            x: x,
            y: y,
            abs_x: abs_x,
            abs_y: abs_y,
            height: height,
        }
    }
}

impl VoxelData {
    pub fn new(data: Vec<Voxel>) -> Self {
        Self { voxels: data }
    }
}
