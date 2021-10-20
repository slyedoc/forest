use crate::prelude::*;
use bevy::{math::*, prelude::*};
use bevy_dolly::prelude::*;

/// Template Placeholder
pub struct LSystemPlugin;
impl Plugin for LSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::LSystemTest)
                .with_system(setup)
                .with_system(light_default_setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::LSystemTest).with_system(back_to_menu_system),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::LSystemTest).with_system(clear_system),
        );
    }
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn_bundle(DollyControlCameraBundle {
            rig: Rig::default()
                .add(RigPosition::default())
                .add(Rotation::default())
                .add(Smooth::new(1.0, 1.0)),
            transform: Transform::from_xyz(-5.0, 5.0, -5.0)
                .looking_at(vec3(2.0, 2.0, 5.0), Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"));

    // LSystems
    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::DragonCurve {
                iterations: 10,
                size: 1.0,
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("DragonCurve"));

    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::FractalPlant {
                iterations: 6,
                size: 1.0,
            },
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("FractalPlant"));

    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::Abop19 {
                r: 1.46,
                iterations: 8,
                size: 1.0,
            },
            transform: Transform::from_xyz(4.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Abop19"));

    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::KochCurve {
                iterations: 4,
                size: 1.4,
            },
            // TODO: need to fix offset in KochCurve
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("KochCurve"));

    commands
        .spawn_bundle(ProceduralBundle {
            data: procedural::Sierpinski {
                iterations: 5,
                size: 1.0,
            },
            transform: Transform::from_xyz(8.0, 1.0, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Sierpinski"));
}
