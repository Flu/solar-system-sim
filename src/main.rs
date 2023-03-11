use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::mesh::VertexAttributeValues,
    window::{PresentMode, WindowPlugin},
};

mod planets;
use crate::planets::*;

mod camera;
use crate::camera::*;

mod planet_components;
use crate::planet_components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(orbit_camera)
        .add_system(focus_camera)
        .add_system(move_planets)
        .add_system(fps_counter_system)
        .add_system(celestial_body_information_system)
        .run();
}

fn read_planet_parameters() -> SolarSystem {
    let input_path = "assets/planets/planets.json";

    let solar_system = {
        let planets_json_string = std::fs::read_to_string(&input_path).unwrap();

        // Load the planet and star data from the string into the models
        serde_json::from_str::<SolarSystem>(&planets_json_string).unwrap()
    };
    return solar_system;
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let solar_system = read_planet_parameters();
    let sun_radius = solar_system.stars.iter().next().unwrap().radius;

    // Build the sun(s) of the system. Make them emit light
    for sun in solar_system.stars.iter() {
        let mut sun_mesh = Mesh::from(shape::Icosphere {
            radius: sun.radius,
            subdivisions: 5,
        });
        let color = &sun.color;

        // Construct the object and the mesh
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            sun_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let planet_color: Vec<[f32; 4]> = positions
                .iter()
                .map(|_| {
                    [
                        color.red as f32 / 255.0,
                        color.green as f32 / 255.0,
                        color.blue as f32 / 255.0,
                        0.01,
                    ]
                })
                .collect();
            sun_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, planet_color);
        }

        commands
            .spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    point_light: PointLight {
                        intensity: 3.573E+27, // lumens - roughly a 10000W non-halogen incandescent bulb
                        color: Color::ANTIQUE_WHITE,
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },
                FocusableEntity { is_focused: true },
                CelestialBody::new(sun.mass, &sun.name),
            ))
            .with_children(|builder| {
                builder.spawn(PbrBundle {
                    mesh: meshes.add(sun_mesh),
                    material: materials.add(StandardMaterial {
                        base_color: Color::ANTIQUE_WHITE,
                        emissive: Color::rgba_linear(7.13, 7.13, 7.13, 0.0),
                        ..default()
                    }),
                    ..default()
                });
            });
    }

    for planet in solar_system.planets.iter() {
        let mut planet_mesh = Mesh::from(shape::Icosphere {
            radius: planet.radius,
            subdivisions: 5,
        });

        // Construct the object and the mesh
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            planet_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let color = &planet.color;
            let planet_color: Vec<[f32; 4]> = positions
                .iter()
                .map(|_| {
                    [
                        color.red as f32 / 255.0,
                        color.green as f32 / 255.0,
                        color.blue as f32 / 255.0,
                        1.0,
                    ]
                })
                .collect();
            planet_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, planet_color);
        }

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(planet_mesh),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                transform: Transform::from_xyz(planet.periapsis + sun_radius, 0., 0.0),
                ..default()
            },
            FocusableEntity::default(),
            Velocity::from_xyz(0.0, 0.0, -planet.orbital_velocity_pe),
            Acceleration::from_xyz(0.0, 0.0, 0.0),
            CelestialBody::new(planet.mass, &planet.name),
        ));
    }

    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/SmallLcdSignRegular.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/SmallLcdSignRegular.ttf"),
                font_size: 15.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));

    let normal_style = TextStyle {
        font: asset_server.load("fonts/SmallLcdSignRegular.ttf"),
        font_size: 10.0,
        color: Color::WHITE,
    };

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Smallest recorded distance\t",
                normal_style
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/SmallLcdSignRegular.ttf"),
                font_size: 10.0,
                color: Color::BLUE
            })])
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        max_size: Size {
            width: Val::Px(500.),
            height: Val::Undefined,
        },
        ..default()
    }), CelestialBodyInformationText)
);

    spawn_camera(commands);
}

fn move_planets(
    suns: Query<(&Transform, &CelestialBody), Without<Acceleration>>,
    mut planets: Query<(
        &mut Transform,
        &mut Acceleration,
        &mut Velocity,
        &CelestialBody,
    )>,
) {
    let dv = 2f32;
    let grav_constant = 6.6743E-11;

    for (sun_position, sun_body) in suns.iter() {
        for (
            mut planet_position,
            mut planet_acceleration,
            mut planet_velocity,
            planet_body,
        ) in planets.iter_mut()
        {
            let r_vector = sun_position.translation - planet_position.translation;
            let distance = r_vector.length();
            let acc = ((grav_constant * sun_body.mass * planet_body.mass* r_vector)
                / (distance * distance * distance))
                / planet_body.mass;

            planet_acceleration.vector = acc;

            planet_velocity.vector += acc * dv;

            planet_position.translation += planet_velocity.vector;
        }
    }
}

#[derive(Component)]
struct FpsText;

fn fps_counter_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

#[derive(Component)]
struct CelestialBodyInformationText;

fn celestial_body_information_system(
    mut planets: Query<(
        &Transform,
        &Acceleration,
        &Velocity,
        &mut CelestialBody,
        &FocusableEntity,
    )>,
    mut texts: Query<&mut Text, With<CelestialBodyInformationText>>,
) {
    for (pos, acc, vel, mut body, focus) in planets.iter_mut() {
        let speed = vel.vector.length();
        let distance = pos.translation.length();

        if speed > body.highest_speed {
            body.highest_speed = speed;
        }
        if speed < body.smallest_speed {
            body.smallest_speed = speed;
        }
        if distance > body.smallest_distance {
            body.smallest_distance = distance;
        }
        if distance < body.highest_distance {
            body.highest_distance = distance;
        }

        // Update text only when body is in focus
        if focus.is_focused {
            for mut text in &mut texts {
                text.sections[1].value = format!("{distance:.2}");
            }
        }
    }
}
