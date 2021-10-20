use crate::helper::*;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub struct StylePlugin;
impl Plugin for StylePlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<ButtonAssets>();
    }
}

#[derive(Inspectable)]
pub struct ButtonAssets {
    pub fira_sans: Handle<Font>,
    pub fira_mono: Handle<Font>,

    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonAssets {
    fn from_world(world: &mut World) -> Self {

        let (fira_sans, fira_mono) = world.resource_scope( |_world: &mut World, asset_server: Mut<AssetServer> | {
            (asset_server.load("fonts/FiraSans-Bold.ttf"),
            asset_server.load("fonts/FiraMono-Medium.ttf"))
        });
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        ButtonAssets {
            fira_sans,
            fira_mono,
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
