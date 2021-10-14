#![allow(clippy::type_complexity)]
#![feature(derive_default_enum)]

mod bundles;
mod editor;
mod helper;
mod scenes;
mod style;
mod camera;
mod city;
mod environment;
mod loading;

pub mod prelude {
    pub use crate::{AppState, bundles::*, editor::*, helper::*, camera::*, city::*, environment::* };
}

use bundles::BundlePlugins;
use city::CityPlugin;
use editor::*;
use helper::*;
use loading::LoadingPlugin;
use scenes::*;
use camera::*;
use style::StylePlugin;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Menu,
    Forest,
    TreeTest,
    TurtleTest,
}

pub fn run() {
    let mut app = App::new();
    app.add_state(AppState::Menu)
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Forests".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
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
        .add_plugin(CityPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(HelperPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugins(BundlePlugins)
        .add_plugins(StatePlugins);

        //.add_plugin(BoundingVolumePlugin::<sphere::BSphere>::default())
        //.add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default())
        //.add_plugin(BoundingVolumePlugin::<obb::Obb>::default());

    app.run();
}
