pub mod forest;
pub mod menu;
pub mod tree_test;
pub mod turtle_test;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use { forest::*, menu::*, tree_test::*, turtle_test::*, };
#[derive(Component)]
struct StateCleanup;

pub struct StatePlugins;
impl PluginGroup for StatePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(MenuPlugin);
        group.add(ForestPlugin);
        group.add(TreeTestPlugin);
        group.add(TurtleTestPlugin);
        //group.add(DragonCurvePlugin);
    }
}
