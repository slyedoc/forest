mod actions;
mod bundle;
pub use actions::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
pub use bundle::*;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // These are only use for camera control system
            .init_resource::<CameraConfig>()
            .add_system_to_stage(CoreStage::PreUpdate, update_camera_system);
    }
}

/// Configuration Resource for Dolly Controlled Rigs
// TODO: We could store the targeting data here, would really make user
// interaction
pub struct CameraConfig {
    pub speed: f32,
    pub key_rotation: f32,
    pub boost_multiplyer: f32,
    pub sensitivity: Vec2,
    pub position_smoothness: f32,
    pub rotation_smoothness: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            speed: 10.0,
            key_rotation: 15.0,
            boost_multiplyer: 5.0,
            sensitivity: Vec2::splat(0.1),
            position_smoothness: 2.0,
            rotation_smoothness: 2.0,
        }
    }
}

// An ad-hoc multiplier to make default smoothness parameters
// produce good-looking results.
//const SMOOTHNESS_MULT: f32 = 8.0;

/// Updates rigs with a generic control system
///
/// This only runs for DollyControlCameraBundles, not DollyCameraBundles
/// 
fn update_camera_system(
    time: Res<Time>,
    input_keys: Res<Input<KeyCode>>,
    input_mouse_btn: Res<Input<MouseButton>>,
    config: Res<CameraConfig>,
    mut windows: ResMut<Windows>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &ControlActions)>,
) {
    for (mut t, control_actions) in query.iter_mut() {
        let window = windows.get_primary_mut().unwrap();
        // Update position
        let mut move_vec = Vec3::ZERO;
        if control_actions.key_pressed(Action::Forward, &input_keys) {
            move_vec.z -= 1.0;
        }
        if control_actions.key_pressed(Action::Backward, &input_keys) {
            move_vec.z += 1.0;
        }
        if control_actions.key_pressed(Action::Left, &input_keys) {
            move_vec.x -= 1.0;
        }
        if control_actions.key_pressed(Action::Right, &input_keys) {
            move_vec.x += 1.0;
        }
        if control_actions.key_pressed(Action::Up, &input_keys) {
            move_vec.y += 1.0;
        }
        if control_actions.key_pressed(Action::Down, &input_keys) {
            move_vec.y -= 1.0;
        }

        // apply a turbo
        let boost = match control_actions.key_pressed(Action::Boost, &input_keys) {
            true => config.boost_multiplyer,
            false => 1.0,
        };

        // Make movement relative to current transform(camera) and limit effect
        move_vec = t.rotation * move_vec.clamp_length_max(1.0);
        //move_vec.y = 0.0;

        // Update rotation
        let mut delta = Vec2::ZERO;

        if control_actions.key_pressed(Action::RotateLeft, &input_keys) {
            delta.x -= 10.0;
        }
        if control_actions.key_pressed(Action::RotateRight, &input_keys) {
            delta.x += 10.0;
        }

        // Mouse Enable Look
        if let Some(btn) = control_actions.mouse_map.get(&Action::EnableLook) {
            look_around(
                window,
                &input_mouse_btn,
                btn,
                &mut mouse_motion_events,
                &mut delta,
            );
        }

        // Apply the move
        let delta_time = time.delta_seconds();
        t.translation += move_vec * delta_time * config.speed * boost;

        if delta != Vec2::ZERO {
            // careful here, easy to get wrong
            let yaw = Quat::from_rotation_y(-delta.x * config.sensitivity.x * delta_time);
            let pitch = Quat::from_rotation_x(-delta.y * config.sensitivity.y * delta_time);
            t.rotation = yaw * t.rotation; // rotate around global y axis
            t.rotation *= pitch; // rotate around local x axis
        }

        // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
        // parent = x and y rotation
        // child = z-offset
        //let rot_matrix = Mat3::from_quat(t.rotation);
        //t.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));

        // t.rotate(Quat::from_rotation_x((-0.1 * delta.x * config.sensitivity.x).to_radians()));
        // t.rotate(Quat::from_rotation_y((-0.1 * delta.y * config.sensitivity.y).to_radians()));
        // //( Quat::from_euler(
        //     bevy::math::EulerRot::YXZ,
        //     (-0.1 * delta.x * config.sensitivity.x).to_radians(),
        //     (-0.1 * delta.y * config.sensitivity.y).to_radians(),
        //     0.0,
        // ));
        // let rot_x = Quat::from_rotation_x((-0.1 * delta.x * config.sensitivity.x).to_radians() * delta_time);
        // let rot_ = Quat::from_rotation_y((-0.1 * delta.y * config.sensitivity.y).to_radians() * delta_time);
        // t.rotate(rot_x * rot_y );
    }
}


// fn pan_orbit_camera(
//     windows: Res<Windows>,
//     mut ev_motion: EventReader<MouseMotion>,
//     mut ev_scroll: EventReader<MouseWheel>,
//     input_mouse: Res<Input<MouseButton>>,
//     mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
// ) {
//     // change input mapping for orbit and panning here
//     let orbit_button = MouseButton::Right;
//     let pan_button = MouseButton::Middle;

//     let mut pan = Vec2::ZERO;
//     let mut rotation_move = Vec2::ZERO;
//     let mut scroll = 0.0;
//     let mut orbit_button_changed = false;

//     if input_mouse.pressed(orbit_button) {
//         for ev in ev_motion.iter() {
//             rotation_move += ev.delta;
//         }
//     } else if input_mouse.pressed(pan_button) {
//         // Pan only if we're not rotating at the moment
//         for ev in ev_motion.iter() {
//             pan += ev.delta;
//         }
//     }
//     for ev in ev_scroll.iter() {
//         scroll += ev.y;
//     }
//     if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
//         orbit_button_changed = true;
//     }

//     for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
//         if orbit_button_changed {
//             // only check for upside down when orbiting started or ended this frame
//             // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
//             let up = transform.rotation * Vec3::Y;
//             pan_orbit.upside_down = up.y <= 0.0;
//         }

//         let mut any = false;
//         if rotation_move.length_squared() > 0.0 {
//             any = true;
//             let window = get_primary_window_size(&windows);
//             let delta_x = {
//                 let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
//                 if pan_orbit.upside_down { -delta } else { delta }
//             };
//             let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
//             let yaw = Quat::from_rotation_y(-delta_x);
//             let pitch = Quat::from_rotation_x(-delta_y);
//             transform.rotation = yaw * transform.rotation; // rotate around global y axis
//             transform.rotation = transform.rotation * pitch; // rotate around local x axis
//         } else if pan.length_squared() > 0.0 {
//             any = true;
//             // make panning distance independent of resolution and FOV,
//             let window = get_primary_window_size(&windows);
//             pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
//             // translate by local axes
//             let right = transform.rotation * Vec3::X * -pan.x;
//             let up = transform.rotation * Vec3::Y * pan.y;
//             // make panning proportional to distance away from focus point
//             let translation = (right + up) * pan_orbit.radius;
//             pan_orbit.focus += translation;
//         } else if scroll.abs() > 0.0 {
//             any = true;
//             pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
//             // dont allow zoom to reach zero or you get stuck
//             pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
//         }

//         if any {
//             // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
//             // parent = x and y rotation
//             // child = z-offset
//             let rot_matrix = Mat3::from_quat(transform.rotation);
//             transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
//         }
//     }
// }


fn look_around<T: Copy + Eq + std::hash::Hash>(
    window: &mut Window,
    input: &Input<T>,
    btn: &T,
    mouse_motion_events: &mut EventReader<MouseMotion>,
    delta: &mut Vec2,
) {
    if input.just_pressed(*btn) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
    if input.just_released(*btn) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
    if input.pressed(*btn) {
        for event in mouse_motion_events.iter() {
            *delta += event.delta;
        }
    }
}
