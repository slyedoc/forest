use bevy::prelude::*;
use bevy_inspector_egui::*;
use strum_macros::EnumIter;

use super::GltfAssetType;
use super::init_asset_type_system;

pub struct SpiderPlugin;
impl Plugin for SpiderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_asset_type_system::<SpiderType>);
        // registering custom component to be able to edit it in inspector
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();
        registry.register::<SpiderType>();
    }
}

#[derive(Bundle, Default)]
pub struct SpiderAssetBundle {
    pub spider: SpiderType,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component, Default, EnumIter, PartialEq, Debug, Inspectable, Copy, Clone)]
pub enum SpiderType {
    #[default]
    Default,
}

impl GltfAssetType for SpiderType {
    fn get_path(&self) -> &str {
        match self {
            SpiderType::Default => "spider/scene.gltf#Scene0"
        }
    }
}