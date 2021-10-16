pub mod procedural;
pub mod rabbit;
pub mod tertian;
pub mod tree;
pub mod city;
pub mod space;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use procedural::*;
pub use rabbit::*;
pub use tertian::*;
pub use tree::*;
pub use city::*;
pub use space::*;

pub struct BundlePlugins;

impl PluginGroup for BundlePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(RabbitPlugin);
        group.add(TertianPlugin);
        group.add(TreePlugin);
        group.add(ProceduralPlugin);
        group.add(CityPlugin);
        group.add(SpacePlugin);
    }
}
