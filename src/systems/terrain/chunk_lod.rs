use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{System, SystemData, World},
};

// computes LOD values for chunks
#[derive(SystemDesc)]
pub struct ChunkLodSystem;

impl Default for ChunkLodSystem {
    fn default() -> Self {
        ChunkLodSystem {}
    }
}

impl<'a> System<'a> for ChunkLodSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {}
}
