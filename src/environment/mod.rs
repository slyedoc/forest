use crate::prelude::AppInspector;
use bevy::{pbr::AmbientLight, prelude::*};
use bevy_inspector_egui::Inspectable;
use bevy_shadows::prelude::*;

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_inspector_resource::<EnvironmentConfig>()
            .add_plugin(ShadowPlugin::default())
            .add_startup_system(light_setup)
            .add_system(light_config_system)
            .add_system(light_direction);
    }
}

#[derive(Inspectable)]
pub struct EnvironmentConfig {
    pub color: Color,
    pub illuminance: f32,
    pub light_direction: Vec3,
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            light_direction: Vec3::new(1.0, -1.0, 0.0),
            color: Color::WHITE,
            illuminance: 32000.0,
        }
    }
}

fn light_setup(mut commands: Commands, config: Res<EnvironmentConfig>, mut ambient_light: ResMut<AmbientLight>) {

    ambient_light.brightness = 0.01;
    
    let half_size = 5.0;
    info!("creating sun");
    commands
        .spawn()
        .insert(DirectionalLight::new(
            config.color,
            config.illuminance,
            config.light_direction,
        ))
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .insert(ShadowDirectionalLight {
            left: -half_size,
            right: half_size,
            bottom: -half_size,
            top: half_size,
            ..Default::default()
        });
}

fn light_config_system(_time: Res<Time>, config: Res<EnvironmentConfig>,  mut query: Query<&mut DirectionalLight>) {
    if config.is_changed() {
        for mut light in query.iter_mut() {
            // let theta = std::f32::consts::TAU * time.seconds_since_startup() as f32 / 10.0;
            // let (s, c) = theta.sin_cos();
            // light.set_direction(Vec3::new(c, -1.0, s));
            light.illuminance = config.illuminance;
            light.color = config.color;
            light.set_direction(config.light_direction);
        }
    }
}



#[allow(dead_code)] // rotator for testing
fn light_direction(time: Res<Time>, mut query: Query<&mut DirectionalLight>) {
    for mut light in query.iter_mut() {
        let theta = std::f32::consts::TAU * time.seconds_since_startup() as f32 / 10.0;
        let (s, c) = theta.sin_cos();
        light.set_direction(Vec3::new(c, -1.0, s));
    }
}
