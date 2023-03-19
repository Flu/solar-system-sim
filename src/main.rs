use bevy::prelude::*;

mod solar_system_plugin;
mod physical_constant_models;
mod planet_components;
mod planet_models;
mod camera_plugin;
mod debug_information_plugin;
mod labels;

use solar_system_plugin::*;
use camera_plugin::*;
use debug_information_plugin::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight { color: Color::Rgba { red: 0.01, green: 0.01, blue: 0.01, alpha: 1.0 }, brightness: 500.0})
        .add_plugins(DefaultPlugins)
        .add_plugin(SolarSystemPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugInformationPlugin)
        .run();
}