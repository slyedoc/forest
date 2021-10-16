use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct StylePlugin;
impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<ButtonAssets>()
            .init_inspector_resource::<FontAssets>();
    }
}

#[derive(Inspectable)]
pub struct FontAssets {
    pub fira_sans: Handle<Font>,
    pub fira_mono: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Self {
            fira_sans: asset_server.load("fonts/FiraSans-Bold.ttf"),
            fira_mono: asset_server.load("fonts/FiraMono-Medium.ttf"),
        }
    }
}

#[derive(Inspectable)]
pub struct ButtonAssets {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonAssets {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
