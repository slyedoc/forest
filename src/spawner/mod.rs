use crate::prelude::*;
use bevy::prelude::*;
use bevy_egui::*;
use bevy_inspector_egui::egui::*;

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawner_window_system);
    }
}
pub fn spawner_window_system(
    mut commands: Commands,
    egui_ctx: Res<EguiContext>,
    mut windows: ResMut<Editor>,
) {

    bevy_egui::egui::Window::new("Spawner")
        .open(&mut windows.spawner)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            let mut building: Option<CityType> = None;
            //ui.label("Add Building");
            CollapsingHeader::new("Buildings")
                .default_open(true)
                .show(ui, |ui| {
                    if ui.button("Low").clicked() {
                        building = Some(CityType::Low(Low::BuildingA));
                    }
                    if ui.button("Large").clicked() {
                        building = Some(CityType::Large(Large::BuildingA));
                    }
                    if ui.button("Small").clicked() {
                        building = Some(CityType::Small(Small::BuildingA));
                    }
                    if ui.button("Skyscraper").clicked() {
                        building = Some(CityType::Skyscraper(Skyscraper::BuildingA));
                    }
                });

            if let Some(t) = building {
                commands.spawn_bundle(CityAssetBundle {
                    city_type: t,
                    ..Default::default()
                });
            }

        });
}
// This is a unit struct we will use to mark our generic `RayCastMesh`s and `RayCastSource` as part
// of the same group, or "RayCastSet". For more complex use cases, you might use this to associate
// some meshes with one ray casting source, and other meshes with a different ray casting source."
pub struct SpawnerSet;

