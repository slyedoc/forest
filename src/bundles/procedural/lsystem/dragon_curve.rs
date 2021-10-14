use super::linden_mayer::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct DragonCurve {
    pub iterations: usize,
    pub size: f32,
}
impl Default for DragonCurve {
    fn default() -> Self {
        Self {
            iterations: 3,
            size: 1.0,
        }
    }
}

impl From<DragonCurve> for Mesh {
    fn from(data: DragonCurve) -> Self {
        let axiom = symstr("FX");

        let mut system = LMSystem::new();
        system.add_rule(rule('X', "X+YF+"));
        system.add_rule(rule('Y', "-FX-Y"));

        let (symstr, _iters) = system.develop(axiom, data.iterations);
        let t = build(&symstr, 0.0, 90.0, 10.0);

        generate_mesh(t, data.size)
    }
}
