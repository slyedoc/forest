mod helper;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_mod_bounding::*;
use forest::prelude::*;
use helper::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        //.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            despawnable_entities: true,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        // Our plugins
        .add_plugin(CameraPlugin)
        .add_plugins(BundlePlugins)
        .add_plugin(EditorPlugin)
        .add_startup_system(setup)
        .add_plugin(BoundingVolumePlugin::<aabb::Aabb>::default())
        .add_system(rotation_system)
        //.add_startup_system(setup_example_scene)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::Plane::default(),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Test"))
        //.insert(Bounded::<aabb::Aabb>::default())
        //.insert(debug::DebugBounds)
        .insert(Rotator);

    commands.spawn_bundle(CameraBundle{
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::Y, Vec3::Y),
        ..Default::default()
    });

    // Spawn some ground for it to stand on
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.2).into()),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    // Create a light so we can see it once we add a camera
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(-3.0, 5.0, -3.0),
        ..Default::default()
    });
}

pub fn setup_example_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(DollyControlCameraBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Spawn some ground for it to stand on
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.2).into()),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    // Create a light so we can see it once we add a camera
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
