use crate::{helper::*, linden_mayer::*};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::DebugLines;


use super::DrawLines;

pub struct DragonCurvePlugin;
impl Plugin for DragonCurvePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_inspector_resource::<DragonCurveAssets>();
        //.add_system(generate_system)
    }
}

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct DragonCurve {
    pub iterations: usize,
}
impl Default for DragonCurve {
    fn default() -> Self {
        Self { iterations: 3 }
    }
}

impl DrawLines for DragonCurve {
    fn draw_lines(&self, lines: &mut DebugLines) {
        let axiom = symstr("FX");

        let mut system = LMSystem::new();
        system.add_rule(rule('X', "X+YF+"));
        system.add_rule(rule('Y', "-FX-Y"));
        println!("{:?}", system);

        let (symstr, _iters) = system.develop(axiom, self.iterations);

        draw_l_system(&symstr, 0.0, 90.0, 10.0, lines);
    }
}

impl From<DragonCurve> for Mesh {
    fn from(_data: DragonCurve) -> Self {

        // TODO: starting with line render, will be back
        Mesh::from(shape::Box {
            ..Default::default()
        })
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
