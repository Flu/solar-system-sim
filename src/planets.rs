use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PlanetColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Star {
    pub name: String,
    pub body_type: String,
    pub radius: f32,
    pub mass: f32,
    pub surface_gravitation_acceleration: f32,
    pub sidereal_rotation_period: f32,
    pub soi: f32,
    pub color: PlanetColor
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Planet {
    pub name: String,
    pub body_type: String,
    pub radius: f32,
    pub mass: f32,
    pub surface_gravitation_acceleration: f32,
    pub sidereal_rotation_period: f32,
    pub soi: f32,
    pub inclination: f32,
    pub arg_pe: f32,
    pub semi_major_axis: f32,
    pub apoapsis: f32,
    pub periapsis: f32,
    pub orbital_velocity_pe: f32,
    pub color: PlanetColor
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SolarSystem {
    pub stars: Vec<Star>,
    pub planets: Vec<Planet>
}