pub mod procedural;
pub mod rabbit;
pub mod tertian;
pub mod tree;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use procedural::*;
pub use rabbit::*;
pub use tertian::*;
pub use tree::*;

pub struct BundlePlugins;

impl PluginGroup for BundlePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(RabbitPlugin);
        group.add(TertianPlugin);
        group.add(TreePlugin);
        group.add(ProceduralPlugin);
    }
}
