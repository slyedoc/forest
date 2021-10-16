use bevy::asset::LoadState;
use bevy::gltf::Gltf;
use bevy::math::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_inspector_egui::widgets::InspectableButton;
use rand::prelude::*;
//use bevy_mod_bounding::aabb;
//use bevy_mod_bounding::debug::DebugBounds;
//use bevy_mod_bounding::Bounded;
//use bevy_mod_picking::PickableBundle;

pub struct CityPlugin;
impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CityAssets>()
            .add_event::<GridResetEventButton>()
            .add_system(rebuild_button)
            .add_system_to_stage(CoreStage::Last, update_building_system.label("building"))
            .add_system_to_stage(CoreStage::First, update_city_road_system.after("building"));

        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        // registering custom component to be able to edit it in inspector
        registry.register::<CityBuilding>();
        registry.register::<CityRoad>();
    }
}

//#[derive(Inspectable)]: Handle<Scene>,
pub struct CityAssets {
    pub detail_awning: Handle<Scene>,
    pub detail_awning_wide: Handle<Scene>,
    pub detail_overhang: Handle<Scene>,
    pub detail_overhang_wide: Handle<Scene>,
    pub detail_umbrella_detailed: Handle<Scene>,
    pub detail_umbrella: Handle<Scene>,
    pub large_building_a: Handle<Scene>,
    pub large_building_b: Handle<Scene>,
    pub large_building_c: Handle<Scene>,
    pub large_building_d: Handle<Scene>,
    pub large_building_e: Handle<Scene>,
    pub large_building_f: Handle<Scene>,
    pub large_building_g: Handle<Scene>,
    pub low_building_a: Handle<Scene>,
    pub low_building_b: Handle<Scene>,
    pub low_building_c: Handle<Scene>,
    pub low_building_d: Handle<Scene>,
    pub low_building_e: Handle<Scene>,
    pub low_building_f: Handle<Scene>,
    pub low_building_g: Handle<Scene>,
    pub low_building_h: Handle<Scene>,
    pub low_building_i: Handle<Scene>,
    pub low_building_j: Handle<Scene>,
    pub low_building_k: Handle<Scene>,
    pub low_building_l: Handle<Scene>,
    pub low_building_m: Handle<Scene>,
    pub low_building_n: Handle<Scene>,
    pub low_wide_a: Handle<Scene>,
    pub low_wide_b: Handle<Scene>,
    pub roof_center: Handle<Scene>,
    pub roof_corner: Handle<Scene>,
    pub roof_overhang: Handle<Scene>,
    pub roof_side: Handle<Scene>,
    pub sign_billboard: Handle<Scene>,
    pub sign_hospital: Handle<Scene>,
    pub skyscraper_a: Handle<Scene>,
    pub skyscraper_b: Handle<Scene>,
    pub skyscraper_c: Handle<Scene>,
    pub skyscraper_d: Handle<Scene>,
    pub skyscraper_e: Handle<Scene>,
    pub skyscraper_f: Handle<Scene>,
    pub small_building_a: Handle<Scene>,
    pub small_building_b: Handle<Scene>,
    pub small_building_c: Handle<Scene>,
    pub small_building_d: Handle<Scene>,
    pub small_building_e: Handle<Scene>,
    pub small_building_f: Handle<Scene>,
    pub wall_door_a: Handle<Scene>,
    pub wall_door_b: Handle<Scene>,
    pub wall_solid: Handle<Scene>,
    pub wall_window_a: Handle<Scene>,
    pub wall_window_b: Handle<Scene>,
    pub wall_window_c: Handle<Scene>,
    pub wall_window_d: Handle<Scene>,
    pub wall_window_e: Handle<Scene>,
    pub wall_window_f: Handle<Scene>,
}

#[derive(Bundle, Default)]
pub struct CityRoadBundle {
    pub block: CityRoad,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Bundle, Default)]
pub struct CityBuildingBundle {
    pub building: CityBuilding,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Inspectable, Component, Default)]
pub struct CityBuilding {
    pub index: Option<u32>,
    pub building_type: BuildingType,
}

#[derive(Component, Debug, Inspectable, Default, Copy, Clone)]
pub enum BuildingType {
    #[default]
    Low,
    Small,
    Large,
    Skyscraper,
}

#[derive(Default)]
pub struct GridResetEventButton;

#[derive(Component, Debug, Inspectable)]
pub struct CityRoad {
    pub start: Vec3,
    pub end: Vec3,
    #[inspectable(min = 0.01, max = 3.0)]
    pub width_half: f32,
    #[inspectable(min = 1.0, max = 10.0)]
    pub spacing: f32,
    pub building_type: BuildingType,

    #[inspectable(text = "Rebuild")]
    pub rebuild: InspectableButton<GridResetEventButton>,
}

impl Default for CityRoad {
    fn default() -> Self {
        Self {
            start: Vec3::new(-10.0, 0.0, 0.0),
            end: Vec3::new(10.0, 0.0, 0.0),
            width_half: 1.0,
            building_type: Default::default(),
            spacing: 3.0,
            rebuild: InspectableButton::<GridResetEventButton>::new(),
        }
    }
}

fn rebuild_button(
    mut commands: Commands,
    mut ev_reset: EventReader<GridResetEventButton>,
    query: Query<(Entity, &Children), (Changed<CityBuilding>, Without<Spawn>)>,
) {
    for _ in ev_reset.iter() {
        // changed and has children, delete children and add spawn
        for (_, children) in query.iter() {
            info!("delete building children, add mark spawn");
            for child_entity in children.iter() {
                commands.entity(*child_entity).insert(Spawn);
            }
        }
    }
}


fn update_building_system(
    mut commands: Commands,
    assets: Res<CityAssets>,
    asset_server: Res<AssetServer>,
    mut query: QuerySet<(
        // Spawn scene, remove spawn
        QueryState<(Entity, &mut CityBuilding), (With<Spawn>, Without<Children>)>,
        // New, add spawn
        QueryState<Entity, Added<CityBuilding>>,
    )>,
) {
    // on next frame build a building back
    for (e, mut building) in query.q0().iter_mut() {
        info!("init city building");
        commands.entity(e).remove::<Spawn>();


        let mut rng = rand::thread_rng();
        let handle = match building.building_type {

            BuildingType::Low => {
                let index = match building.index {
                    Some(i) => i.clamp(0, 10),
                    None => rng.gen_range(0u32..10u32),
                };
                building.index = Some(index); // save index for ref
                match index {
                    0 => assets.low_building_a.clone(),
                    1 => assets.low_building_b.clone(),
                    2 => assets.low_building_c.clone(),
                    3 => assets.low_building_d.clone(),
                    4 => assets.low_building_e.clone(),
                    5 => assets.low_building_f.clone(),
                    6 => assets.low_building_g.clone(),
                    7 => assets.low_building_h.clone(),
                    8 => assets.low_building_i.clone(),
                    9 => assets.low_building_j.clone(),
                    10 => assets.low_building_k.clone(),
                    11 => assets.low_building_l.clone(),
                    _ => assets.low_building_n.clone(),
                }
            }
            BuildingType::Large => {
                let index = match building.index {
                    Some(i) => i.clamp(0, 6),
                    None => rng.gen_range(0u32..6u32),
                };
                building.index = Some(index); // save index for ref
                match index {
                    0 => assets.large_building_a.clone(),
                    1 => assets.large_building_b.clone(),
                    2 => assets.large_building_c.clone(),
                    3 => assets.large_building_d.clone(),
                    4 => assets.large_building_e.clone(),
                    5 => assets.large_building_f.clone(),
                    _ => assets.large_building_g.clone(),
                }
            }
            BuildingType::Skyscraper => {
                let index = match building.index {
                    Some(i) => i.clamp(0, 5),
                    None => rng.gen_range(0u32..5u32),
                };
                building.index = Some(index); // save index for ref
                match index {
                    0 => assets.skyscraper_a.clone(),
                    1 => assets.skyscraper_b.clone(),
                    2 => assets.skyscraper_c.clone(),
                    3 => assets.skyscraper_d.clone(),
                    4 => assets.skyscraper_e.clone(),
                    _ => assets.skyscraper_f.clone(),
                }
            }
            BuildingType::Small => {
                let index = match building.index {
                    Some(i) => i.clamp(0, 5),
                    None => rng.gen_range(0u32..5u32),
                };
                building.index = Some(index); // save index for ref
                match index {
                    0 => assets.small_building_a.clone(),
                    1 => assets.small_building_b.clone(),
                    2 => assets.small_building_c.clone(),
                    3 => assets.small_building_d.clone(),
                    4 => assets.small_building_e.clone(),
                    _ => assets.small_building_f.clone(),
                }
            }
        };

        let state = asset_server.get_load_state(handle.clone());
        if state == LoadState::Loaded {
            commands.entity(e).with_children(|parent| {
                parent.spawn_scene(handle);
            });
        } else {
            warn!("city handle: {:?}", state);
        }
        info!("init building done");
    }



    // Mark for spawn, do this last, so build doesn't run till next time
    for e in query.q1().iter_mut() {
        info!("added building spawn");
        commands.entity(e).insert(Spawn);
        //commands.entity(e).insert_bundle(bevy_mod_picking::PickableBundle::default());
    }
}

#[derive(Component)]
pub struct Spawn;

fn update_city_road_system(
    mut commands: Commands,
    mut query: QuerySet<(
        QueryState<(Entity, &mut CityRoad), With<Spawn>>,
        QueryState<(Entity, &Children), (Changed<CityRoad>, Without<Spawn>)>,
        QueryState<Entity, Added<CityRoad>>,
    )>,
) {
    // remove spawn and build out children
    for (e, road) in query.q0().iter_mut() {
        commands.entity(e).remove::<Spawn>();
        info!("init city road");

        let distance = (road.end - road.start).length();

        let mut current  = 0.0;
        while current < distance {

            // Left Side
            commands.entity(e).with_children(|parent| {
                parent.spawn_bundle(CityBuildingBundle {
                    building: CityBuilding {
                        building_type: road.building_type,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(current, 0.0, road.width_half),
                    ..Default::default()
                });
            });

            // Right Side
            commands.entity(e).with_children(|parent| {
                parent.spawn_bundle(CityBuildingBundle {
                    building: CityBuilding {
                        building_type: road.building_type,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(current, 0.0, -road.width_half)
                        .looking_at(vec3(current, 0.0, 0.0), Vec3::Y ),
                    ..Default::default()
                });
            });

            current += road.spacing;
        }

        // // Set or update name
        // let name = format!("Block - {:?}", block.block_type);
        // match name_option {
        //     Some(mut n) => {
        //         n.set(name);
        //     }
        //     None => {
        //         commands.entity(e).insert(Name::new(name));
        //     }
        // }
    }

    // delete children if changed and not marked for spawn
    for (e, children) in query.q1().iter_mut() {
        for child_entity in children.iter() {
            commands.entity(*child_entity).despawn_recursive();
        }
        commands.entity(e).insert(Spawn);
    }

    // Mark for spawn
    for e in query.q2().iter_mut() {
        commands.entity(e).insert(Spawn);
    }
}

// fn spawn_gltf(mut commands: Commands, asset: Handle<Scene>) {

//     // to be able to position our 3d model:
//     // spawn a parent entity with a Transform and GlobalTransform
//     // and spawn our gltf as a scene under it
//     commands
//         .spawn_bundle((
//             Transform::from_xyz(2.0, 0.0, -5.0),
//             GlobalTransform::identity(),
//         ))
//         .with_children(|parent| {
//             parent.spawn_scene(asset);
//         });
// }

impl FromWorld for CityAssets {
    fn from_world(world: &mut World) -> Self {
        let ass = world.get_resource::<AssetServer>().unwrap();
        Self {
            detail_awning: ass.load("city/detail_awning.glb#Scene0"),
            detail_awning_wide: ass.load("city/detail_awningWide.glb#Scene0"),
            detail_overhang: ass.load("city/detail_overhang.glb#Scene0"),
            detail_overhang_wide: ass.load("city/detail_overhangWide.glb#Scene0"),
            detail_umbrella_detailed: ass.load("city/detail_umbrellaDetailed.glb#Scene0"),
            detail_umbrella: ass.load("city/detail_umbrella.glb#Scene0"),
            large_building_a: ass.load("city/large_buildingA.glb#Scene0"),
            large_building_b: ass.load("city/large_buildingB.glb#Scene0"),
            large_building_c: ass.load("city/large_buildingC.glb#Scene0"),
            large_building_d: ass.load("city/large_buildingD.glb#Scene0"),
            large_building_e: ass.load("city/large_buildingE.glb#Scene0"),
            large_building_f: ass.load("city/large_buildingF.glb#Scene0"),
            large_building_g: ass.load("city/large_buildingG.glb#Scene0"),
            low_building_a: ass.load("city/low_buildingA.glb#Scene0"),
            low_building_b: ass.load("city/low_buildingB.glb#Scene0"),
            low_building_c: ass.load("city/low_buildingC.glb#Scene0"),
            low_building_d: ass.load("city/low_buildingD.glb#Scene0"),
            low_building_e: ass.load("city/low_buildingE.glb#Scene0"),
            low_building_f: ass.load("city/low_buildingF.glb#Scene0"),
            low_building_g: ass.load("city/low_buildingG.glb#Scene0"),
            low_building_h: ass.load("city/low_buildingH.glb#Scene0"),
            low_building_i: ass.load("city/low_buildingI.glb#Scene0"),
            low_building_j: ass.load("city/low_buildingJ.glb#Scene0"),
            low_building_k: ass.load("city/low_buildingK.glb#Scene0"),
            low_building_l: ass.load("city/low_buildingL.glb#Scene0"),
            low_building_m: ass.load("city/low_buildingM.glb#Scene0"),
            low_building_n: ass.load("city/low_buildingN.glb#Scene0"),
            low_wide_a: ass.load("city/low_wideA.glb#Scene0"),
            low_wide_b: ass.load("city/low_wideB.glb#Scene0"),
            roof_center: ass.load("city/roof_center.glb#Scene0"),
            roof_corner: ass.load("city/roof_corner.glb#Scene0"),
            roof_overhang: ass.load("city/roof_overhang.glb#Scene0"),
            roof_side: ass.load("city/roof_side.glb#Scene0"),
            sign_billboard: ass.load("city/sign_billboard.glb#Scene0"),
            sign_hospital: ass.load("city/sign_hospital.glb#Scene0"),
            skyscraper_a: ass.load("city/skyscraperA.glb#Scene0"),
            skyscraper_b: ass.load("city/skyscraperB.glb#Scene0"),
            skyscraper_c: ass.load("city/skyscraperC.glb#Scene0"),
            skyscraper_d: ass.load("city/skyscraperD.glb#Scene0"),
            skyscraper_e: ass.load("city/skyscraperE.glb#Scene0"),
            skyscraper_f: ass.load("city/skyscraperF.glb#Scene0"),
            small_building_a: ass.load("city/small_buildingA.glb#Scene0"),
            small_building_b: ass.load("city/small_buildingB.glb#Scene0"),
            small_building_c: ass.load("city/small_buildingC.glb#Scene0"),
            small_building_d: ass.load("city/small_buildingD.glb#Scene0"),
            small_building_e: ass.load("city/small_buildingE.glb#Scene0"),
            small_building_f: ass.load("city/small_buildingF.glb#Scene0"),
            wall_door_a: ass.load("city/wall_doorA.glb#Scene0"),
            wall_door_b: ass.load("city/wall_doorB.glb#Scene0"),
            wall_solid: ass.load("city/wall_solid.glb#Scene0"),
            wall_window_a: ass.load("city/wall_windowA.glb#Scene0"),
            wall_window_b: ass.load("city/wall_windowB.glb#Scene0"),
            wall_window_c: ass.load("city/wall_windowC.glb#Scene0"),
            wall_window_d: ass.load("city/wall_windowD.glb#Scene0"),
            wall_window_e: ass.load("city/wall_windowE.glb#Scene0"),
            wall_window_f: ass.load("city/wall_windowF.glb#Scene0"),
        }
    }
}

#[allow(dead_code)]
fn spawn_city_handle(
    pos: Vec3,
    handle: Handle<Scene>,
    assets_gltf: &Assets<Gltf>,
    commands: &mut Commands,
) {
    if let Some(gltf) = assets_gltf.get(handle) {
        let mut t = Transform::from_translation(pos);
        t.scale = Vec3::splat(5.0);

        commands
            .spawn_bundle((t, GlobalTransform::identity()))
            .with_children(|parent| {
                //parent.spawn_scene(gltf.named_scenes["YellowCar"].clone());
                parent.spawn_scene(gltf.scenes[0].clone());
            });
        //commands.spawn_scene(gltf.scenes[0].clone());
    }
}
