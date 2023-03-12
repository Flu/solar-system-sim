use bevy::prelude::*;

mod solar_system_plugin;
mod physical_constant_models;
mod planet_components;
mod planet_models;
mod camera_plugin;
mod debug_information_plugin;

use solar_system_plugin::*;
use camera_plugin::*;
use debug_information_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .add_plugin(SolarSystemPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugInformationPlugin)
        .run();
}