use crate::planet_components::*;
use bevy::input::keyboard::*;
use bevy::input::mouse::*;
use bevy::input::ButtonState;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(focus_camera)
            .add_system(orbit_camera);
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 3.0,
            upside_down: false,
        }
    }
}

fn focus_camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut cameras: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    mut query_focus: Query<(&mut FocusableEntity, &mut Transform), Without<PanOrbitCamera>>,
) {
    let change_focus_button = KeyCode::Tab;

    for ev in key_evr.iter() {
        // Change focus to the next object when the button is pressed

        if ev.state == ButtonState::Pressed && ev.key_code == Some(change_focus_button) {
            let focused_item = query_focus.iter_mut().position(|x| x.0.is_focused);

            if focused_item.is_none() {
                query_focus.iter_mut().next().unwrap().0.is_focused = true;
            } else {
                let index = focused_item.unwrap();
                query_focus.iter_mut().nth(index).unwrap().0.is_focused = false;

                if index == query_focus.iter().count() - 1 {
                    query_focus.iter_mut().next().unwrap().0.is_focused = true;
                } else {
                    query_focus.iter_mut().nth(index + 1).unwrap().0.is_focused = true;
                }
            }
        }
    }

    let focused = query_focus.iter().find(|x| x.0.is_focused).unwrap();
    for (mut pan_orbit, mut transform, _) in cameras.iter_mut() {
        pan_orbit.focus.x = focused.1.translation.x;
        pan_orbit.focus.y = focused.1.translation.y;
        pan_orbit.focus.z = focused.1.translation.z;

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
    }
}

/// Zoom with scroll wheel, orbit with right mouse click.
fn orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query_camera: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    planets: Query<(&FocusableEntity, &CelestialBody)>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, _) in query_camera.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;

            // minimum zoom is the radius of the currently focused body plus 1
            let min_zoom = planets.iter().find(|x| x.0.is_focused).unwrap().1.radius;
            pan_orbit.radius = f32::max(pan_orbit.radius, min_zoom+10.);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }

    // consume any remaining events, so they don't pile up if we don't need them
    // (and also to avoid Bevy warning us about not checking events every frame update)
    ev_motion.clear();
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Spawn a camera like this
fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = 750000.0;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}
