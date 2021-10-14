use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use super::linden_mayer::*;
use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct FractalPlant {
    pub iterations: usize,
    pub size: f32,
}
impl Default for FractalPlant {
    fn default() -> Self {
        Self {
            iterations: 3,
            size: 1.0,
        }
    }
}

impl From<FractalPlant> for Mesh {
    fn from(data: FractalPlant) -> Self {
        let axiom = symstr("X");

        let mut system = LMSystem::new();
        system.add_rule(rule('X', "F-[[X]+X]+F[+FX]-X"));
        system.add_rule(rule('F', "FF"));

        let (after, _iters) = system.develop(axiom, data.iterations);
        let t = build(&after, 0.0, 25.0, 10.0);

        generate_mesh(t, data.size)
    }
}
