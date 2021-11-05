use bevy::{math::*, prelude::*, render::camera::Camera};

use crate::prelude::*;

pub struct AstroidPlugin;
impl Plugin for AstroidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Astroid)
                .with_system(setup)
                .with_system(light_default_setup),
        )
        .add_system_set(SystemSet::on_update(AppState::Astroid).with_system(back_to_menu_system))
        .add_system_set(SystemSet::on_exit(AppState::Astroid).with_system(clear_system));
    }
}

fn setup(mut commands: Commands) {
    let _player = commands
        .spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Craft(Craft::SpeederA),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Player"))
        .id();

    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            camera: Camera {
                name: Some("Camera3d".to_string()),
                ..Default::default()
            },
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"));
}

pub fn back_to_menu_system(mut state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();

        keys.reset(KeyCode::Escape);
    }
}
