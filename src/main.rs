use bevy::{prelude::*, render::mesh::VertexAttributeValues};

mod planets;
use crate::planets::*;

mod camera;
use crate::camera::*;

mod planet_entity;
use crate::planet_entity::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(pan_orbit_camera)
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
) {

    let solar_system = read_planet_parameters();
    let mut position = 0.0;

    // Build the sun(s) of the system. Make them emit light
    for sun in solar_system.stars.iter() {
        let mut sun_mesh = Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 5});
        let color = &sun.color;

        // Construct the object and the mesh
        if let Some(VertexAttributeValues::Float32x3(positions)) = sun_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            
            let planet_color: Vec<[f32; 4]> = positions
                .iter()
                .map(|_| [color.red as f32 / 255.0, color.green as f32 / 255.0, color.blue as f32 / 255.0, 0.6])
                .collect();
            sun_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, planet_color);
        }

        commands.spawn((PointLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            point_light: PointLight {
                intensity: 100000.0, // lumens - roughly a 10000W non-halogen incandescent bulb
                color: Color::ANTIQUE_WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        }, FocusableEntity { is_focused: true}))
            .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: meshes.add(sun_mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::ANTIQUE_WHITE,
                    emissive: Color::rgba_linear(color.red as f32 / 255.0, color.green as f32 / 255.0, color.blue as f32 / 255.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
    }

    for planet in solar_system.planets.iter() {
        let mut planet_mesh = Mesh::from(shape::Icosphere {radius: 1.0, subdivisions: 5});

        // Construct the object and the mesh
        if let Some(VertexAttributeValues::Float32x3(positions)) = planet_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let color = &planet.color;
            let planet_color: Vec<[f32; 4]> = positions
                .iter()
                .map(|_| [color.red as f32 / 255.0, color.green as f32 / 255.0, color.blue as f32 / 255.0, 1.0])
                .collect();
            planet_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, planet_color);
        }

        commands.spawn((PbrBundle {
            mesh: meshes.add(planet_mesh),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: Transform::from_xyz(position, 5.0, 0.0)
                .with_scale(Vec3::new(1.,1.,1.)),
            
            ..default()
        }, FocusableEntity::default()));

        position += 3.0;
    }

    spawn_camera(commands);
}