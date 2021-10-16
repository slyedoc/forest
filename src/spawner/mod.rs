use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::*;
use bevy_inspector_egui::egui::*;
use bevy_mod_picking::*;
use bevy_mod_raycast::*;

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawner_window_system);
            //.add_plugin(DefaultRaycastingPlugin::<SpawnerSet>::default());
        // You will need to pay attention to what order you add systems! Putting them in the wrong
        // order can result in multiple frames of latency. Ray casting should probably happen after
        // the positions of your meshes have been updated in the UPDATE stage.
        // .add_system_to_stage(
        //     CoreStage::PreUpdate,
        //     update_raycast_with_cursor.before(RaycastSystem::BuildRays),
        // )
        // .add_system_to_stage(
        //     CoreStage::PreUpdate,
        //     update_spawn_cursor::<PickingRaycastSet>.after(PickingSystem::UpdateRaycast),
        // );
    }
}

pub fn spawner_window_system(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    mut windows: ResMut<EditorWindows>,
) {
    use bevy_inspector_egui::egui::Window;

    Window::new("Spawner")
        .open(&mut windows.spawner)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            let mut building: Option<BuildingType> = None;
            //ui.label("Add Building");
            CollapsingHeader::new("Buildings")
                .default_open(true)
                .show(ui, |ui| {
                    if ui.button("Low").clicked() {
                        building = Some(BuildingType::Low);
                    }
                    if ui.button("Large").clicked() {
                        building = Some(BuildingType::Large);
                    }
                    if ui.button("Small").clicked() {
                        building = Some(BuildingType::Small);
                    }
                    if ui.button("Skyscraper").clicked() {
                        building = Some(BuildingType::Skyscraper);
                    }
                });

            if let Some(b) = building {
                commands.spawn_bundle(CityBuildingBundle {
                    building: CityBuilding {
                        building_type: b,
                        index: Some(0),
                    },
                    ..Default::default()
                });
            }

            let mut block: Option<BuildingType> = None;
            CollapsingHeader::new("Road")
                .default_open(true)
                .show(ui, |ui| {
                    if ui.button("Low").clicked() {
                        block = Some(BuildingType::Low);
                    }
                    if ui.button("Large").clicked() {
                        block = Some(BuildingType::Large);
                    }
                    if ui.button("Small").clicked() {
                        block = Some(BuildingType::Small);
                    }
                    if ui.button("Skyscraper").clicked() {
                        block = Some(BuildingType::Skyscraper);
                    }
                });

            if let Some(b) = block {
                commands.spawn_bundle(CityRoadBundle {
                    block: CityRoad {
                        building_type: b,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        });
}
// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct SpawnerSet;

// Update our `RayCastSource` with the current cursor position every frame.
// fn update_raycast_with_cursor(
//     mut cursor: EventReader<CursorMoved>,
//     mut query: Query<&mut RayCastSource<SpawnerSet>>,
// ) {
//     for mut pick_source in &mut query.iter_mut() {
//         // Grab the most recent cursor event if it exists:
//         if let Some(cursor_latest) = cursor.iter().last() {
//             pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
//         }
//     }
// }



#[derive(Default, Debug)]
pub struct SpawnerState {
    enabled: bool,
}

/// Updates the 3d cursor to be in the pointed world coordinates
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn update_spawn_cursor<T: 'static + Send + Sync>(
    mut commands: Commands,
    state: Res<SpawnerState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    added_sources_query: Query<&RayCastSource<T>, Added<RayCastSource<T>>>,
    mut cursor_query: Query<&mut GlobalTransform, With<DebugCursor<T>>>,
    mut cursor_tail_query: Query<
        &mut GlobalTransform,
        (With<DebugCursorTail<T>>, Without<DebugCursor<T>>),
    >,
    mut visibility_query: Query<&mut Visible, With<DebugCursorMesh<T>>>,
    raycast_source_query: Query<&RayCastSource<T>>,
) {
    if !state.enabled {
        return;
    }

    let cube_size = 0.04;
    let cube_tail_scale = 20.0;
    let ball_size = 0.08;
    let debug_material = &materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        unlit: true,
        ..Default::default()
    });

    for _source in added_sources_query.iter() {
        commands
            // cursor
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    subdivisions: 4,
                    radius: ball_size,
                })),
                material: debug_material.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                let mut transform = Transform::from_translation(Vec3::new(
                    0.0,
                    (cube_size * cube_tail_scale) / 2.0,
                    0.0,
                ));
                transform.apply_non_uniform_scale(Vec3::from([1.0, cube_tail_scale, 1.0]));

                // child cube
                parent.spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
                    material: debug_material.clone(),
                    transform,
                    ..Default::default()
                });
                //.insert(DebugCursorTail::<T>::default())
                //.insert(DebugCursorMesh::<T>::default());
            });
        //.insert(DebugCursor::<T>::default())
        //.insert(DebugCursorMesh::<T>::default());
    }

    // Set the cursor translation to the top pick's world coordinates
    for raycast_source in raycast_source_query.iter() {
        match raycast_source.intersect_top() {
            Some(top_intersection) => {
                let transform_new = top_intersection.1.normal_ray().to_transform();
                for mut transform in cursor_query.iter_mut() {
                    *transform = GlobalTransform::from_matrix(transform_new);
                }
                for mut transform in cursor_tail_query.iter_mut() {
                    let scale = Vec3::from([1.0, cube_tail_scale, 1.0]);
                    let rotation = Quat::default();
                    let translation = Vec3::new(0.0, (cube_size * cube_tail_scale) / 2.0, 0.0);
                    let transform_move =
                        Mat4::from_scale_rotation_translation(scale, rotation, translation);
                    *transform = GlobalTransform::from_matrix(transform_new * transform_move)
                }
                for mut visible in &mut visibility_query.iter_mut() {
                    visible.is_visible = true;
                }
            }
            None => {
                for mut visible in &mut visibility_query.iter_mut() {
                    visible.is_visible = false;
                }
            }
        }
    }
}