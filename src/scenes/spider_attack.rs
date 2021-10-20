use std::{
    f32::consts::PI,
    ops::{Deref, DerefMut},
};

use bevy::{math::*, prelude::*};
use bevy_dolly::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::prelude::*;

pub struct SpiderAttackPlugin;
impl Plugin for SpiderAttackPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::SpiderAttack)
                .with_system(setup)
                .with_system(light_default_setup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::SpiderAttack)
                .with_system(back_to_menu_system)
                .with_system(spider_update_system)
                .with_system(spider_flip_hack_system)
                .with_system(spider_mode_update_system)
                .with_system(spider_update_wonder_target_system),
        )
        .add_system_set(SystemSet::on_exit(AppState::SpiderAttack).with_system(clear_system));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn();

    commands
        .spawn_bundle(SpiderAssetBundle {
            spider: SpiderType::Default,
            transform: Transform {
                translation: vec3(0.0, 0.1, 0.0),
                scale: Vec3::splat(0.01),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ModeTimer(Timer::from_seconds(5.0, true)))
        .insert(Spider::default())
        .insert(Name::new("Spider"));

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //         material: materials.add(Color::GRAY.into()),
    //         transform: Transform::from_xyz(3.0, 0.5, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(Name::new("Cube"));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
            material: materials.add(Color::rgb_linear(0.0, 0.1, 0.0).into()),
            transform: Transform::default(),
            ..Default::default()
        })
        .insert(Name::new("Ground"));

    // Camera
    commands
        .spawn_bundle(DollyControlCameraBundle {
            rig: Rig::default()
                .add(RigPosition::default())
                .add(Rotation::default())
                .add(Smooth::new(1.0, 1.0)),
            transform: Transform::from_xyz(0.0, 2.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Name::new("Camera"))
        .insert(Player);
}

#[derive(Component, Inspectable, Debug)]
pub struct Spider {
    mode: SpiderMode,
    wonder_target: Vec3,
}

#[derive(Debug, EnumIter, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum SpiderMode {
    #[default]
    Wonder,
    Chase,
    Jump,
}

#[derive(Component)]
pub struct Player;

impl Default for Spider {
    fn default() -> Self {
        let mut result = Self {
            mode: SpiderMode::Chase,
            wonder_target: Vec3::ZERO,
        };
        result.random_target();
        result
    }
}

impl Spider {
    pub fn next_mode(&mut self) {
        let mut rand = thread_rng();
        let mut modes = SpiderMode::iter().collect::<Vec<SpiderMode>>();
        modes.shuffle(&mut rand);

        while let Some(mode) = modes.pop() {
            if mode != self.mode {
                self.mode = mode;
            }
        }
    }

    pub fn random_target(&mut self) {
        let mut rand = thread_rng();
        let range = -30.0..30.0;
        self.wonder_target = vec3(rand.gen_range(range.clone()), 0.0, rand.gen_range(range));
    }
}


pub fn spider_update_system(
    mut query: QuerySet<(
        QueryState<&Transform, With<Player>>,
        QueryState<(&mut Transform, &Spider)>,
    )>,
) {
    let player = query.q0().get_single().unwrap().translation;
    for (mut transform, spider) in query.q1().iter_mut() {
        match spider.mode {
            SpiderMode::Wonder => {
                transform.look_at(spider.wonder_target, Vec3::Y);
                let forward = transform.forward(); // again its backward
                transform.translation += forward * 0.01;
            }
            SpiderMode::Chase => {
                // model is backwards, fliping with negatives
                transform.look_at(vec3(player.x, 0.0, player.z), Vec3::Y);
                let forward = transform.forward(); // again its backward
                transform.translation += forward * 0.01;
            }
            SpiderMode::Jump => {
                // TODO: right now just runs faster
                transform.look_at(vec3(player.x, 0.0, player.z), Vec3::Y);
                let forward = transform.forward(); // again its backward
                transform.translation += forward * 0.03;
            }
        }
    }
}

//TODO: Spider model is all jacked up, rotating child when created as a hack fix
pub fn spider_flip_hack_system(
    query: Query<&Children, (With<GenerateBounding>, With<Spider>)>,
    mut transform_query: Query<&mut Transform>,
) {
    for children in query.iter() {
        for c in children.iter() {
            if let Ok(mut t) = transform_query.get_mut(*c) {
                t.rotate(Quat::from_rotation_y(PI));
            }
        }
    }
}

pub fn spider_update_wonder_target_system(mut query: Query<(&mut Spider, &Transform)>) {
    for (mut spider, transform) in query.iter_mut() {
        let distance_squared = transform.translation.distance_squared(spider.wonder_target);
        if distance_squared < 1.0 {
            spider.random_target();
        }
    }
}

fn spider_mode_update_system(time: Res<Time>, mut query: Query<(&mut Spider, &mut ModeTimer)>) {
    for (mut spider, mut timer) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            spider.next_mode();
            info!("change spider mode: {:?}", spider.mode);
        }
    }
}

#[derive(Component)]
pub struct ModeTimer(Timer);
impl Deref for ModeTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ModeTimer {
    fn deref_mut(&mut self) -> &mut Timer {
        &mut self.0
    }
}
