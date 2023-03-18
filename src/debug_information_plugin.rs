use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::planet_components::{CelestialBody, FocusableEntity};

pub struct DebugInformationPlugin;

impl Plugin for DebugInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_text)
            .add_system(update_fps)
            .add_system(update_planet_name_text)
            .add_system(update_r_vector_text)
            .add_system(update_acceleration_vector_text)
            .add_system(update_speed_vector_text);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct DebugInfoPlanetInfo;

#[derive(Component)]
struct DebugInfoRVector;

#[derive(Component)]
struct DebugInfoSpeedVector;

#[derive(Component)]
struct DebugInfoAccelerationVector;

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/LcdRoundedRegular.ttf");

    let parameter_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    let value_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::ALICE_BLUE,
    };

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));

    // Add debugging information
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("", parameter_style.clone()),
            TextSection::from_style(value_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(20.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        DebugInfoPlanetInfo,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("R-vector: ", parameter_style.clone()),
            TextSection::from_style(value_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        DebugInfoRVector,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Speed: ", parameter_style.clone()),
            TextSection::from_style(value_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(50.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        DebugInfoSpeedVector,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("Acceleration: ", parameter_style.clone()),
            TextSection::from_style(value_style.clone()),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(65.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        DebugInfoAccelerationVector,
    ));
}

fn update_fps(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

fn update_planet_name_text(
    planets: Query<( &CelestialBody, &FocusableEntity)>,
    mut texts: Query<(&mut Text, &mut DebugInfoPlanetInfo)>,
) {
    for (body, focus) in planets.iter() {
        if focus.is_focused {
            for (mut text, _) in texts.iter_mut() {
                let name = body.name.to_owned();
                let mass = body.mass;
                text.sections[1].value = format!("Name: {name} Mass: {mass} M tonnes");
            }
        }
    }
}

fn update_r_vector_text(
    planets: Query<(&Transform, &FocusableEntity)>,
    mut texts: Query<(&mut Text, &mut DebugInfoRVector)>,
) {
    for (pos, focus) in planets.iter() {
        if focus.is_focused {
            for (mut text, _) in texts.iter_mut() {
                let x = pos.translation.x;
                let y = pos.translation.y;
                let z = pos.translation.z;
                
                let distance_from_sun = pos.translation.length();
                text.sections[1].value =
                    format!("[{x:.5}, {y:.5}, {z:.5}] {distance_from_sun:.3} Mm");
            }
        }
    }
}

fn update_speed_vector_text(
    planets: Query<(&CelestialBody, &FocusableEntity)>,
    mut texts: Query<(&mut Text, &mut DebugInfoSpeedVector)>,
) {
    for (body, focus) in planets.iter() {
        if focus.is_focused {
            for (mut text, _) in texts.iter_mut() {
                let x = body.vel.vector.x;
                let y = body.vel.vector.y;
                let z = body.vel.vector.z;

                let speed = body.vel.vector.length();
                text.sections[1].value = format!("[{x:.5}, {y:.5}, {z:.5}] {speed:.3} Mm/day");
            }
        }
    }
}

fn update_acceleration_vector_text(
    planets: Query<(&CelestialBody, &FocusableEntity)>,
    mut texts: Query<(&mut Text, &mut DebugInfoAccelerationVector)>,
) {
    for (body, focus) in planets.iter() {
        if focus.is_focused {
            for (mut text, _) in texts.iter_mut() {
                let x = body.acc.vector.x;
                let y = body.acc.vector.y;
                let z = body.acc.vector.z;

                let acc = body.acc.vector.length();
                text.sections[1].value = format!("[{x:.5}, {y:.5}, {z:.5}] {acc:.3} Mm/day^2");
            }
        }
    }
}
