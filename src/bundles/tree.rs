use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

// TODO: Port something like https://gltf-trees.donmccurdy.com/

// planet_demo.rs
pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<TreeAssets>();
    }
}

#[derive(Inspectable)]
pub struct TreeAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TreeAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.1),
            reflectance: 0.02,
            roughness: 1.0,
            unlit: false,
            ..Default::default()
        });

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
        Self {
            material: material_handle,
            mesh: mesh_handle,
        }
    }
}

#[derive(Component, Inspectable, Default)]
pub struct TreeConfig {
    // clumpMax:f32,
// clumpMin:f32,
// lengthFalloffFactor:f32,
// lengthFalloffPower:f32,
// branchFactor:f32,
// radiusFalloffRate:f32,
// climbRate:f32,
// trunkKink:f32,
// maxRadius:f32,
// treeSteps:usize,
// taperRate:f32,
// twistRate:usize,
// segments:usize,
// levels:usize,
// sweepAmount:f32,
// initalBranchLength:f32,
// trunkLength:f32,
// dropAmount: f32,
// growAmount: f32,
// vMultiplier:f32,
// twigScale:f32,
}
