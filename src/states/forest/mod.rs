use crate::{AppState, bundles::*, helper::cleanup_system};
use bevy::prelude::*;
use rand::prelude::*;

use super::StateCleanup;

pub struct ForestPlugin;

impl Plugin for ForestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::Forest).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Forest)
                    .with_system(escape_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Forest).with_system(cleanup_system::<StateCleanup>),
            );
    }
}

fn escape_system(mut state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();

        keys.reset(KeyCode::Escape);
    }
}

fn setup(
    mut commands: Commands,
    rabbit_assets: Res<RabbitAssets>,
    tertian_assets: Res<TertianAssets>,
    tree_assets: Res<TreeAssets>,
) {
    // settings this up here in for
    let mut rng = StdRng::from_entropy();

    // Camera
    commands
        .spawn_bundle(PanOrbitCameraBundle::new(
            Vec3::new(0.0, 15.0, -15.0),
            Vec3::ZERO,
        ))
        .insert(StateCleanup)
        .insert(Name::new("ForestCamera"));

    // Light
    commands
        .spawn_bundle(SunBundle::new())
        .insert(StateCleanup)
        .insert(Name::new("Sun"));

    // Tertian
    commands
        .spawn_bundle(TertianBundle::new( Vec3::ZERO, &tertian_assets))
        .insert(StateCleanup)
        .insert(Name::new("Tertian"));

    // Rabbits
    for i in 0..10 {
        let pos = Vec3::new(
            rng.gen_range(-10.0..10.0),
            0.0,
            rng.gen_range(-10.0..10.00),
        );
        commands
            .spawn_bundle(RabbitBundle::new(pos, &rabbit_assets ))
            .insert(StateCleanup)
            .insert(Name::new(format!("Rabbit {}", i)));
    }

    // trees
    commands.spawn_bundle(TreeBundle::new(Vec3::ZERO, &tree_assets))
        .insert(StateCleanup)
        .insert(Name::new("Tree"));
}



