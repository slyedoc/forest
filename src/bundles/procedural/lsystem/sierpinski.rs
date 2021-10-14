use super::linden_mayer::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct Sierpinski {
    pub iterations: usize,
    pub size: f32,
}
impl Default for Sierpinski {
    fn default() -> Self {
        Self {
            iterations: 9,
            size: 1.0,
        }
    }
}

impl From<Sierpinski> for Mesh {
    fn from(data: Sierpinski) -> Self {
        let axiom = symstr("A");

        let mut system = LMSystem::new();
        system.add_rule(rule('A', "+B-A-B+"));
        system.add_rule(rule('B', "-A+B+A-"));

        let (after, _iters) = system.develop(axiom, data.iterations);

        // replace A and B with F
        let mut system = LMSystem::new();
        system.add_rule(rule('A', "F"));
        system.add_rule(rule('B', "F"));
        let (after, _iters) = system.develop_next(&after);

        let t = build(&after, -90.0, 60.0, 10.0);

        generate_mesh(t, data.size)
    }
}
