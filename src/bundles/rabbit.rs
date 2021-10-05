use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;

pub struct RabbitPlugin;
impl Plugin for RabbitPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<RabbitAssets>()
            .add_system(move_system);
    }
}

fn move_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Rabbit>>,
    config: Res<RabbitAssets>,
) {
    for mut transform in query.iter_mut() {
        // Add simple bounce for now
        transform.translation.y = (time.seconds_since_startup() as f32 + transform.translation.x)
            .cos()
            .abs()
            + config.size_half;
    }
}

#[derive(Inspectable)]
pub struct RabbitAssets {
    pub size_half: f32,
    #[inspectable()]
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl FromWorld for RabbitAssets {
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
        let size = 0.5;
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh_handle = meshes.add(Mesh::from(shape::Cube { size }));

        Self {
            size_half: size,
            material: material_handle,
            mesh: mesh_handle,
        }
    }
}

#[derive(Component, Default)]
pub struct Rabbit;

#[derive(Default, Bundle)]
pub struct RabbitBundle {
    rabbit: Rabbit,
    #[bundle]
    pbr: PbrBundle,
}

impl RabbitBundle {
    pub fn new(pos: Vec3, assets: &RabbitAssets) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_translation(pos),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
