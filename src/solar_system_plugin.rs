use crate::{planet_components::*, planet_models::*};
use bevy::{prelude::*, render::mesh::VertexAttributeValues};

pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SolarSystemConfiguration>()
            .add_startup_system(create_sun_and_planets)
            .add_system(move_planets);
    }
}

fn create_sun_and_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    config: Res<SolarSystemConfiguration>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create the suns of the system
    for sun in config.solar_system.stars.iter() {
        let mesh = create_mesh(sun.radius, sun.color);

        commands
            .spawn((
                PointLightBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    point_light: PointLight {
                        intensity: 3.573E+27, // lumens - roughly a 10000W non-halogen incandescent bulb
                        color: sun.color.to_color(),
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },
                FocusableEntity { is_focused: true },
                CelestialBody {
                    mass: sun.mass,
                    name: sun.name.clone(),
                    radius: sun.radius,
                    vel: Velocity::default(),
                    acc: Acceleration::default(),
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

    for planet in config.solar_system.planets.iter() {
        let mesh = create_mesh(planet.radius, planet.color);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(1., 1., 1.).into()),
                transform: Transform::from_xyz(planet.periapsis, 0., 0.),
                ..default()
            },
            FocusableEntity::default(),
            CelestialBody {
                mass: planet.mass,
                name: planet.name.clone(),
                radius: planet.radius,
                vel: Velocity::from_xyz(0.0, 0.0, -planet.orbital_velocity_pe),
                acc: Acceleration::from_xyz(0.0, 0.0, 0.0),
            },
            Planet,
        ));
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
            let acc = ((constants.physical_constants.gravitational_constant
                * sun_body.mass
                * planet_body.mass
                * r_vector)
                / (distance * distance * distance))
                / planet_body.mass;

            planet_body.acc.vector = acc;

            planet_body.vel.vector += acc * constants.physical_constants.dv;

            planet_pos.translation += planet_body.vel.vector * constants.physical_constants.dv;

            dbg!(r_vector);
            dbg!(distance);
            dbg!(planet_body.acc.vector);
            dbg!(planet_body.vel.vector);
        }
    }
}

fn create_mesh(radius: f32, color: PlanetColor) -> Mesh {
    // Create the mesh of the sun
    let mut mesh = Mesh::from(shape::Icosphere {
        subdivisions: 10,
        radius,
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
