pub mod critter;
pub mod level;
pub mod terrain;

pub use terrain::Biome;

pub trait NamedPrefab {
    fn name(&self) -> Option<&str>;
}
