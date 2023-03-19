use crate::{planet_components::*, planet_models::*, labels::*};
use bevy::{prelude::*, render::mesh::VertexAttributeValues, input::{keyboard::KeyboardInput, ButtonState}};

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SolarSystemConfiguration>()
            .add_startup_system(create_sun_and_planets)
            .add_system(move_planets
                .label(SystemTypes::PhysicsLabel)
                .before(SystemTypes::CameraLabel))
            .add_system(rotate_planets
                .label(SystemTypes::PhysicsLabel)
                .before(SystemTypes::CameraLabel))
            .add_system(change_time_dv
                .label(SystemTypes::PhysicsLabel)
                .before(SystemTypes::CameraLabel));
    }
}

fn create_sun_and_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<SolarSystemConfiguration>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create the suns of the system
    for sun in config.solar_system.stars.iter() {
        let mesh = create_mesh(sun.radius, sun.color);

        commands
            .spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    point_light: PointLight {
                        intensity: 5.573E+10, // lumens
                        color: sun.color.to_color(),
                        shadows_enabled: true,
                        range: f32::MAX,
                        ..default()
                    },
                    ..default()
                },
                FocusableEntity { is_focused: true },
                CelestialBody {
                    mass: sun.mass,
                    name: sun.name.clone(),
                    radius: sun.radius,
                    gravitational_parameter: sun.gravitational_parameter,
                    vel: Velocity::default(),
                    acc: Acceleration::default(),
                    rot: 2.0*(std::f32::consts::PI)/sun.sidereal_rotation_period,
                    inclination: 0.0,
                },
                Star,
            ))
            .with_children(|builder| {
                builder.spawn(PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(StandardMaterial {
                        base_color: sun.color.to_color(),
                        emissive: sun.color.to_color(),
                        perceptual_roughness: 0.3,
                        ..default()
                    }),
                    ..default()
                });
            });
    }

    if let Some(sun) = config.solar_system.stars.iter().next() {
        for planet in config.solar_system.planets.iter() {
            let mut mesh = create_mesh(planet.radius, planet.color);

            let base_color_texture: Option<Handle<Image>> = if planet.color_texture != "" {
                Some(asset_server.load(planet.color_texture.to_owned()))
            } else {
                None
            };

            let normal_map_texture: Option<Handle<Image>> = if planet.normal_texture != "" {
                Some(asset_server.load(planet.normal_texture.to_owned()))
            } else {
                None
            };

            Mesh::generate_tangents(&mut mesh).expect("Something");

            let planet_pbr_bundle = PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: planet.color.to_color(),
                    base_color_texture,
                    normal_map_texture,
                    perceptual_roughness: 0.9,
                    reflectance: 0.2,
                    ..default()
                }),
                transform:
                    Transform::from_xyz(planet.periapsis + sun.radius, 0., 0.)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2 + (planet.inclination/180. * std::f32::consts::PI)))
                    ,
                ..default()
            };

            commands.spawn((planet_pbr_bundle,
                FocusableEntity::default(),
                CelestialBody {
                    mass: planet.mass,
                    name: planet.name.clone(),
                    radius: planet.radius,
                    gravitational_parameter: 0.0,
                    vel: Velocity::from_xyz(0.0, 0.0, -planet.orbital_velocity_pe),
                    acc: Acceleration::from_xyz(0.0, 0.0, 0.0),
                    rot: 2.0*(std::f32::consts::PI)/planet.sidereal_rotation_period,
                    inclination: planet.inclination,
                },
                Planet,
            ));
        }
    }
}

fn move_planets(
    suns: Query<(&Transform, &CelestialBody), Without<Planet>>,
    mut planets: Query<(&mut Transform, &mut CelestialBody, &Planet)>,
    constants: Res<SolarSystemConfiguration>,
) {
    //dbg!(constants.physical_constants.dv);
    for (sun_pos, sun_body) in suns.iter() {
        for (mut planet_pos, mut planet_body, _) in planets.iter_mut() {
            let r_vector = sun_pos.translation - planet_pos.translation;
            let distance = r_vector.length();

            let acc = (sun_body.gravitational_parameter * r_vector) / (distance * distance * distance);

            planet_body.acc.vector = acc;

            planet_body.vel.vector += acc * constants.physical_constants.dv;

            planet_pos.translation += planet_body.vel.vector * constants.physical_constants.dv;
        }
    }
}

fn rotate_planets(
    mut planets: Query<(&mut Transform, &CelestialBody, &Planet)>,
    constants: Res<SolarSystemConfiguration>,
) {
    for (mut planet_pos, planet_body, _) in planets.iter_mut() {
        planet_pos.rotate_y(planet_body.rot*constants.physical_constants.dv);
    }
}

fn change_time_dv(
    mut key_evr: EventReader<KeyboardInput>,
    mut constants: ResMut<SolarSystemConfiguration>
) {

    let increase_dv_button = KeyCode::Period;
    let decrease_dv_button = KeyCode::Comma;

    for ev in key_evr.iter() {
        // Change focus to the next object when the button is pressed

        if ev.state == ButtonState::Pressed && ev.key_code == Some(increase_dv_button) {
            constants.physical_constants.dv *= 10.0;
        }

        if ev.state == ButtonState::Pressed && ev.key_code == Some(decrease_dv_button) {
            constants.physical_constants.dv /= 10.0;
        }
    }
}

fn create_mesh(radius: f32, color: PlanetColor) -> Mesh {
    // Create the mesh of the sun
    let mut mesh = Mesh::from(shape::UVSphere {
        radius,
        sectors: 100,
        stacks: 50,
        ..default()
    });

    // Create the object and color it
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    {
        let body_color: Vec<[f32; 4]> = positions.iter().map(|_| color.to_rgba()).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, body_color);
    }

    return mesh;
}
