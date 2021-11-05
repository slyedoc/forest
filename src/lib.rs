#![allow(clippy::type_complexity)]
#![feature(derive_default_enum)]

mod bundles;

mod editor;
mod environment;
mod helper;
mod scenes;
mod spawner;
mod style;

pub mod prelude {
    pub use crate::{
        bundles::*, editor::*, environment::*, helper::*, spawner::*,
        scenes::*, style::*,
        AppPlugin, AppState,
    };
}

use bevy_dolly::DollyPlugin;
use prelude::*;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use heron::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 8 })
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(WindowDescriptor {
                title: "Forests".to_string(),
                vsync: false, // disable to break 60 fps
                ..Default::default()
            })
            // Bevy Plugins
            .add_plugins(DefaultPlugins)
            .insert_resource(WorldInspectorParams {
                enabled: false,
                despawnable_entities: true,
                ..Default::default()
            })
            .add_plugin(FrameTimeDiagnosticsPlugin)

            // 3rd-Party Plugins
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(DollyPlugin)
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
            
            //.add_plugin(bevy_transform_gizmo::TransformGizmoPlugin)
            //.add_plugin(ConfigCam)
            //.add_plugin(RapierPhysicsPlugin::<MyUserData>::default())
            //.add_plugin(RapierRenderPlugin)
            //.add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default())
            //.add_plugin(BoundingVolumePlugin::<sphere::BSphere>::default())
            //.add_plugin(BoundingVolumePlugin::<obb::Obb>::default())
            //.add_plugin(DebugLinesPlugin)
            // .insert_resource(DebugLines {
            //     depth_test: true,
            //     ..Default::default()
            // })
            //.add_plugin(PickingPlugin)
            //.add_plugin(DefaultPickingPlugins)
            //.add_plugin(DebugCursorPickingPlugin)
            //.add_plugin(DebugEventsPickingPlugin)

            // Local Plugins
            .add_plugin(EditorPlugin)
            .add_plugin(SpawnerPlugin)
            .add_plugin(StylePlugin)
            .add_plugin(HelperPlugin)
            .add_plugin(BundlePlugin);
    }

}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    SolarSystem,
    Astroid,
    SpiderAttack,

    // Tests
    LSystemTest,
    SpaceAssetPreview,
    CityAssetPreview,
}

//#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_state(AppState::Menu)
        .add_plugin(AppPlugin)
        .add_plugins(ScenePlugins)
            // when building for Web, use WebGL2 rendering
        .run();
}
