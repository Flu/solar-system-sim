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
    pub equatorial_radius: f64,
    pub mass: f64,
    pub surface_gravitation_acceleration: f64,
    pub sidereal_rotation_period: f64,
    pub soi: f64,
    pub color: PlanetColor
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Planet {
    pub name: String,
    pub body_type: String,
    pub equatorial_radius: f64,
    pub mass: f64,
    pub surface_gravitation_acceleration: f64,
    pub sidereal_rotation_period: f64,
    pub soi: f64,
    pub inclination: f64,
    pub arg_pe: f64,
    pub semi_major_axis: f64,
    pub apoapsis: f64,
    pub periapsis: f64,
    pub orbital_velocity_pe: f64,
    pub color: PlanetColor
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SolarSystem {
    pub stars: Vec<Star>,
    pub planets: Vec<Planet>
}