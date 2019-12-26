use crate::components::RotateMe;
use amethyst::{
  core::{
    math::{Vector3},
    transform::Transform,
    Time,
  },
  ecs::*,
};

pub struct RotatorSystem;

impl<'s> System<'s> for RotatorSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, RotateMe>,
    Read<'s, Time>,
  );

  fn run(&mut self, (mut transforms, rms, time): Self::SystemData) {
    let delta_time = time.delta_seconds();
    for (transform, rm) in (&mut transforms, &rms).join() {
      transform.append_rotation(Vector3::<f32>::x_axis(), rm.angular_velocity * delta_time);
    }
  }
}
