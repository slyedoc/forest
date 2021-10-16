mod circle;
mod classic;
mod lsystem;
mod plane;
mod tree;

use bevy::{
    pbr::render_graph::PBR_PIPELINE_HANDLE,
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
};
use bevy_inspector_egui::{Inspectable, InspectableRegistry};

pub use {circle::*, classic::*, lsystem::*, plane::*, tree::*};

/// Generate Meshes on the fly
/// TODO: Work in progress
pub struct ProceduralPlugin;
impl Plugin for ProceduralPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                // TODO: any way to do with with one system?
                .with_system(mesh_detection_system::<Box>)
                .with_system(mesh_detection_system::<Cube>)
                .with_system(mesh_detection_system::<Quad>)
                .with_system(mesh_detection_system::<DragonCurve>)
                .with_system(mesh_detection_system::<KochCurve>)
                .with_system(mesh_detection_system::<FractalPlant>)
                .with_system(mesh_detection_system::<Abop19>)
                .with_system(mesh_detection_system::<Sierpinski>)
                .with_system(mesh_detection_system::<Tree>)
                .with_system(mesh_detection_system::<Plane>)
                .with_system(mesh_detection_system::<Circle>),
        );

        // getting registry from world
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        // registering custom component to be able to edit it in inspector
        registry.register::<Box>();
        registry.register::<Cube>();
        registry.register::<Quad>();
        registry.register::<DragonCurve>();
        registry.register::<KochCurve>();
        registry.register::<FractalPlant>();
        registry.register::<Abop19>();
        registry.register::<Sierpinski>();
        registry.register::<Tree>();
        registry.register::<Plane>();
        registry.register::<Circle>();
    }
}

fn mesh_detection_system<T: Component + Into<Mesh>>(
    mut commands: Commands,
    mut query: Query<(Entity, &T), Or<(Added<T>, Changed<T>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) where
    T: Component + Copy + Inspectable,
    Mesh: From<T>,
{
    for (e, data) in query.iter_mut() {
        let texture_handle = asset_server.load("uv_debug.png");
        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            unlit: true,
            ..Default::default()
        });

        commands.entity(e).insert_bundle(ProceduralPbrBundle {
            mesh: meshes.add(Mesh::from(*data)),
            material: material_handle,
            ..Default::default()
        });
    }
}

#[derive(Bundle, Default)]
pub struct ProceduralBundle<T>
where
    T: Component + Into<Mesh> + Inspectable,
{
    // Adding component user can set
    pub data: T,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Bundle)]
struct ProceduralPbrBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
}

impl Default for ProceduralPbrBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                PBR_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
        }
    }
}
#[derive(Inspectable)]
pub struct UVOffset {
    pub offset: Vec2,
}

// TODO: Find better way, and using [f32;3] the entire time is not it
pub fn convert_vec3(given: &[Vec3]) -> Vec<[f32; 3]> {
    let mut result = Vec::<[f32; 3]>::new();
    for v in given.iter() {
        result.push([v.x, v.y, v.z]);
    }
    result
}

// TODO: See above
pub fn convert_vec2(given: &[Vec2]) -> Vec<[f32; 2]> {
    let mut result = Vec::<[f32; 2]>::new();
    for v in given.iter() {
        result.push([v.x, v.y]);
    }
    result
}
