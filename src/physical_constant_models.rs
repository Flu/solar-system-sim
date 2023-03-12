use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PhysicalConstantsModel {
    pub gravitational_constant: f32,
    pub dv: f32
}