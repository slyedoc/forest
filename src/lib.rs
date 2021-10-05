#![allow(clippy::type_complexity)]
#![feature(derive_default_enum)]

mod bundles;
mod editor;
mod helper;
mod scenes;
mod style;
mod linden_mayer;

use wasm_bindgen::prelude::*;
use bevy_prototype_debug_lines::*;
use bundles::*;
use editor::*;
use helper::*;
use scenes::*;
use style::*;
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Forest,
    TreeTest,
    TurtleTest,
}

#[wasm_bindgen]
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
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin)
        .insert_resource(DebugLines { depth_test: true, ..Default::default() })
        //.add_plugin(PickingPlugin)
        //.add_plugin(DefaultPickingPlugins)
        //.add_plugin(DebugCursorPickingPlugin)
        //.add_plugin(DebugEventsPickingPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // Local Plugins
        .add_plugin(EditorPlugin)
        .add_plugin(StylePlugin)
        .add_plugin(HelperPlugin)
        .add_plugins(BundlePlugins)
        .add_plugins(StatePlugins);

        #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

        app.run();
}
