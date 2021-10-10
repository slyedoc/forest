mod r#box;
mod cube;
mod dragon_curve;
mod koch_curve;
mod plane;
mod quad;
use bevy::{
    pbr::render_graph::PBR_PIPELINE_HANDLE,
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
};
use bevy_inspector_egui::{Inspectable, InspectableRegistry};
use std::fmt::Debug;

use crate::{linden_mayer::turtle, prelude::AppInspector};


pub use {
    cube::*,
    dragon_curve::*,
    plane::*,
    quad::*,
    r#box::*,
    koch_curve::*,
};

/// Generate Meshes on the fly
/// TODO: Work in progress
pub struct ProceduralPlugin;
impl Plugin for ProceduralPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<KochCurveAssets>()
            .init_inspector_resource::<DragonCurveAssets>()

            .add_system_set(
            SystemSet::new()
                // TODO: any way to do with with one system?
                .with_system(mesh_detection_system::<Box>)
                .with_system(mesh_detection_system::<Cube>)
                .with_system(mesh_detection_system::<DragonCurve>)
                .with_system(mesh_detection_system::<KochCurve>)
                .with_system(mesh_detection_system::<Plane>)
                .with_system(mesh_detection_system::<Quad>),

        );

        // getting registry from world
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        // registering custom component to be able to edit it in inspector
        registry.register::<Box>();
        registry.register::<Cube>();
        registry.register::<Plane>();
        registry.register::<Quad>();
        registry.register::<DragonCurve>();
        registry.register::<KochCurve>();
    }
}

fn mesh_detection_system<T: Component + Into<Mesh>>(
    mut commands: Commands,
    mut query: Query<(Entity, &T), Or<(Added<T>, Changed<T>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) where
    T: Component + Copy + Debug + Inspectable,
    Mesh: From<T>,
{
    for (e, data) in query.iter_mut() {
        commands.entity(e).insert_bundle(ProceduralPbrBundle {
            mesh: meshes.add(Mesh::from(*data)),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..Default::default()
            }),
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


pub fn generate_mesh(t: turtle::Canvas, size: f32) -> Mesh {


    let lines = t.draw_lines(size);

    for line in lines.iter().take(5) {
        println!("{} {}", line.0 , line.1);
    }

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for (v1, v2) in lines.iter() {
        positions.push([v1.x, v1.y, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([1.0, 1.0]);

        positions.push([v2.x, v2.y, 0.0]);
        normals.push([0.0, 1.0, 0.0]);
        uvs.push([1.0, 1.0]);
    }
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}