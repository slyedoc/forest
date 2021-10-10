#![allow(clippy::type_complexity)]
#![feature(derive_default_enum)]

mod bundles;
mod editor;
mod helper;
mod linden_mayer;
mod scenes;
mod style;

use bevy_dolly::DollyPlugin;
use bevy_prototype_debug_lines::*;

pub mod prelude {
    pub use crate::{bundles::*, editor::*, helper::*};
}

use bundles::BundlePlugins;
use editor::*;
use helper::*;
use scenes::*;
use style::StylePlugin;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
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
        .add_plugin(DebugLinesPlugin)
        .insert_resource(DebugLines {
            depth_test: true,
            ..Default::default()
        })
        //.add_plugin(PickingPlugin)
        //.add_plugin(DefaultPickingPlugins)
        //.add_plugin(DebugCursorPickingPlugin)
        //.add_plugin(DebugEventsPickingPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(DollyPlugin)
        // Local Plugins
        .add_plugin(EditorPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(HelperPlugin)
        .add_plugins(BundlePlugins)
        .add_plugins(StatePlugins);

    app.run();
}
