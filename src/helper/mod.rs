// TODO: Yea I hate the helper classes too
// For now this is for qol utilities
mod path_config;
mod resource_inspector;
mod window_config;

use bevy::{ecs::component::Component, prelude::*};

pub use path_config::*;
pub use resource_inspector::*;
use window_config::*;

use crate::AppState;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<PathConfig>()
            .add_plugin(WindowConfigPlugin);
    }
}

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn back_to_menu_system(mut state: ResMut<State<AppState>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();

        keys.reset(KeyCode::Escape);
    }
}

// from bevy bounding
pub struct Aabb {
    #[allow(dead_code)]
    pub minimums: Vec3,
    /// The coordinates of the point located at the maximum x, y, and z coordinate. This can also
    /// be thought of as the length of the +x, +y, +z axes that extend from the origin and touch
    /// the inside of the bounding box faces.
    pub maximums: Vec3,
}

pub fn compute_aabb(vertices: &[Vec3]) -> Aabb {
    let mut maximums = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
    let mut minimums = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    for vertex in vertices.iter() {
        maximums = vertex.max(maximums);
        minimums = vertex.max(minimums);
    }
    Aabb { minimums, maximums }
}
