use super::linden_mayer::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct KochCurve {
    pub iterations: usize,
    pub size: f32,
}
impl Default for KochCurve {
    fn default() -> Self {
        Self {
            iterations: 5,
            size: 1.0,
        }
    }
}

impl From<KochCurve> for Mesh {
    fn from(data: KochCurve) -> Self {
        let axiom = symstr("F++F++F");

        let mut system = LMSystem::new();
        system.add_rule(rule('F', "F-F++F-F"));

        let (after, _iters) = system.develop(axiom, data.iterations);

        //draw(&after, 90.0, 60.0, 10.0, &format!("koch_{:02}", iters));
        let t = build(&after, 90.0, 60.0, 10.0);

        generate_mesh(t, data.size)
    }
}
