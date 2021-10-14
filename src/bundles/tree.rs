use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

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

