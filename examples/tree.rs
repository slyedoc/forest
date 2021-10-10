mod helper;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use forest::prelude::*;
use helper::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        
        // Our plugins
        .add_plugin(DollyPlugin)
        .add_plugins(BundlePlugins)

        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_example_scene)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(ProceduralBundle {
        data: procedural::KochCurve { iterations: 5 },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
