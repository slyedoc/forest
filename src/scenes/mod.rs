pub mod forest;
pub mod menu;
pub mod tree_test;
pub mod turtle_test;
use crate::bundles::*;
use bevy_dolly::prelude::*;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use {forest::*, menu::*, tree_test::*, turtle_test::*};
#[derive(Component)]
struct StateCleanup;

pub struct StatePlugins;
impl PluginGroup for StatePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MenuPlugin);
        group.add(ForestPlugin);
        group.add(TreeTestPlugin);
        group.add(TurtleTestPlugin);
        //group.add(DragonCurvePlugin);
    }
}

pub fn setup_default_scene(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(DollyControlCameraBundle {
            transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(StateCleanup)
        .insert(Name::new("Camera"));

    // Light
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(StateCleanup)
        .insert(Name::new("Sun"));

    // Tertian
    commands
        .spawn_bundle(TertianBundle::default())
        .insert(StateCleanup)
        .insert(Name::new("Tertian"));

    // ui camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(StateCleanup)
        .insert(Name::new("UI Camera"));
}
