mod helper;

use bevy::prelude::*;
use bevy_dolly::prelude::*;
use bevy_inspector_egui::Inspectable;
//use bevy_mod_picking::*;
use forest::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(AppPlugin)
        .add_plugin(DollyPlugin)
        .add_startup_system(setup)
        .add_system(move_player)
        //.add_system(move_sheep_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(SpaceAssetBundle {
        space_type: SpaceType::Rock(Rock::LargeA),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let _player = commands
        .spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Craft(Craft::SpeederA),
            transform: Transform::from_xyz(0.0, 0.0, -10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Player::default())
        .insert(Name::new("Player"))
        .id();

    commands
        .spawn_bundle(DollyControlCameraBundle {
            rig: Rig::default()
            //.add(Anchor::new(player))
            .add(RigPosition::default())
            .add(Rotation::new(180.0, -30.0))
            .add(Smooth::new(0.0, 1.0))
            .add(Arm::new(Vec3::Z * 8.0)),
            transform: Transform::from_xyz(0.0, 2.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
             ..Default::default()
        })
        .insert_bundle(bevy_mod_picking::PickingCameraBundle::default())
        .insert(bevy_transform_gizmo::GizmoPickSource::default())
        .insert(Name::new("Camera"));

}

#[derive(Inspectable, Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 30.0 }
    }
}

pub fn move_player(input: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &Player)>) {
    for (mut t, p) in query.iter_mut() {
        // Update position
        let mut move_vec = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            move_vec.z += 1.0;
        }
        if input.pressed(KeyCode::S) {
            move_vec.z -= 1.0;
        }
        if input.pressed(KeyCode::A) {
            move_vec.x += 1.0;
        }
        if input.pressed(KeyCode::D) {
            move_vec.x -= 1.0;
        }
        t.translation += 0.001 * move_vec.clamp_length_max(1.0) * p.speed;
    }
}

// Move Player around
#[allow(dead_code)]
fn move_player_system2(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut left: Local<bool>,
) {
    if keys.just_pressed(KeyCode::Z) {
        *left = true;
    }
    if keys.just_pressed(KeyCode::X) {
        *left = false;
    }

    for mut t in query.iter_mut() {
        let rotation = t.rotation;
        t.translation += rotation * (Vec3::Z * 0.05);
        t.rotation *= Quat::from_rotation_y(if *left { 0.01 } else { -0.01 });
    }
}