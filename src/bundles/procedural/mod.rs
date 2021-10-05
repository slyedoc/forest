mod r#box;
mod cube;
mod dragon_curve;
mod plane;
mod quad;
use bevy::{pbr::render_graph::PBR_PIPELINE_HANDLE, prelude::*, render::{pipeline::RenderPipeline, render_graph::base::MainPass}};
use bevy_inspector_egui::{Inspectable, InspectableRegistry};
use bevy_prototype_debug_lines::DebugLines;
pub use {r#box::r#Box, cube::Cube, dragon_curve::DragonCurve, plane::Plane, quad::Quad};
use std::fmt::Debug;

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
            .with_system(line_detection_system::<DragonCurve>)
            .with_system(mesh_detection_system::<Plane>)
            .with_system(mesh_detection_system::<Quad>)
        );

        // getting registry from world
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        // registering custom component to be able to edit it in inspector
        registry.register::<Box>();
        registry.register::<Cube>();
        
        registry.register::<Plane>();
        registry.register::<Quad>();

        registry.register::<DragonCurve>();
    }
}


fn mesh_detection_system<T: Component + Into<Mesh>>(
    mut commands: Commands,
    mut query: Query<(Entity, &T), Or<(Added<T>, Changed<T>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) where T: Component + Copy + Debug + Inspectable, Mesh: From<T>,
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

fn line_detection_system<T: Component + DrawLines>(
    mut query: Query<(Entity, &T), Or<(Added<T>, Changed<T>)>>,
    mut lines: ResMut<DebugLines>
) where T: Component + Copy + Debug + Inspectable + DrawLines,
{
    for (_e, data) in query.iter_mut() {
        info!("{:?}", data);

        data.draw_lines(&mut lines);
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

trait DrawLines {
    fn draw_lines(&self, lines: &mut DebugLines);
}