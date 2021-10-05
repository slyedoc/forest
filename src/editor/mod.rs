mod ui;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_input_actionmap::{ActionPlugin, InputMap};
use bevy_inspector_egui::WorldInspectorParams;

use crate::{bundles::*, helper::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Playing,
    Disabled,
}

/// Provides Bevy Editor for Debugging
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        app.add_state(EditorState::Disabled)
            .add_system_set(SystemSet::on_enter(EditorState::Playing).with_system(setup_playing))
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(ui::toolbar_system),
            )
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing)
                    .with_system(ui::close_windows_system)
                    .with_system(cleanup_system::<EditorCleanup>),
            )
            .add_startup_system(setup_actions)
            .add_system(action_system);

        app.add_plugin(ActionPlugin::<EditorAction>::default());

        // let mut state = app.world.get_resource_mut::<State<EditorState>>().unwrap();
        // state.push(EditorState::Playing).unwrap();
    }
}
#[derive(Component)]
struct EditorCleanup;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum EditorAction {
    ToggleEditor,
    ToggleWorld,
}

fn setup_actions(mut input_map: ResMut<InputMap<EditorAction>>) {
    info!("Toggle Editor - F12");
    info!("Toggle World Inspector - F11");
    input_map.bind(EditorAction::ToggleEditor, KeyCode::F12);
    input_map.bind(EditorAction::ToggleWorld, KeyCode::F11);
}

fn setup_playing(mut commands: Commands) {
    commands
        .spawn_bundle(PanOrbitCameraBundle::new(
            Vec3::new(100.0, 100.0, 600.0),
            Vec3::ZERO,
        ))
        .insert(Name::new("Editor 3d Camera"))
        .insert(EditorCleanup);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(Name::new("Editor Ui Camera"))
        .insert(EditorCleanup);
}

fn action_system(
    input_map: Res<InputMap<EditorAction>>,
    mut state: ResMut<State<EditorState>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
) {
    if input_map.just_active(EditorAction::ToggleEditor) {
        match state.current() {
            EditorState::Playing => state.pop().unwrap(),
            EditorState::Disabled => state.push(EditorState::Playing).unwrap(),
        };
    }

    if input_map.just_active(EditorAction::ToggleWorld) {
        world_inspection.enabled = !world_inspection.enabled;
    }
}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
