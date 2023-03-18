use bevy::prelude::*;

#[derive(Component)]
pub struct FocusableEntity {
    pub is_focused: bool,
}

impl Default for FocusableEntity {
    fn default() -> Self {
        FocusableEntity { is_focused: false }
    }
}

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Star;

#[derive(Component, Default)]
pub struct CelestialBody {
    pub mass: f32,
    pub name: String,
    pub radius: f32,
    pub gravitational_parameter: f32,
    pub vel: Velocity,
    pub acc: Acceleration,
    pub rot: f32,
    pub inclination: f32
}

#[derive(Default, Debug)]
pub struct Velocity {
    pub vector: Vec3,
}

impl Velocity {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Velocity {
            vector: Vec3::new(x, y, z),
        }
    }
}

#[derive(Default, Debug)]
pub struct Acceleration {
    pub vector: Vec3,
}

impl Acceleration {

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Acceleration {
            vector: Vec3::new(x, y, z),
        }
    }
}
