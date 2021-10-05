use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};
use bevy_input_actionmap::{ActionPlugin, InputMap};
use bevy_mod_picking::PickingCameraBundle;

/// TODO: REPLACE WITH SOMEONE ELSES
pub struct PanOrbitCameraPlugin;
impl Plugin for PanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin::<PanOrbitCameraAction>::default())
            .add_startup_system(setup)
            // TODO: Limit this to ActiveCamera
            .add_system(mouse_movement_system)
            .add_system(keyboard_movement_system)
            ;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PanOrbitCameraAction {
    Orbit,
    Pan,
    Left,
    Right,
    Forward,
    Back,
}


fn setup(mut input_map: ResMut<InputMap<PanOrbitCameraAction>>) {
    input_map
        .bind(PanOrbitCameraAction::Orbit, MouseButton::Right)
        .bind(PanOrbitCameraAction::Pan, MouseButton::Middle)
        .bind( PanOrbitCameraAction::Left, KeyCode::A)
        .bind( PanOrbitCameraAction::Right, KeyCode::D)
        .bind( PanOrbitCameraAction::Forward, KeyCode::W)
        .bind( PanOrbitCameraAction::Back, KeyCode::S);
}



pub fn keyboard_movement_system(
    time: Res<Time>,
    input_map: Res<InputMap<PanOrbitCameraAction>>,
    mut query: Query<&mut Transform, With<PanOrbitCamera>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if input_map.active(PanOrbitCameraAction::Left) {
            direction += transform.left();
        }

        if input_map.active(PanOrbitCameraAction::Right) {
            direction += transform.right();
        }

        if input_map.active(PanOrbitCameraAction::Forward) {
            direction += transform.forward();
        }

        if input_map.active(PanOrbitCameraAction::Back) {
            direction += transform.back();
        }

        let move_dir = direction * 100.0 * time.delta_seconds();
        transform.translation += move_dir;
    }
}

#[derive(Default, Bundle)]
pub struct PanOrbitCameraBundle {
    config: PanOrbitCamera,

    #[bundle]
    camera: PerspectiveCameraBundle,
    #[bundle]
    picking_camera: PickingCameraBundle,
}

#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    focus: Vec3,
    radius: f32,
    upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

impl PanOrbitCameraBundle {
    pub fn new(pos: Vec3, looking_at: Vec3) -> Self {
        Self {
            camera: PerspectiveCameraBundle {
                transform: Transform::from_translation(pos).looking_at(looking_at, Vec3::Y),
                perspective_projection: PerspectiveProjection {
                    far: f32::MAX,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}


// This is from the bevy cheatbook
/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn mouse_movement_system(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
       // change input mapping for orbit and panning here
       let orbit_button = MouseButton::Right;
       let pan_button = MouseButton::Middle;
   
       let mut pan = Vec2::ZERO;
       let mut rotation_move = Vec2::ZERO;
       let mut scroll = 0.0;
       let mut orbit_button_changed = false;
   
       if input_mouse.pressed(orbit_button) {
           for ev in ev_motion.iter() {
               rotation_move += ev.delta;
           }
       } else if input_mouse.pressed(pan_button) {
           // Pan only if we're not rotating at the moment
           for ev in ev_motion.iter() {
               pan += ev.delta;
           }
       }
       for ev in ev_scroll.iter() {
           scroll += ev.y;
       }
       if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
           orbit_button_changed = true;
       }
   
       for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
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
                   if pan_orbit.upside_down { -delta } else { delta }
               };
               let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
               let yaw = Quat::from_rotation_y(-delta_x);
               let pitch = Quat::from_rotation_x(-delta_y);
               transform.rotation = yaw * transform.rotation; // rotate around global y axis
               transform.rotation = transform.rotation * pitch; // rotate around local x axis
           } else if pan.length_squared() > 0.0 {
               any = true;
               // make panning distance independent of resolution and FOV,
               let window = get_primary_window_size(&windows);
               pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
               // translate by local axes
               let right = transform.rotation * Vec3::X * -pan.x;
               let up = transform.rotation * Vec3::Y * pan.y;
               // make panning proportional to distance away from focus point
               let translation = (right + up) * pan_orbit.radius;
               pan_orbit.focus += translation;
           } else if scroll.abs() > 0.0 {
               any = true;
               pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
               // dont allow zoom to reach zero or you get stuck
               pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
           }
   
           if any {
               // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
               // parent = x and y rotation
               // child = z-offset
               let rot_matrix = Mat3::from_quat(transform.rotation);
               transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
           }
       }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    Vec2::new(window.width() as f32, window.height() as f32)
}
