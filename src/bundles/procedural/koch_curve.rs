use crate::{helper::*, linden_mayer::*};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::generate_mesh;

pub struct KochCurvePlugin;
impl Plugin for KochCurvePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        
    }
}

#[derive(Inspectable, Component, Debug, Copy, Clone)]
pub struct KochCurve {
    pub iterations: usize,
}
impl Default for KochCurve {
    fn default() -> Self {
        Self { iterations: 5 }
    }
}

impl From<KochCurve> for Mesh {
    fn from(data: KochCurve) -> Self {

        let axiom = symstr("F++F++F");

        let mut system = LMSystem::new();
        system.add_rule(rule('F', "F-F++F-F"));
        println!("{:?}", system);

        let (after, _iters) = system.develop(axiom, data.iterations);

        //draw(&after, 90.0, 60.0, 10.0, &format!("koch_{:02}", iters));
        let t = build(&after, 90.0, 60.0, 10.0);

        generate_mesh(t, 10.0)
    }
}

#[derive(Inspectable)]
pub struct KochCurveAssets {
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for KochCurveAssets {
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
