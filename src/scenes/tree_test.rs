use super::StateCleanup;
use crate::{bundles::*, helper::*, AppState};
use bevy::prelude::*;

/// Template Placeholder
pub struct TreeTestPlugin;
impl Plugin for TreeTestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::TreeTest).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::TreeTest).with_system(back_to_menu_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::TreeTest)
                    .with_system(cleanup_system::<StateCleanup>),
            );
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands
        .spawn_bundle(PanOrbitCameraBundle::new(
            Vec3::new(0.0, 5.0, -5.0),
            Vec3::ZERO,
        ))
        .insert(StateCleanup)
        .insert(Name::new("Camera"));

    // Light
    commands
        .spawn_bundle(SunBundle::new())
        .insert(StateCleanup)
        .insert(Name::new("Sun"));

    // Tertian
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(StateCleanup)
        .insert(Name::new("Tertian"));


    // trees
    commands.spawn_bundle(ProceduralBundle {
             data: procedural::DragonCurve { iterations : 2 },
             transform: Transform::from_xyz(0.0, 0.0, 0.0),
             ..Default::default()
         })
        .insert(StateCleanup)
        .insert(Name::new("- Primary -"));


}
