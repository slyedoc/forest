use bevy::gltf::Gltf;
use bevy::math::*;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CityAssets>();
    }
}

//#[derive(Inspectable)]: Handle<Gltf>,
#[derive(AssetCollection)]
pub struct CityAssets {

    #[asset(path = "city/detail_awning.glb")]
    pub detail_awning: Handle<Gltf>,
    #[asset(path = "city/detail_awningWide.glb")]
    pub detail_awning_wide: Handle<Gltf>,
    #[asset(path = "city/detail_overhang.glb")]
    pub detail_overhang: Handle<Gltf>,
    #[asset(path = "city/detail_overhangWide.glb")]
    pub detail_overhang_wide: Handle<Gltf>,
    #[asset(path = "city/detail_umbrellaDetailed.glb")]
    pub detail_umbrella_detailed: Handle<Gltf>,
    #[asset(path = "city/detail_umbrella.glb")]
    pub detail_umbrella: Handle<Gltf>,
    #[asset(path = "city/large_buildingA.glb")]
    pub large_building_a: Handle<Gltf>,
    #[asset(path = "city/large_buildingB.glb")]
    pub large_building_b: Handle<Gltf>,
    #[asset(path = "city/large_buildingC.glb")]
    pub large_building_c: Handle<Gltf>,
    #[asset(path = "city/large_buildingD.glb")]
    pub large_building_d: Handle<Gltf>,
    #[asset(path = "city/large_buildingE.glb")]
    pub large_building_e: Handle<Gltf>,
    #[asset(path = "city/large_buildingF.glb")]
    pub large_building_f: Handle<Gltf>,
    #[asset(path = "city/large_buildingG.glb")]
    pub large_building_g: Handle<Gltf>,
    #[asset(path = "city/low_buildingA.glb")]
    pub low_building_a: Handle<Gltf>,
    #[asset(path = "city/low_buildingB.glb")]
    pub low_building_b: Handle<Gltf>,
    #[asset(path = "city/low_buildingC.glb")]
    pub low_building_c: Handle<Gltf>,
    #[asset(path = "city/low_buildingD.glb")]
    pub low_building_d: Handle<Gltf>,
    #[asset(path = "city/low_buildingE.glb")]
    pub low_building_e: Handle<Gltf>,
    #[asset(path = "city/low_buildingF.glb")]
    pub low_building_f: Handle<Gltf>,
    #[asset(path = "city/low_buildingG.glb")]
    pub low_building_g: Handle<Gltf>,
    #[asset(path = "city/low_buildingH.glb")]
    pub low_building_h: Handle<Gltf>,
    #[asset(path = "city/low_buildingI.glb")]
    pub low_building_i: Handle<Gltf>,
    #[asset(path = "city/low_buildingJ.glb")]
    pub low_building_j: Handle<Gltf>,
    #[asset(path = "city/low_buildingK.glb")]
    pub low_building_k: Handle<Gltf>,
    #[asset(path = "city/low_buildingL.glb")]
    pub low_building_l: Handle<Gltf>,
    #[asset(path = "city/low_buildingM.glb")]
    pub low_building_m: Handle<Gltf>,
    #[asset(path = "city/low_buildingN.glb")]
    pub low_building_n: Handle<Gltf>,
    #[asset(path = "city/low_wideA.glb")]
    pub low_wide_a: Handle<Gltf>,
    #[asset(path = "city/low_wideB.glb")]
    pub low_wide_b: Handle<Gltf>,
    #[asset(path = "city/roof_center.glb")]
    pub roof_center: Handle<Gltf>,
    #[asset(path = "city/roof_corner.glb")]
    pub roof_corner: Handle<Gltf>,
    #[asset(path = "city/roof_overhang.glb")]
    pub roof_overhang: Handle<Gltf>,
    #[asset(path = "city/roof_side.glb")]
    pub roof_side: Handle<Gltf>,
    #[asset(path = "city/sign_billboard.glb")]
    pub sign_billboard: Handle<Gltf>,
    #[asset(path = "city/sign_hospital.glb")]
    pub sign_hospital: Handle<Gltf>,
    #[asset(path = "city/skyscraperA.glb")]
    pub skyscraper_a: Handle<Gltf>,
    #[asset(path = "city/skyscraperB.glb")]
    pub skyscraper_b: Handle<Gltf>,
    #[asset(path = "city/skyscraperC.glb")]
    pub skyscraper_c: Handle<Gltf>,
    #[asset(path = "city/skyscraperD.glb")]
    pub skyscraper_d: Handle<Gltf>,
    #[asset(path = "city/skyscraperE.glb")]
    pub skyscraper_e: Handle<Gltf>,
    #[asset(path = "city/skyscraperF.glb")]
    pub skyscraper_f: Handle<Gltf>,
    #[asset(path = "city/small_buildingA.glb")]
    pub small_building_a: Handle<Gltf>,
    #[asset(path = "city/small_buildingB.glb")]
    pub small_building_b: Handle<Gltf>,
    #[asset(path = "city/small_buildingC.glb")]
    pub small_building_c: Handle<Gltf>,
    #[asset(path = "city/small_buildingD.glb")]
    pub small_building_d: Handle<Gltf>,
    #[asset(path = "city/small_buildingE.glb")]
    pub small_building_e: Handle<Gltf>,
    #[asset(path = "city/small_buildingF.glb")]
    pub small_building_f: Handle<Gltf>,
    #[asset(path = "city/wall_doorA.glb")]
    pub wall_door_a: Handle<Gltf>,
    #[asset(path = "city/wall_doorB.glb")]
    pub wall_door_b: Handle<Gltf>,
    #[asset(path = "city/wall_solid.glb")]
    pub wall_solid: Handle<Gltf>,
    #[asset(path = "city/wall_windowA.glb")]
    pub wall_window_a: Handle<Gltf>,
    #[asset(path = "city/wall_windowB.glb")]
    pub wall_window_b: Handle<Gltf>,
    #[asset(path = "city/wall_windowC.glb")]
    pub wall_window_c: Handle<Gltf>,
    #[asset(path = "city/wall_windowD.glb")]
    pub wall_window_d: Handle<Gltf>,
    #[asset(path = "city/wall_windowE.glb")]
    pub wall_window_e: Handle<Gltf>,
    #[asset(path = "city/wall_windowF.glb")]
    pub wall_window_f: Handle<Gltf>,
}

pub fn spawn_city_debug(commands: &mut Commands, city: &CityAssets, assets_gltf: &Assets<Gltf>) {
    let grid = 4;
    let offset = 10.0;
    let mut positions = Vec::new();
    for i in -grid..grid {
        let x = i as f32 * offset;
        for j in -grid..grid {
            let z = j as f32 * offset;
            positions.push(vec3(x, 0.1, z));
        }
    }

    // For some reason a few of these don't work
    spawn_city_handle(
        positions[0],
        city.detail_awning.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[1],
        city.detail_awning_wide.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[2],
        city.detail_overhang.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[3],
        city.detail_overhang_wide.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[3],
        city.detail_umbrella.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[4],
        city.detail_umbrella_detailed.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[5],
        city.large_building_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[6],
        city.large_building_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[7],
        city.large_building_c.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[8],
        city.large_building_d.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[9],
        city.large_building_e.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[10],
        city.large_building_f.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[11],
        city.large_building_g.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[12],
        city.low_building_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[13],
        city.low_building_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[14],
        city.low_building_c.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[15],
        city.low_building_d.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[16],
        city.low_building_e.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[17],
        city.low_building_f.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[18],
        city.low_building_g.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[19],
        city.low_building_h.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[20],
        city.low_building_i.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[21],
        city.low_building_j.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[22],
        city.low_building_k.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[23],
        city.low_building_l.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[24],
        city.low_building_m.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[25],
        city.low_building_n.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[26],
        city.low_building_n.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[27],
        city.low_wide_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[28],
        city.low_wide_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[29],
        city.roof_center.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[30],
        city.roof_corner.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[31],
        city.roof_overhang.clone(),
        assets_gltf,
        commands,
    );
    // wtf is formater handin
    spawn_city_handle(positions[32], city.roof_side.clone(), assets_gltf, commands);
    spawn_city_handle(
        positions[33],
        city.sign_billboard.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[34],
        city.sign_hospital.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[35],
        city.skyscraper_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[36],
        city.skyscraper_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[37],
        city.skyscraper_c.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[38],
        city.skyscraper_d.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[39],
        city.skyscraper_e.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[40],
        city.skyscraper_f.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[41],
        city.small_building_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[42],
        city.small_building_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[43],
        city.small_building_c.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[44],
        city.small_building_d.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[45],
        city.small_building_e.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[46],
        city.small_building_f.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[47],
        city.wall_door_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[48],
        city.wall_door_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[49],
        city.wall_solid.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[50],
        city.wall_window_a.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[51],
        city.wall_window_b.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[52],
        city.wall_window_c.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[53],
        city.wall_window_d.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[54],
        city.wall_window_e.clone(),
        assets_gltf,
        commands,
    );
    spawn_city_handle(
        positions[55],
        city.wall_window_f.clone(),
        assets_gltf,
        commands,
    );
}

fn spawn_city_handle(
    pos: Vec3,
    handle: Handle<Gltf>,
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
