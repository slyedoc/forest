use bevy::{pbr::AmbientLight, prelude::*};

pub fn light_default_setup(
    mut commands: Commands,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.brightness = 0.05;

    commands.spawn_bundle( PointLightBundle {
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 100000.0,
            range: 1000.0,
            radius: 0.0,
        },
        transform: Transform::from_xyz(50.0, 100.0, 50.0 ),
        ..Default::default()
    }).insert(Name::new("Default Light"));
}