/// raw JSON representation of the model
pub mod unresolved;

use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct StringConstraint {
    pub max_length: Option<usize>,
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct NumericConstraint<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}
