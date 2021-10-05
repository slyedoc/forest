pub mod pan_orbit_camera;
pub mod procedural;
pub mod rabbit;
pub mod sun;
pub mod tertian;
pub mod tree;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use pan_orbit_camera::*;
pub use procedural::*;
pub use rabbit::*;
pub use sun::*;
pub use tertian::*;
pub use tree::*;

pub struct BundlePlugins;

impl PluginGroup for BundlePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(PanOrbitCameraPlugin);
        group.add(RabbitPlugin);
        group.add(TertianPlugin);
        group.add(TreePlugin);
        group.add(ProceduralPlugin);
        //group.add(DragonCurvePlugin);
    }
}
