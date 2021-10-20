use bevy::{math::*, prelude::*};
use bevy_dolly::prelude::*;

use crate::prelude::*;

pub struct AstroidPlugin;
impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Astroid)
                .with_system(setup)
                .with_system(light_default_setup)
        ).add_system_set(
            SystemSet::on_update(AppState::Astroid)
                .with_system(back_to_menu_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Astroid)
                .with_system(clear_system)
        );
    }
}

fn setup(mut commands: Commands) {
    
    let player = commands.spawn_bundle(SpaceAssetBundle {
        space_type: SpaceType::Craft(Craft::SpeederA),
        transform: Transform::from_xyz(0.0 , 0.0 , 0.0),
        ..Default::default()
    })
    .insert(Name::new("Player"))
    .id();
    
    // Camera
    commands
        .spawn_bundle(DollyControlCameraBundle {
            rig: Rig::default()
                //.add(RigPosition::default())
                .add(Anchor::new(player))
                .add(Rotation::default())
                .add( Arm::new(Vec3::Z * 10.0))
                .add(Smooth::new(1.0, 1.0)),
            transform: Transform::from_xyz(-5.0, 5.0, -5.0)
                .looking_at(vec3(10.0, 0.0, 10.0), Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default())
        .insert(Name::new("Camera"));

    
}

pub fn back_to_menu_system(mut state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();

        keys.reset(KeyCode::Escape);
    }
}