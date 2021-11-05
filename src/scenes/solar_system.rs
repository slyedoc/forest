use crate::prelude::*;
use bevy::{core::FixedTimestep, math::*, prelude::*, render::camera::Camera};
use bevy_dolly::{DollyActions, prelude::Rig};
use rand::{thread_rng, Rng};

pub struct SolarSystemPlugin;
impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SolarSystem)
                .with_system(setup)
                //.with_system(light_default_setup)
                .with_system(generate_bodies)
                .with_system(generate_background),
        )
        .add_system_set(
            SystemSet::on_update(AppState::SolarSystem)
                .with_run_criteria(FixedTimestep::step(DELTA_TIME))
                .with_system(back_to_menu_system)
                .with_system(interact_bodies.label("interact"))
                .with_system(integrate.after("interact")),
        )
        .add_system_set(SystemSet::on_exit(AppState::SolarSystem).with_system(clear_system));
    }
}

fn setup(mut commands: Commands) {
    // create some orbital bodies that
    commands
        .spawn_bundle(SpaceAssetBundle {
            space_type: SpaceType::Craft(Craft::SpeederA),
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..Default::default()
        })
        .insert(Name::new("Ship"))
        .id();

    // Camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            camera: Camera {
                name: Some("Camera3d".to_string()),
                ..Default::default()
            },
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Rig::default())
        .insert(DollyActions::default())
        .insert(Name::new("Player"))
        .id();
}

const DELTA_TIME: f64 = 0.01;
const GRAVITY_CONSTANT: f32 = 0.001;
const SOFTENING: f32 = 0.01;
const NUM_BODIES: usize = 10;

#[derive(Component, Default)]
struct Mass(f32);
#[derive(Component, Default)]
struct Acceleration(Vec3);
#[derive(Component, Default)]
struct LastPos(Vec3);

#[derive(Bundle, Default)]
struct BodyBundle {
    #[bundle]
    pbr: PbrBundle,
    mass: Mass,
    last_pos: LastPos,
    acceleration: Acceleration,
}

fn generate_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 3,
    }));

    let pos_range = 50.0..100.0;
    let color_range = 0.5..1.0;
    let vel_range = -0.5..0.5;

    let bodies = commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Solar System"))
        .id();

    let mut rng = thread_rng();
    for i in 0..NUM_BODIES {
        let mass_value_cube_root: f32 = rng.gen_range(0.4..6.0);
        let mass_value: f32 = mass_value_cube_root * mass_value_cube_root * mass_value_cube_root;

        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-0.01..0.01),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * rng.gen_range(pos_range.clone());

        commands.entity(bodies).with_children(|parent| {
            parent
                .spawn_bundle(BodyBundle {
                    pbr: PbrBundle {
                        transform: Transform {
                            translation: position,
                            scale: Vec3::splat(mass_value_cube_root * 0.5),
                            ..Default::default()
                        },
                        mesh: mesh.clone(),
                        material: materials.add(
                            Color::rgb_linear(
                                rng.gen_range(color_range.clone()),
                                rng.gen_range(color_range.clone()),
                                rng.gen_range(color_range.clone()),
                            )
                            .into(),
                        ),
                        ..Default::default()
                    },
                    mass: Mass(mass_value),
                    acceleration: Acceleration(Vec3::ZERO),
                    last_pos: LastPos(
                        position
                            - Vec3::new(
                                rng.gen_range(vel_range.clone()),
                                rng.gen_range(vel_range.clone()),
                                rng.gen_range(vel_range.clone()),
                            ) * DELTA_TIME as f32,
                    ),
                })
                .insert(Name::new(format!("Body{}", i)));
        });
    }

    // add bigger "star" body in the center

    commands.entity(bodies).with_children(|parent| {
        parent
            .spawn_bundle(BodyBundle {
                pbr: PbrBundle {
                    transform: Transform {
                        scale: Vec3::splat(3.0),
                        ..Default::default()
                    },
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 1.0,
                        subdivisions: 5,
                    })),
                    material: materials.add((Color::ORANGE_RED * 10.0).into()),
                    ..Default::default()
                },
                mass: Mass(1000.0),
                ..Default::default()
            })
            .insert(PointLight {
                intensity: 100000.0,
                range: 1000.0,
                color: Color::ORANGE_RED,
                ..Default::default()
            })
            .insert(Name::new("Star"));
    });
}

fn interact_bodies(mut query: Query<(&Mass, &GlobalTransform, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(Mass(m1), transform1, mut acc1), (Mass(m2), transform2, mut acc2)]) =
        iter.fetch_next()
    {
        let delta = transform2.translation - transform1.translation;
        let distance_sq: f32 = delta.length_squared();

        let f = GRAVITY_CONSTANT / (distance_sq * (distance_sq + SOFTENING).sqrt());
        let force_unit_mass = delta * f;
        acc1.0 += force_unit_mass * *m2;
        acc2.0 -= force_unit_mass * *m1;
    }
}

// TODO: something is wrong with system order, half the time it blows up
#[allow(dead_code)]
fn integrate(mut query: Query<(&mut Acceleration, &mut Transform, &mut LastPos)>) {
    let dt_sq = (DELTA_TIME * DELTA_TIME) as f32;
    for (mut acceleration, mut transform, mut last_pos) in query.iter_mut() {
        // verlet integration
        // x(t+dt) = 2x(t) - x(t-dt) + a(t)dt^2 + O(dt^4)
        let new_pos =
            transform.translation + transform.translation - last_pos.0 + acceleration.0 * dt_sq;
        acceleration.0 = Vec3::ZERO;
        last_pos.0 = transform.translation;
        transform.translation = new_pos;
    }
}

const NUM_BACKGROUND: usize = 500;
const BACKGROUND_RANGE: f32 = 400.0;

fn generate_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Quad {
        size: vec2(1.0, 1.0),
        flip: true,
    }));

    let background = commands
        .spawn_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Background"))
        .id();

    let color_range = 0.5..1.0;

    let mut rng = thread_rng();

    for _ in 0..NUM_BACKGROUND {
        let mass_value_cube_root: f32 = rng.gen_range(0.2..1.0);

        let position = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
        .normalize()
            * BACKGROUND_RANGE;

        commands.entity(background).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                transform: Transform {
                    translation: position,
                    scale: Vec3::splat(mass_value_cube_root),
                    ..Default::default()
                }
                .looking_at(Vec3::ZERO, Vec3::Y),
                mesh: mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb_linear(
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone()),
                        rng.gen_range(color_range.clone()),
                    ),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
    }
}
