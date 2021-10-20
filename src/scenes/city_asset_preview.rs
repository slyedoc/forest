use bevy::{math::*, prelude::*};
use bevy_dolly::prelude::*;
use strum::IntoEnumIterator;
use crate::prelude::*;

pub struct CityAssetPreviewPlugin;
impl Plugin for CityAssetPreviewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::CityAssetPreview)
                .with_system(setup)
                .with_system(light_default_setup)
        ).add_system_set(
            SystemSet::on_update(AppState::CityAssetPreview)
                .with_system(back_to_menu_system)
        )
        .add_system_set(
            SystemSet::on_exit(AppState::CityAssetPreview)
                .with_system(clear_system)
        );
    }
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(DollyControlCameraBundle {
            rig: Rig::default()
                .add(RigPosition::default())
                .add(Rotation::default())
                .add(Smooth::new(1.0, 1.0)),
            transform: Transform::from_xyz(-5.0, 5.0, -5.0)
                .looking_at(vec3(10.0, 0.0, 10.0), Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"));

    // Spawn one of each asset
    let mut current = vec3(0.0, 0.0, 0.0);

    for e in Sign::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Sign(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Detail::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Detail(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }


    for e in Wall::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Wall(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(1.0, 0.0, 0.0);
    }

    for e in Roof::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Roof(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 4.0;

    for e in Small::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Small(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 4.0;

    for e in Low::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Low(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 4.0;

    for e in Large::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Large(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(2.0, 0.0, 0.0);
    }

    current.x = 0.0;
    current.z += 4.0;

    for e in Skyscraper::iter() {
        commands.spawn_bundle(CityAssetBundle {
            city_type: CityType::Skyscraper(e),
            transform: Transform::from_translation(current),
            ..Default::default()
        });
        current += vec3(3.0, 0.0, 0.0);
    }

}