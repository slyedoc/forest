pub mod menu;
pub mod solar_system;
pub mod astroid;
pub mod spider_attack;
pub mod lsystem_test;
pub mod space_asset_preview;
pub mod city_asset_preview;
use space_asset_preview::*;
use city_asset_preview::*;
use spider_attack::*;

use bevy::{app::PluginGroupBuilder, prelude::*};
use menu::MenuPlugin;
use lsystem_test::LSystemPlugin;
use solar_system::SolarSystemPlugin;

use crate::AppState;

use astroid::AstroidPlugin;


pub struct ScenePlugins;
impl PluginGroup for ScenePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MenuPlugin);
        group.add(SolarSystemPlugin);
        group.add(AstroidPlugin);
        group.add(SpiderAttackPlugin);


        group.add(LSystemPlugin);
        group.add(SpacePreviewPlugin);
        group.add(CityAssetPreviewPlugin);
    }
}

pub fn back_to_menu_system(mut state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();

        keys.reset(KeyCode::Escape);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn clear_system(mut commands: Commands, q: Query<Entity>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
