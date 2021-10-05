use bevy_inspector_egui::Inspectable;
use std::env::var;

#[derive(Inspectable)]
pub struct PathConfig {
    #[allow(dead_code)]
    #[inspectable(multiline)]
    pub config_home: String,
    #[inspectable(multiline)]
    pub window_config: String,
}

const CONFIG_DIR: &str = "forests";
impl Default for PathConfig {
    fn default() -> Self {
        // Find our config home
        let config_home = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{}/.config/{}", home, CONFIG_DIR)))
            .unwrap();

        // Create if it doesn't exists
        let _ = std::fs::create_dir_all(&config_home);

        Self {
            window_config: format!("{}/window_config.ron", config_home),
            config_home,
        }
    }
}
