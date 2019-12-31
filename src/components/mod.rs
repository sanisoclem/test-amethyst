pub mod critter;
pub mod level;
pub mod terrain;

pub trait NamedPrefab {
    fn name(&self) -> Option<&str>;
}
