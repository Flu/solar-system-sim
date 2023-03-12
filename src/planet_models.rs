use bevy::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::physical_constant_models::*;

#[derive(Resource, Debug)]
pub struct SolarSystemConfiguration {
    pub solar_system: SolarSystemModel,
    pub physical_constants: PhysicalConstantsModel,
}

impl FromWorld for SolarSystemConfiguration {
    fn from_world(_: &mut World) -> Self {
        // You have full access to anything in the ECS from here.
        // For instance, you can mutate other resources:
        let solar_input_path = "assets/planets/planets.json";

        let solar_system = {
            let planets_json_string = std::fs::read_to_string(&solar_input_path).unwrap();

            // Load the planet and star data from the string into the models
            serde_json::from_str::<SolarSystemModel>(&planets_json_string).unwrap()
        };

        let constants_input_path = "assets/planets/physical_constants.json";

        let physical_constants = {
            let constants_json_string = std::fs::read_to_string(&constants_input_path).unwrap();

            // Load the constants data from the string into the models
            serde_json::from_str::<PhysicalConstantsModel>(&constants_json_string).unwrap()
        };

        SolarSystemConfiguration {
            solar_system,
            physical_constants,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct PlanetColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PlanetColor {
    pub fn to_rgba(&self) -> [f32;4] {
        [
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
            0.01,
        ]
    }

    pub fn to_color(&self) -> Color {
        Color::Rgba { red: self.red as f32 / 255.0, green: self.green as f32 / 255.0, blue: self.blue as f32 / 255.0, alpha: 0.0 }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StarModel {
    pub name: String,
    pub body_type: String,
    pub radius: f32,
    pub mass: f32,
    pub surface_gravitation_acceleration: f32,
    pub sidereal_rotation_period: f32,
    pub soi: f32,
    pub color: PlanetColor,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlanetModel {
    pub name: String,
    pub body_type: String,
    pub color_texture: String,
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
    pub color: PlanetColor,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SolarSystemModel {
    pub stars: Vec<StarModel>,
    pub planets: Vec<PlanetModel>,
}
