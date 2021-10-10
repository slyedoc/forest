use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;

pub struct TertianPlugin;
impl Plugin for TertianPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<TertianAssets>()
            .add_system_to_stage(CoreStage::PreUpdate, init_terian_system);
    }
}

#[derive(Default, Component)]
pub struct Tertain;

#[derive(Default, Bundle, Component)]
pub struct TertianBundle {
    tertain: Tertain,
    #[bundle]
    pbr: PbrBundle,
}

pub fn init_terian_system(
    mut query: Query<(&mut Handle<Mesh>, &mut Handle<StandardMaterial>), Added<Tertain>>,
    assets: ResMut<TertianAssets>,
) {
    for (mut mesh, mut material) in query.iter_mut() {
        warn!("setting");
        *mesh = assets.mesh.clone();
        *material = assets.material.clone();
    }
}

#[derive(Inspectable)]
pub struct TertianAssets {
    #[inspectable(label = "Size", min = 0.0, max = 100.0, speed = 10.0)]
    pub size: f32,
    #[inspectable(ignore)]
    pub mesh: Handle<Mesh>,

    pub material: Handle<StandardMaterial>,
}

impl FromWorld for TertianAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.1, 0.7, 0.1),
            reflectance: 0.02,
            roughness: 1.0,
            unlit: false,
            ..Default::default()
        });

        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let size = 50.0;
        Self {
            size,
            material,
            mesh: meshes.add(Mesh::from(shape::Plane { size })),
        }
    }
}
