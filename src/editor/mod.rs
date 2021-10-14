mod ui;
mod spawner;

use crate::helper::*;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::WorldInspectorParams;
use spawner::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Playing,
    Disabled,
}

/// Provides Bevy Editor for Debugging
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        info!("Toggle Editor - F12");
        info!("Toggle World Inspector - F11");

        #[cfg(feature = "editor")]
        app.add_state(EditorState::Disabled)
            .add_event::<SpawnEvent>()
            .add_system_set(
                SystemSet::on_enter(EditorState::Playing)
                .with_system(setup_playing)

            )
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(ui::toolbar_system)
                .with_system(spawner_system),
            )
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing)
                    .with_system(ui::close_windows_system)
                    .with_system(cleanup_system::<EditorCleanup>),
            )
            .add_system(action_system);
    }
}
#[derive(Component)]
struct EditorCleanup;

fn setup_playing(mut _commands: Commands) {

    // commands
    //     .spawn_bundle(PerspectiveCameraBundle {
    //         transform: Transform::from_xyz(100.0, 100.0, 600.0),
    //         ..Default::default()
    //     })
    //     .insert(Name::new("Editor 3d Camera"))
    //     .insert(EditorCleanup);

    // commands
    //     .spawn_bundle(UiCameraBundle::default())
    //     .insert(Name::new("Editor Ui Camera"))
    //     .insert(EditorCleanup);
}

fn action_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut state: ResMut<State<EditorState>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut spawner_event: EventWriter<SpawnEvent>
) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        match state.current() {
            EditorState::Playing => state.pop().unwrap(),
            EditorState::Disabled => state.push(EditorState::Playing).unwrap(),
        };
    }

    if keyboard_input.just_pressed(KeyCode::F11) {
        world_inspection.enabled = !world_inspection.enabled;
    }

    if keyboard_input.just_pressed(KeyCode::F10) {
        spawner_event.send(SpawnEvent(SpawnType::City));
    }

}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
