use bevy::prelude::*;
use bevy_inspector_egui::*;

pub struct TertianPlugin;
impl Plugin for TertianPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TertianAssets>()
        .add_plugin(InspectorPlugin::<TertianAssets>::new().open(false));
    }
}

#[derive(Bundle)]
pub struct TertianBundle {
    #[bundle]
    pbr: PbrBundle,
}

impl TertianBundle {
    pub fn new(pos: Vec3, assets: &TertianAssets) -> Self {
        Self {
            pbr: PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_translation(pos).looking_at(pos - Vec3::Y,  Vec3::Z),
                ..Default::default()
            },
        }
    }
}

#[derive(Inspectable)]
pub struct TertianAssets {
    #[inspectable(label= "Size XY", min = Vec2::ZERO, max = Vec2::splat(100.0), speed = 1.0 )]
    pub size: Vec2,
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
        let size =Vec2::new(50.0, 50.0);
        Self {
            size: size,
            material: material,
            mesh: meshes.add(Mesh::from(shape::Quad { size: size, flip: false })),
        }
    }
}
