use bevy::prelude::*;
use bevy_dolly::prelude::*;

/// Spawn a few basic thingsx
#[allow(dead_code)]
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

#[derive(Component)]
pub struct Rotator;

/// Rotate the meshes to demonstrate how the bounding volumes update
pub fn rotation_system(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query:Query<&mut Transform, With<Rotator>>,
    mut enabled: Local<bool>,
) {
    if input.just_pressed(KeyCode::Space) {
        *enabled = !*enabled;
    }
    if *enabled {
        for mut transform in query.iter_mut() {
            let scale = Vec3::ONE * ((time.seconds_since_startup() as f32).sin() * 0.3 + 1.0) * 0.3;
            let rot_x =
                Quat::from_rotation_x((time.seconds_since_startup() as f32 / 5.0).sin() / 50.0);
            let rot_y =
                Quat::from_rotation_y((time.seconds_since_startup() as f32 / 3.0).sin() / 50.0);
            let rot_z =
                Quat::from_rotation_z((time.seconds_since_startup() as f32 / 4.0).sin() / 50.0);
            transform.scale = scale;
            transform.rotate(rot_x * rot_y * rot_z);
        }
    }
}
