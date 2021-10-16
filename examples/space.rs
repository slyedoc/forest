mod helper;

use bevy::prelude::*;
//use bevy_mod_picking::*;
use forest::prelude::*;

fn main() {
    App::new()
        .add_plugin(AppPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(CameraBundle {
        transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::Y, Vec3::Y),
        ..Default::default()
    });

    // Spawn some ground for it to stand on
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            material: materials.add(Color::rgb(0.2, 0.4, 0.2).into()),
            ..Default::default()
        })
        //.insert_bundle(PickableBundle::default())
        .insert(Name::new("Ground"));
}
