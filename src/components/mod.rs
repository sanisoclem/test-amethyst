pub mod critter;
pub mod level;

pub trait NamedPrefab {
  fn name(&self) -> Option<&str>;
}
