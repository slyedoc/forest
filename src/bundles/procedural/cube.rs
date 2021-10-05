// From bevy_render/src/mesh/shape/mod.rs
// Import so I can add Component and play with them
use bevy::prelude::*;
use super::r#box::Box as Box;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Debug, Copy, Clone)]
pub struct Cube {
    pub size: f32,
}

impl Cube {
    pub fn new(size: f32) -> Cube {
        Cube { size }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube { size: 1.0 }
    }
}

impl From<Cube> for Mesh {
    fn from(cube: Cube) -> Self {
        Box::new(cube.size, cube.size, cube.size).into()
    }
}


