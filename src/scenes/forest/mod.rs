


use crate::{
    bundles::*,
    helper::{back_to_menu_system, cleanup_system},
    AppState,
};
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand::prelude::*;

use super::StateCleanup;

/// This is where the demo scene that uses everything else goes (long way to go)
pub struct ForestPlugin;
impl Plugin for ForestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Forest).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Forest)
                    .with_system(back_to_menu_system)
                    // big brain testing
                    //.with_system(ai::thirst_system)
                    //.with_system(ai::drink_action_system)
                    //.with_system(ai::thirsty_scorer_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Forest).with_system(cleanup_system::<StateCleanup>),
            );
    }
}

fn setup(
    mut commands: Commands,
    rabbit_assets: Res<RabbitAssets>,
    tertian_assets: Res<TertianAssets>,
) {
    // settings this up here in for
    let mut rng = StdRng::from_entropy();

    // Camera
    commands
        .spawn_bundle(PanOrbitCameraBundle::new(
            Vec3::new(0.0, 5.0, -5.0),
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
        .spawn_bundle(TertianBundle::new(Vec3::ZERO, &tertian_assets))
        .insert_bundle(PickableBundle::default())
        .insert(StateCleanup)
        .insert(Name::new("Tertian"));

    // Rabbits
    for i in 0..20 {
        let pos = Vec3::new(rng.gen_range(-10.0..10.0), 0.0, rng.gen_range(-10.0..10.00));
        commands
            .spawn_bundle(RabbitBundle::new(pos, &rabbit_assets))
            .insert_bundle(PickableBundle::default())
            .insert(StateCleanup)
            .insert(Name::new(format!("Rabbit {}", i)));
    }

}
