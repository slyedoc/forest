/// Utility Class to persit Window information
///
/// Note: Really this is to speed up dev time
/// I sick of my own app covering up vscode
///
/// There are lots issues with this implementation
/// - cross platform and wasm
/// - fires to much during resize
///
use super::PathConfig;
use bevy::{
    prelude::*,
    window::{WindowCreated, WindowResized},
};
use ron::ser::PrettyConfig;
use std::fs::File;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct WindowConfig {
    width: f32,
    height: f32,
    position: IVec2,
}
impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            height: 800.0,
            width: 1200.0,
            position: IVec2::new(0, 0),
        }
    }
}

/// Provides Window Position and Size Retention
pub struct WindowConfigPlugin;
impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(change_detection_system)
            .add_system(delay_hack_system);

        // TODO: If we just use setup to set window properties, it works fine the
        // left ~75% of my ultrawide monitor, anything to the right of that will get
        // moved left, I can hard code a system to set it and it will work after a frame or two

        // For now just using WindowCreation event to trigger the load_and_set later
        // Missing something here and tired of messing with it
    }
}

fn setup(windows: ResMut<Windows>, paths: Res<PathConfig>) {
    load_and_set(paths, windows);
}

fn delay_hack_system(
    mut ev: EventReader<WindowCreated>,
    windows: ResMut<Windows>,
    paths: Res<PathConfig>,
) {
    if ev.iter().count() > 0 {
        load_and_set(paths, windows);
    }
}

fn load_and_set(paths: Res<PathConfig>, mut windows: ResMut<Windows>) {
    let config = match File::open(paths.window_config.clone()) {
        Ok(x) => match ron::de::from_reader(x) {
            Ok(x) => x,
            Err(_) => {
                warn!("Couldn't parse {}", paths.window_config);
                WindowConfig::default()
            }
        },
        Err(_) => {
            warn!("Couldn't open {}", paths.window_config);
            WindowConfig::default()
        }
    };
    let window = windows.get_primary_mut().unwrap();
    window.set_position(config.position);
    window.set_resolution(config.width, config.height);
}

fn change_detection_system(
    mut move_events: EventReader<WindowMoved>,
    mut resize_events: EventReader<WindowResized>,
    mut windows: ResMut<Windows>,
    paths: Res<PathConfig>,
) {
    if move_events.iter().count() > 0 || resize_events.iter().count() > 0 {
        // TODO: Runs to much, every frame while resizing
        let window = windows.get_primary_mut().unwrap();
        let value = WindowConfig {
            width: window.width(),
            height: window.height(),
            position: window.position().unwrap(),
        };

        let pretty = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);

        let f = File::create(&paths.window_config).expect("Failed window_config file!");

        // TODO: handle this better, whats everyone else doing in bevy
        match ron::ser::to_writer_pretty(f, &value, pretty) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to write config: {}", e);
            }
        };
    }
}
