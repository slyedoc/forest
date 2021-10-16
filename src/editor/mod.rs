mod ui;

use crate::helper::*;
use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_inspector_egui::{*, plugin::InspectorWindows};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum EditorState {
    Playing,
    Disabled,
}

#[derive(Inspectable, Default)]
struct Inspector {
    #[inspectable(deletable = false)]
    active: Option<Entity>,
}

#[derive(Inspectable, Default)]
pub struct EditorWindows {
    pub spawner: bool,
    pub egui_settings: bool,
    pub egui_inspection: bool,
}

/// Provides Bevy Editor for Debugging
pub struct EditorPlugin;
impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        info!("Toggle Editor - F12");
        info!("Toggle World Inspector - F11");

        #[cfg(feature = "editor")]
        app.init_inspector_resource::<EditorWindows>()
            .init_inspector_resource::<Inspector>()
            .add_state(EditorState::Disabled)
            .add_system_set(SystemSet::on_enter(EditorState::Playing).with_system(setup_playing))
            .add_system_set(
                SystemSet::on_update(EditorState::Playing).with_system(ui::toolbar_system),
            )
            .add_system_set(
                SystemSet::on_exit(EditorState::Playing)
                    .with_system(ui::close_windows_system)
                    .with_system(cleanup_system::<EditorCleanup>),
            )
            .add_system(action_system)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                maintain_inspected_entities
                    // Could need to be after for faster interaction
                    //.after(bevy_mod_picking::PickingSystem::Focus),
            );
    }
}

fn maintain_inspected_entities(
    mut commands: Commands,
    mut inspector: ResMut<Inspector>,
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
) {
    let entity = query
        .iter()
        .filter(|(_, interaction)| matches!(interaction, Interaction::Clicked))
        .map(|(entity, _)| entity)
        .next();

    if let Some(entity) = entity {
        if let Some(active) = inspector.active {
            commands.entity(active).remove::<bevy_transform_gizmo::GizmoTransformable>();
            inspector.active = None;
        } else {
            commands.entity(entity).insert(bevy_transform_gizmo::GizmoTransformable);
            inspector.active = Some(entity);

        }
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
    mut windows: ResMut<EditorWindows>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    if keyboard_input.just_pressed(KeyCode::F12) {
        match state.current() {
            EditorState::Playing => state.pop().unwrap(),
            EditorState::Disabled => state.push(EditorState::Playing).unwrap(),
        };
    }

    if keyboard_input.just_pressed(KeyCode::F11) {
       let inspector = inspector_windows.window_data_mut::<Inspector>();
       inspector.visible = !inspector.visible;

    }
    if keyboard_input.just_pressed(KeyCode::F10) {
        world_inspection.enabled = !world_inspection.enabled;
    }

    if keyboard_input.just_pressed(KeyCode::F9) {
        windows.spawner = !windows.spawner;
    }
}

#[allow(dead_code)]
pub fn run_if_editor(state: Res<State<EditorState>>) -> ShouldRun {
    match state.current() {
        EditorState::Playing => ShouldRun::Yes,
        EditorState::Disabled => ShouldRun::No,
    }
}
