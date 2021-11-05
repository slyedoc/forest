use bevy::{math::*, prelude::*, render::camera::Camera};
use bevy_dolly::{DollyActions, prelude::Rig};
use strum::IntoEnumIterator;

use crate::prelude::*;

pub struct SpacePreviewPlugin;
impl Plugin for SpacePreviewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SpaceAssetPreview)
                .with_system(setup)
                .with_system(light_default_setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::SpaceAssetPreview).with_system(back_to_menu_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::SpaceAssetPreview).with_system(clear_system));
    }
}

fn setup(mut commands: Commands) {
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
        .insert(Rig::default())
        .insert(DollyActions::default())
        .insert(Name::new("Camera1"));

    // Spawn one of each asset
    let mut current = vec3(0.0, 0.0, 0.0);
    for e in Craft::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Craft(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(4.0, 0.0, 0.0);
    }
    commands.spawn_bundle(SpaceAssetBundle {
        space_type: SpaceType::Rover,
        transform: Transform::from_translation(current),
        ..Default::default()
    });
    current += vec3(4.0, 0.0, 0.0);

    current.x = 0.0;
    current.z += 4.0;

    for e in Character::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Character(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Weapon::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Weapon(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Turret::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Turret(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    commands.spawn_bundle(SpaceAssetBundle {
        space_type: SpaceType::Bones,
        transform: Transform::from_translation(current),
        ..Default::default()
    });
    current += vec3(1.0, 0.0, 0.0);

    for e in Barrel::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Barrel(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Chimney::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Chimney(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Crater::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Crater(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Desk::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Desk(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Corridor::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Corridor(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Gate::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Gate(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Machine::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Machine(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Meteor::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Meteor(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Monorail::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Monorail(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Pipe::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Pipe(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Platform::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Platform(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Rail::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Rail(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    for e in Rock::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Rock(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Rocket::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Rocket(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in SatelliteDish::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::SatelliteDish(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Stairs::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Stairs(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Structure::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Structure(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }
    for e in Supports::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Supports(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 2.0;

    for e in Terrain::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Terrain(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.5, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 5.0;

    for e in Hanger::iter() {
        commands.spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Hanger(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(3.0, 0.0, 0.0);
    }
}
