use bevy::{ecs::query::WorldQuery, prelude::*};

#[derive(Component)]
pub struct FocusableEntity {
    pub is_focused: bool,
}

impl Default for FocusableEntity {
    fn default() -> Self {
        FocusableEntity { is_focused: false }
    }
}

#[derive(Component, Default)]
pub struct CelestialBody {
    pub mass: f32,
    pub name: String,
    pub smallest_distance: f32,
    pub highest_distance: f32,
    pub smallest_speed: f32,
    pub highest_speed: f32,
}

impl CelestialBody {
    pub fn new(mass: f32, name: &String) -> Self {
        CelestialBody {
            mass,
            name: name.to_owned(),
            smallest_distance: 0.0,
            highest_distance: 0.0,
            smallest_speed: 0.0,
            highest_speed: 0.0,
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity {
    pub vector: Vec3,
}

impl Velocity {
    pub fn new(velocity_vector: Vec3) -> Self {
        Velocity {
            vector: velocity_vector,
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Velocity {
            vector: Vec3::new(x, y, z),
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Acceleration {
    pub vector: Vec3,
}

impl Acceleration {
    pub fn new(acceleration_vector: Vec3) -> Self {
        Acceleration {
            vector: acceleration_vector,
        }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Acceleration {
            vector: Vec3::new(x, y, z),
        }
    }
}
