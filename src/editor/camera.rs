use bevy::{prelude::*, render::camera::PerspectiveProjection};
use std::fmt;

use bevy_input_actionmap::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::PickingCameraBundle;

use crate::helper::*;

use super::EditorState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionPlugin::<EditorCameraAction>::default())
            .add_system_set(SystemSet::on_enter(EditorState::Loading).with_system(spawn_cameras))
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing)
                    .with_system(cleanup_system::<EditorCamera>.system())
                    .with_system(cleanup_actions_system::<EditorCameraAction>.system()),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum EditorCameraAction {
    Orbit,
    Pan,
}

impl fmt::Display for EditorCameraAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorCameraAction::Orbit => write!(f, "Orbit Camera"),
            EditorCameraAction::Pan => write!(f, "Pan Camera"),
        }
    }
}

/// Marker component for editor game camera
#[derive(Inspectable, Debug)]
pub enum EditorCamera {
    UI,
    Perspective,
}

/// Spawn a camera like this
#[allow(dead_code)]
pub fn spawn_cameras(mut commands: Commands, mut input_map: ResMut<InputMap<EditorCameraAction>>) {
    let location = Vec3::new(100.0, 100.0, 600.0);
    let radius = location.length();

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(location).looking_at(Vec3::ZERO, Vec3::Y),
            perspective_projection: PerspectiveProjection {
                far: f32::MAX,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(EditorCamera::Perspective)
        .insert(Name::new("Editor 3d Camera"));

    input_map
        .bind(EditorCameraAction::Orbit, MouseButton::Right)
        .bind(EditorCameraAction::Pan, MouseButton::Middle);
}

/// Tags an entity as capable of panning and orbiting.
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
            radius: 5.0,
            upside_down: false,
        }
    }
}
