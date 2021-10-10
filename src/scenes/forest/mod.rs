use crate::{bundles::*, helper::*, AppState};

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand::prelude::*;

use super::{setup_default_scene, StateCleanup};

/// This is where the demo scene that uses everything else goes (long way to go)
pub struct ForestPlugin;
impl Plugin for ForestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Forest)
            .with_system(setup_default_scene)
            .with_system(setup))
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

fn setup(mut commands: Commands, rabbit_assets: Res<RabbitAssets>) {
    // settings this up here in for
    let mut rng = StdRng::from_entropy();

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
