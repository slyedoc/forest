#![allow(clippy::type_complexity)]
#![feature(derive_default_enum)]

mod bundles;
mod camera;
mod editor;
mod environment;
mod helper;
mod scenes;
mod spawner;
mod style;

pub mod prelude {
    pub use crate::{
        bundles::*, camera::*, editor::*, environment::*, helper::*, spawner::*,
        AppPlugin, AppState,
    };
}

use bevy_mod_bounding::{BoundingVolumePlugin, aabb};
use bundles::BundlePlugins;
use camera::*;
use editor::*;
use helper::*;
use prelude::EnvironmentPlugin;
use scenes::*;
use spawner::SpawnerPlugin;
use style::StylePlugin;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 8 })
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(WindowDescriptor {
                title: "Forests".to_string(),
                vsync: true, // disable to break 60 fps
                ..Default::default()
            })
            .add_plugins(DefaultPlugins)
            .insert_resource(WorldInspectorParams {
                enabled: false,
                despawnable_entities: true,
                ..Default::default()
            })
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(bevy_mod_picking::DefaultPickingPlugins)
            .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
            .add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default())
            //.add_plugin(DebugLinesPlugin)
            // .insert_resource(DebugLines {
            //     depth_test: true,
            //     ..Default::default()
            // })
            //.add_plugin(PickingPlugin)
            //.add_plugin(DefaultPickingPlugins)
            //.add_plugin(DebugCursorPickingPlugin)
            //.add_plugin(DebugEventsPickingPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            // Local Plugins
            .add_plugin(CameraPlugin)
            .add_plugin(EnvironmentPlugin)
            .add_plugin(EditorPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(StylePlugin)
            .add_plugin(HelperPlugin)
            .add_plugins(BundlePlugins);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Forest,
    TreeTest,
    TurtleTest,
}

pub fn run() {
    App::new()
        .add_state(AppState::Menu)
        .add_plugin(AppPlugin)
        .add_plugins(StatePlugins)
        .run();
    //.add_plugin(BoundingVolumePlugin::<sphere::BSphere>::default())
    //.add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default())
    //.add_plugin(BoundingVolumePlugin::<obb::Obb>::default());
}
