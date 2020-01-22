use amethyst::input::BindingTypes;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ActionBinding {
    Shoot,
}

#[derive(Debug)]
pub struct MovementBindings;

impl BindingTypes for MovementBindings {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
