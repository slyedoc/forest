mod bundles;
mod editor;
mod helper;
mod states;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bundles::*;
use editor::EditorPlugin;
use states::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    Forest,
    Game,
}

impl AppState {
    fn name(&self) -> String {
        match self {
            Self::Menu => "Menu".to_string(),
            Self::Forest => "Forest".to_string(),
            Self::Game => "Game".to_string(),
        }
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .insert_resource(WindowDescriptor {
            title: "Forests".to_string(),
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(PickingPlugin)
        // Our plugins
        .add_plugin(EditorPlugin)
        .add_plugins(BundlePlugins)
        
        // AppState Plugins
        .add_state(AppState::Menu)
        .add_plugin(MenuPlugin)
        .add_plugin(ForestPlugin)
        .add_plugin(GamePlugin)
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
