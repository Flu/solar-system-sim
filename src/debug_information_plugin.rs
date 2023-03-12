use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*
};

use crate::planet_components::{CelestialBody, FocusableEntity};

pub struct DebugInformationPlugin;

impl Plugin for DebugInformationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(setup_text)
            .add_system(update_fps)
            .add_system(update_planet_parameters);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct DebugInfoText;

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {

    let font = asset_server.load("fonts/LcdRoundedRegular.ttf");
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
        ]),
        FpsText,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Current distance from sun: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "\nCurrent speed vector: ",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::ALICE_BLUE,
            }),
            TextSection::from_style(TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::ALICE_BLUE,
            })
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        DebugInfoText,
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

fn update_planet_parameters(
    planets: Query<(&Transform, &CelestialBody, &FocusableEntity)>,
    mut texts: Query<(&mut Text, &mut DebugInfoText)>
) {

    for (pos, body, focus) in planets.iter() {
        if focus.is_focused {
            for (mut text, _) in texts.iter_mut() {
                let distance_from_sun = pos.translation.length();
                text.sections[2].value = format!("{distance_from_sun:.2} m");

                let (vec_x, vec_y, vec_z) = (body.vel.vector.x, body.vel.vector.x, body.vel.vector.x);
                let speed = body.vel.vector.length();
                text.sections[3].value = format!(r"[{vec_x:.4}, {vec_y:.4}, {vec_z:.4}], {speed:.4} m/s");
            }
        }
    }
}
