use super::actions::*;
use bevy::{
    prelude::*,
    render::{
        camera::{Camera, PerspectiveProjection, VisibleEntities},
        render_graph::base,
    },
};

#[derive(Component, Default)]
pub struct CameraBundleMarker;

#[derive(Bundle)]
pub struct CameraBundle {
    pub marker: CameraBundleMarker,
    pub control_actions: ControlActions,
    pub camera: Camera,
    pub perspective_projection: PerspectiveProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            control_actions: ControlActions::default(),
            camera: Camera {
                name: Some(base::camera::CAMERA_3D.to_string()),
                ..Default::default()
            },
            perspective_projection: Default::default(),
            visible_entities: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            marker: Default::default(),
        }
    }
}
