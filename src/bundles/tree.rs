use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct TreePlugin;
impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeAssets>();
    }
}

#[derive(Bundle)]
pub struct TreeBundle {
    config: TreeConfig,

    #[bundle]
    pbr: PbrBundle,
}

impl TreeBundle {
    pub fn new(
        pos: Vec3,
        assets: &TreeAssets,
    ) -> TreeBundle {

        let pbr = PbrBundle {
            transform: Transform::from_translation(pos),
            mesh: assets.mesh.clone(),
            material: assets.material.clone(),
            ..Default::default()
        };

        Self {
            config: TreeConfig::default(),
            pbr: pbr,
        }
    }
}

#[derive(Inspectable)]
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

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            // clumpMax:0.8,
            // clumpMin:0.5,
            // lengthFalloffFactor:0.85,
            // lengthFalloffPower:1.0,
            // branchFactor:2.0,
            // radiusFalloffRate:0.6,
            // climbRate:1.5,
            // trunkKink:0.00,
            // maxRadius:0.25,
            // treeSteps:2,
            // taperRate:0.95,
            // twistRate:13,
            // segments:6,
            // levels:3,
            // sweepAmount:0.0,
            // initalBranchLength:0.85,
            // trunkLength:2.5,
            // dropAmount: 0.0,
            // growAmount: 0.0,
            // vMultiplier:0.2,
            // twigScale:2.0,
         }
    }
}



pub struct TreeAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TreeAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.7, 0.1),
            reflectance: 0.02,
            roughness: 1.0,
            unlit: false,
            ..Default::default()
        });

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        Self {
            material: material,
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        }
    }
}
