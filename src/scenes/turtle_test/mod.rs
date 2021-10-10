use super::{setup_default_scene, StateCleanup};
use crate::{helper::*, AppState};
use bevy::prelude::*;

/// Template Placeholder
pub struct TurtleTestPlugin;
impl Plugin for TurtleTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::TurtleTest)
                .with_system(setup)
                .with_system(setup_default_scene),
        )
        .add_system_set(SystemSet::on_update(AppState::TurtleTest).with_system(back_to_menu_system))
        .add_system_set(
            SystemSet::on_exit(AppState::TurtleTest).with_system(cleanup_system::<StateCleanup>),
        );
    }
}

fn setup(mut _commands: Commands) {

    // TODO
}
