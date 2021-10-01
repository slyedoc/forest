mod actions;
mod camera;
mod grid;
mod ui;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_input_actionmap::InputMap;
use bevy_inspector_egui::WorldInspectorParams;
pub use camera::*;
pub use grid::*;
use std::fmt;
pub use ui::*;

use self::actions::ActionsWindow;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Loading,
    Playing,
    Disabled,
}

pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(EditorState::Disabled)
            .insert_resource(InputMap::<EditorAction>::default())
            // see https://github.com/bevyengine/bevy/issues/2312
            .add_plugin(CameraPlugin)
            //.add_plugin(GridPlugin)
            //.add_plugin(UIPlugin)
            .insert_resource(ActionsWindow { enabled: true })
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(actions::draw_actions),
            )
            .add_startup_system(setup)
            .add_system(actions_system);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum EditorAction {
    Editor,
    World,
    Keys,
}

impl fmt::Display for EditorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorAction::Editor => write!(f, "Toggle Editor"),
            EditorAction::World => write!(f, "Open World Inspector"),
            EditorAction::Keys => write!(f, "Toggle Key Mappings"),
        }
    }
}

fn setup(mut input_map: ResMut<InputMap<EditorAction>>) {
    input_map.bind(EditorAction::Editor, KeyCode::F12);
    input_map.bind(EditorAction::World, KeyCode::F11);
    input_map.bind(EditorAction::Keys, KeyCode::F10);
}

fn actions_system(
    input_map: Res<InputMap<EditorAction>>,
    mut state: ResMut<State<EditorState>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut action_window: ResMut<ActionsWindow>,
) {
    if input_map.just_active(EditorAction::Editor) {
        info!("editor action");
        let result = match state.current() {
            // could only happen if loading takes a while for first frame, but go ahead and disable editor if so
            EditorState::Loading => EditorState::Disabled,
            EditorState::Playing => EditorState::Disabled,
            EditorState::Disabled => EditorState::Loading,
        };
        state.set(result).expect("Editor state didn't set");
    }

    if input_map.just_active(EditorAction::World) {
        world_inspection.enabled = !world_inspection.enabled;
    }

    if input_map.just_active(EditorAction::Keys) {
        action_window.enabled = !action_window.enabled;
    }
}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Loading => ShouldRun::Yes,
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
