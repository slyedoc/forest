mod path_config;
mod resource_inspector;
mod window_config;
use bevy::prelude::*;

pub use path_config::*;
pub use resource_inspector::*;
use window_config::*;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<PathConfig>()
            .add_plugin(WindowConfigPlugin);
    }
}

// from bevy bounding
#[derive(Default, Debug)]
pub struct LocalAabb {
    #[allow(dead_code)]
    pub minimums: Vec3,
    /// The coordinates of the point located at the maximum x, y, and z coordinate. This can also
    /// be thought of as the length of the +x, +y, +z axes that extend from the origin and touch
    /// the inside of the bounding box faces.
    pub maximums: Vec3,
}

pub fn compute_aabb(vertices: &[Vec3]) -> LocalAabb {
    let mut maximums = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    let mut minimums = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    for vertex in vertices.iter() {
        maximums = vertex.max(maximums);
        minimums = vertex.min(minimums);
    }
    LocalAabb { minimums, maximums }
}
