pub mod pan_orbit_camera;
pub mod rabbit;
pub mod sun;
pub mod tertian;
pub mod tree;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use pan_orbit_camera::*;
pub use rabbit::*;
pub use sun::*;
pub use tertian::*;
pub use tree::*;

pub struct BundlePlugins;

// impl Plugin for BundlePlugins {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(TreePlugin)
//             .add_plugin(PanOrbitCameraPlugin)
//             .add_plugin(RabbitPlugin);
//     }
// }

impl PluginGroup for BundlePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(PanOrbitCameraPlugin);
        group.add(RabbitPlugin);
        group.add(TertianPlugin);
        group.add(TreePlugin);
    }
}