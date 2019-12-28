use std::fmt::{self, Display};

use amethyst::input::{BindingTypes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
  ZAxis,
  YAxis,
  XAxis,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
  Interact,
}

impl Display for AxisBinding {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Display for ActionBinding {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Debug)]
pub struct GameBindings;

impl BindingTypes for GameBindings {
  type Axis = AxisBinding;
  type Action = ActionBinding;
}
