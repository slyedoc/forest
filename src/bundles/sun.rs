use bevy::prelude::*;

#[derive(Bundle)]
pub struct SunBundle {
    #[bundle]
    point_light: PointLightBundle,
}

impl SunBundle {
    pub fn new() -> Self {
        Self {
            // Light
            point_light: PointLightBundle {
                point_light: PointLight {
                    color: Color::WHITE,
                    intensity: 10_000_000.0,
                    range: f32::MAX,
                    radius: 10_000.0,
                },
                transform: Transform::from_xyz(0.0, 1000.0, 1000.0),
                ..Default::default()
            },
        }
    }
}
