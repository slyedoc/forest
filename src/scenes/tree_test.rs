use super::{setup_default_scene, StateCleanup};
use crate::{bundles::*, helper::*, AppState};
use bevy::prelude::*;

/// Template Placeholder
pub struct TreeTestPlugin;
impl Plugin for TreeTestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::TreeTest)
                .with_system(setup)
                .with_system(setup_default_scene),
        )
        .add_system_set(SystemSet::on_update(AppState::TreeTest).with_system(back_to_menu_system))
        .add_system_set(
            SystemSet::on_exit(AppState::TreeTest).with_system(cleanup_system::<StateCleanup>),
        );
    }
}

fn setup(mut commands: Commands) {
    // trees
    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::DragonCurve { iterations: 2 },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(StateCleanup)
        .insert(Name::new("Tree"));
}
