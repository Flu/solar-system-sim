use bevy::prelude::*;
use bevy::app::AppExit;

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
        .add_system(exit_system)
        .run();
}

fn exit_system(_: EventWriter<AppExit>) {
    //println!("The app has exited");
    //exit.send(AppExit);
}