use crate::{helper::*, linden_mayer::*};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct DragonCurve {
    pub iterations: usize,
}
impl Default for DragonCurve {
    fn default() -> Self {
        Self { iterations: 3 }
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

        generate_mesh(t, 10.0)

    }
}



#[derive(Inspectable)]
pub struct DragonCurveAssets {
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for DragonCurveAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();

        let material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.2, 0.15),
            reflectance: 0.02,
            roughness: 1.0,
            unlit: false,
            ..Default::default()
        });

        Self {
            material: material_handle,
        }
    }
}
