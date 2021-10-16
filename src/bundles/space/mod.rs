use bevy::asset::LoadState;
use bevy::math::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;

pub struct SpacePlugin;
impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpaceAssets>()
            .add_system_to_stage(CoreStage::Last, update_space_system);

        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();

        // registering custom component to be able to edit it in inspector
        registry.register::<SpaceAsset>();
    }
}

//#[derive(Inspectable)]: Handle<Scene>,
pub struct SpaceAssets {
    pub alien: Handle<Scene>,
    pub astronautA: Handle<Scene>,
    pub astronautB: Handle<Scene>,
    pub barrel: Handle<Scene>,
    pub barrels: Handle<Scene>,
    pub barrels_rail: Handle<Scene>,
    pub bones: Handle<Scene>,
    pub chimney_detailed: Handle<Scene>,
    pub chimney: Handle<Scene>,
    pub corridor_corner: Handle<Scene>,
    pub corridor_cornerRound: Handle<Scene>,
    pub corridor_cornerRoundWindow: Handle<Scene>,
    pub corridor_cross: Handle<Scene>,
    pub corridor_detailed: Handle<Scene>,
    pub corridor_end: Handle<Scene>,
    pub corridor: Handle<Scene>,
    pub corridor_open: Handle<Scene>,
    pub corridor_roof: Handle<Scene>,
    pub corridor_split: Handle<Scene>,
    pub corridor_wallCorner: Handle<Scene>,
    pub corridor_wall: Handle<Scene>,
    pub corridor_windowClosed: Handle<Scene>,
    pub corridor_window: Handle<Scene>,
    pub craft_cargoA: Handle<Scene>,
    pub craft_cargoB: Handle<Scene>,
    pub craft_miner: Handle<Scene>,
    pub craft_racer: Handle<Scene>,
    pub craft_speederA: Handle<Scene>,
    pub craft_speederB: Handle<Scene>,
    pub craft_speederC: Handle<Scene>,
    pub craft_speederD: Handle<Scene>,
    pub crater: Handle<Scene>,
    pub craterLarge: Handle<Scene>,
    pub desk_chairArms: Handle<Scene>,
    pub desk_chair: Handle<Scene>,
    pub desk_chairStool: Handle<Scene>,
    pub desk_computerCorner: Handle<Scene>,
    pub desk_computer: Handle<Scene>,
    pub desk_computerScreen: Handle<Scene>,
    pub gate_complex: Handle<Scene>,
    pub gate_simple: Handle<Scene>,
    pub hangar_largeA: Handle<Scene>,
    pub hangar_largeB: Handle<Scene>,
    pub hangar_roundA: Handle<Scene>,
    pub hangar_roundB: Handle<Scene>,
    pub hangar_roundGlass: Handle<Scene>,
    pub hangar_smallA: Handle<Scene>,
    pub hangar_smallB: Handle<Scene>,
    pub machine_barrel: Handle<Scene>,
    pub machine_barrelLarge: Handle<Scene>,
    pub machine_generator: Handle<Scene>,
    pub machine_generatorLarge: Handle<Scene>,
    pub machine_wirelessCable: Handle<Scene>,
    pub machine_wireless: Handle<Scene>,
    pub meteor_detailed: Handle<Scene>,
    pub meteor: Handle<Scene>,
    pub meteor_half: Handle<Scene>,
    pub monorail_trackCornerLarge: Handle<Scene>,
    pub monorail_trackCornerSmall: Handle<Scene>,
    pub monorail_trackSlope: Handle<Scene>,
    pub monorail_trackStraight: Handle<Scene>,
    pub monorail_trackSupportCorner: Handle<Scene>,
    pub monorail_trackSupport: Handle<Scene>,
    pub monorail_trainBox: Handle<Scene>,
    pub monorail_trainCargo: Handle<Scene>,
    pub monorail_trainEnd: Handle<Scene>,
    pub monorail_trainFlat: Handle<Scene>,
    pub monorail_trainFront: Handle<Scene>,
    pub monorail_trainPassenger: Handle<Scene>,
    pub pipe_cornerDiagonal: Handle<Scene>,
    pub pipe_corner: Handle<Scene>,
    pub pipe_cornerRound: Handle<Scene>,
    pub pipe_cornerRoundLarge: Handle<Scene>,
    pub pipe_cross: Handle<Scene>,
    pub pipe_end: Handle<Scene>,
    pub pipe_entrance: Handle<Scene>,
    pub pipe_open: Handle<Scene>,
    pub pipe_rampLarge: Handle<Scene>,
    pub pipe_rampSmall: Handle<Scene>,
    pub pipe_ring: Handle<Scene>,
    pub pipe_ringHighEnd: Handle<Scene>,
    pub pipe_ringHigh: Handle<Scene>,
    pub pipe_ringSupport: Handle<Scene>,
    pub pipe_split: Handle<Scene>,
    pub pipe_straight: Handle<Scene>,
    pub pipe_supportHigh: Handle<Scene>,
    pub pipe_supportLow: Handle<Scene>,
    pub platform_center: Handle<Scene>,
    pub platform_corner: Handle<Scene>,
    pub platform_cornerOpen: Handle<Scene>,
    pub platform_cornerRound: Handle<Scene>,
    pub platform_end: Handle<Scene>,
    pub platform_high: Handle<Scene>,
    pub platform_large: Handle<Scene>,
    pub platform_long: Handle<Scene>,
    pub platform_low: Handle<Scene>,
    pub platform_side: Handle<Scene>,
    pub platform_smallDiagonal: Handle<Scene>,
    pub platform_small: Handle<Scene>,
    pub platform_straight: Handle<Scene>,
    pub rail_corner: Handle<Scene>,
    pub rail_end: Handle<Scene>,
    pub rail: Handle<Scene>,
    pub rail_middle: Handle<Scene>,
    pub rock_crystals: Handle<Scene>,
    pub rock_crystalsLargeA: Handle<Scene>,
    pub rock_crystalsLargeB: Handle<Scene>,
    pub rocket_baseA: Handle<Scene>,
    pub rocket_baseB: Handle<Scene>,
    pub rocket_finsA: Handle<Scene>,
    pub rocket_finsB: Handle<Scene>,
    pub rocket_fuelA: Handle<Scene>,
    pub rocket_fuelB: Handle<Scene>,
    pub rocket_sidesA: Handle<Scene>,
    pub rocket_sidesB: Handle<Scene>,
    pub rocket_topA: Handle<Scene>,
    pub rocket_topB: Handle<Scene>,
    pub rock: Handle<Scene>,
    pub rock_largeA: Handle<Scene>,
    pub rock_largeB: Handle<Scene>,
    pub rocks_smallA: Handle<Scene>,
    pub rocks_smallB: Handle<Scene>,
    pub rover: Handle<Scene>,
    pub satelliteDish_detailed: Handle<Scene>,
    pub satelliteDish: Handle<Scene>,
    pub satelliteDish_large: Handle<Scene>,
    pub stairs_corner: Handle<Scene>,
    pub stairs: Handle<Scene>,
    pub stairs_short: Handle<Scene>,
    pub structure_closed: Handle<Scene>,
    pub structure_detailed: Handle<Scene>,
    pub structure_diagonal: Handle<Scene>,
    pub structure: Handle<Scene>,
    pub supports_high: Handle<Scene>,
    pub supports_low: Handle<Scene>,
    pub terrain: Handle<Scene>,
    pub terrain_ramp: Handle<Scene>,
    pub terrain_rampLarge_detailed: Handle<Scene>,
    pub terrain_rampLarge: Handle<Scene>,
    pub terrain_roadCorner: Handle<Scene>,
    pub terrain_roadCross: Handle<Scene>,
    pub terrain_roadEnd: Handle<Scene>,
    pub terrain_roadSplit: Handle<Scene>,
    pub terrain_roadStraight: Handle<Scene>,
    pub terrain_sideCliff: Handle<Scene>,
    pub terrain_sideCorner: Handle<Scene>,
    pub terrain_sideCornerInner: Handle<Scene>,
    pub terrain_sideEnd: Handle<Scene>,
    pub terrain_side: Handle<Scene>,
    pub turret_double: Handle<Scene>,
    pub turret_single: Handle<Scene>,
    pub weapon_gun: Handle<Scene>,
    pub weapon_rifle: Handle<Scene>,
}

#[derive(Bundle, Default)]
pub struct SpaceAssetBundle {
    pub building: SpaceAsset,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Inspectable, Component, Default)]
pub struct SpaceAsset {
    pub space_type: SpaceType,
}

fn update_space_system(
    mut commands: Commands,
    assets: Res<SpaceAssets>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut SpaceAsset), Added<SpaceAsset>>,
) {
    // on next frame build a building back
    for (e, mut building) in query.iter_mut() {
        info!("init space asset");
        let handle = get_space_type_handle(building.space_type, &assets);

        let state = asset_server.get_load_state(handle.clone());
        if state == LoadState::Loaded {
            commands.entity(e).with_children(|parent| {
                parent.spawn_scene(handle);
            });
        } else {
            warn!("handle on loaded!, loadstate={:?}", state);
        }
        info!("init space asset done");
    }
}

// Could use asset server, but using SpaceAssets lets us start a preload
// These clones are of the handle only,
pub fn get_space_type_handle(t: SpaceType,  assets: &SpaceAssets) -> Handle<Scene> {
       match t {
            SpaceType::Alien => assets.alien.clone(),
            SpaceType::Astronaut(a) => match a {
                Astronaut::A => assets.astronautA.clone(),
                Astronaut::B => assets.astronautA.clone(),
            },
            SpaceType::Barrel(b) => match b {
                Barrel::Normal => assets.barrel.clone(),
                Barrel::Multiple => assets.barrels.clone(),
                Barrel::Rail => assets.barrels_rail.clone(),
            },
            SpaceType::Bones => assets.bones.clone(),
            SpaceType::Chimney(c) => match c {
                Chimney::Normal => assets.chimney.clone(),
                Chimney::Detailed => assets.chimney_detailed.clone(),
            },
            SpaceType::Corridor(c) => match c {
                Corridor::Normal => assets.corridor.clone(),
                Corridor::Open => assets.corridor_open.clone(),
                Corridor::Corner => assets.corridor_corner.clone(),
                Corridor::CornerRound => assets.corridor_cornerRound.clone(),
                Corridor::CornerRoundWindow => assets.corridor_cornerRoundWindow.clone(),
                Corridor::Cross => assets.corridor_cross.clone(),
                Corridor::Detailed => assets.corridor_detailed.clone(),
                Corridor::End => assets.corridor_end.clone(),
                Corridor::Roof => assets.corridor_roof.clone(),
                Corridor::Split => assets.corridor_split.clone(),
                Corridor::WallCorner => assets.corridor_wallCorner.clone(),
                Corridor::Wall => assets.corridor_wall.clone(),
                Corridor::WindowClosed => assets.corridor_windowClosed.clone(),
                Corridor::Window => assets.corridor_window.clone(),
            },
            SpaceType::Crater(c) => match c {
                Crater::Normal => assets.crater.clone(),
                Crater::Large => assets.craterLarge.clone(),
            },
            SpaceType::Craft(c) => match c {
                Craft::CargoA => assets.craft_cargoA.clone(),
                Craft::CargoB => assets.craft_cargoB.clone(),
                Craft::Miner => assets.craft_miner.clone(),
                Craft::Racer => assets.craft_racer.clone(),
                Craft::SpeederA => assets.craft_speederA.clone(),
                Craft::SpeederB => assets.craft_speederB.clone(),
                Craft::SpeederC => assets.craft_speederC.clone(),
                Craft::SpeederD => assets.craft_speederD.clone(),
            },
            SpaceType::Desk(a) => match a {
                Desk::ChairArms => assets.desk_chairArms.clone(),
                Desk::Chair => assets.desk_chair.clone(),
                Desk::ChairStool => assets.desk_chairStool.clone(),
                Desk::ComputerCorner => assets.desk_computerCorner.clone(),
                Desk::Computer => assets.desk_computer.clone(),
                Desk::ComputerScreen => assets.desk_computerScreen.clone(),
            },
            SpaceType::Gate(a) => match a {
                Gate::Complex => todo!(),
                Gate::Simple => todo!(),
            },
            SpaceType::Hanger(a) => match a {
                Hanger::LargeA => todo!(),
                Hanger::LargeB => todo!(),
                Hanger::RoundA => todo!(),
                Hanger::RoundB => todo!(),
                Hanger::RoundGlass => todo!(),
                Hanger::SmallA => todo!(),
                Hanger::SmallB => todo!(),
            },
            SpaceType::Machine(a) => match a {
                Machine::Barrel => todo!(),
                Machine::BarrelLarge => todo!(),
                Machine::Generator => todo!(),
                Machine::GeneratorLarge => todo!(),
                Machine::WirelessCable => todo!(),
                Machine::Wireless => todo!(),
            },
            SpaceType::Meteor(a) => match a {
                Meteor::Normal => todo!(),
                Meteor::Detailed => todo!(),
                Meteor::Half => todo!(),
            },
            SpaceType::Monorail(a) => match a {
                Monorail::TrackCornerLarge => todo!(),
                Monorail::TrackCornerSmall => todo!(),
                Monorail::TrackSlope => todo!(),
                Monorail::TrackStraight => todo!(),
                Monorail::TrackSupportCorner => todo!(),
                Monorail::TrackSupport => todo!(),
                Monorail::TrainBox => todo!(),
                Monorail::TrainCargo => todo!(),
                Monorail::TrainEnd => todo!(),
                Monorail::TrainFlat => todo!(),
                Monorail::TrainFront => todo!(),
                Monorail::TrainPassenger => todo!(),
            },
            SpaceType::Pipe(a) => match a {
                Pipe::CornerDiagonal => todo!(),
                Pipe::Corner => todo!(),
                Pipe::CornerRound => todo!(),
                Pipe::CornerRoundLarge => todo!(),
                Pipe::Cross => todo!(),
                Pipe::End => todo!(),
                Pipe::Entrance => todo!(),
                Pipe::Open => todo!(),
                Pipe::RampLarge => todo!(),
                Pipe::RampSmall => todo!(),
                Pipe::Ring => todo!(),
                Pipe::RingHighEnd => todo!(),
                Pipe::RingHigh => todo!(),
                Pipe::RingSupport => todo!(),
                Pipe::Split => todo!(),
                Pipe::Straight => todo!(),
                Pipe::SupportHigh => todo!(),
                Pipe::SupportLow => todo!(),
            },
            SpaceType::Platform(a) => match a {
                Platform::Center => todo!(),
                Platform::Corner => todo!(),
                Platform::CornerOpen => todo!(),
                Platform::CornerRound => todo!(),
                Platform::End => todo!(),
                Platform::High => todo!(),
                Platform::Large => todo!(),
                Platform::Long => todo!(),
                Platform::Low => todo!(),
                Platform::Side => todo!(),
                Platform::SmallDiagonal => todo!(),
                Platform::Small => todo!(),
                Platform::Straight => todo!(),
            },
            SpaceType::Rail(a) => match a {
                Rail::Corner => todo!(),
                Rail::End => todo!(),
                Rail::Normal => todo!(),
                Rail::Middle => todo!(),
            },
            SpaceType::Rock(a) => match a {
                Rock::Normal => todo!(),
                Rock::LargeA => todo!(),
                Rock::LargeB => todo!(),
                Rock::SmallA => todo!(),
                Rock::SmallB => todo!(),
                Rock::Crystals => todo!(),
                Rock::CrystalsLargeA => todo!(),
                Rock::CrystalsLargeB => todo!(),
            },
            SpaceType::Rocket(a) => match a {
                Rocket::BaseA => todo!(),
                Rocket::BaseB => todo!(),
                Rocket::FinsA => todo!(),
                Rocket::FinsB => todo!(),
                Rocket::FuelA => todo!(),
                Rocket::FuelB => todo!(),
                Rocket::SidesA => todo!(),
                Rocket::SidesB => todo!(),
                Rocket::TopA => todo!(),
                Rocket::TopB => todo!(),
            },
            SpaceType::Rover => assets.rover.clone(),
            SpaceType::Stairs(a) => match a {
                Stairs::Corner => todo!(),
                Stairs::Normal => todo!(),
                Stairs::Short => todo!(),
            },
            SpaceType::SatelliteDish(a) => match a {
                SatelliteDish::Detailed => todo!(),
                SatelliteDish::Noraml => todo!(),
                SatelliteDish::Large => todo!(),
            },
            SpaceType::Supports(a) => match a {
                Supports::High => todo!(),
                Supports::Low => todo!(),
            },
            SpaceType::Structure(a) => match a {
                Structure::Normal => todo!(),
                Structure::Closed => todo!(),
                Structure::Detailed => todo!(),
                Structure::Diagonal => todo!(),
            },
            SpaceType::Terrain(a) => match a {
                Terrain::Normal => todo!(),
                Terrain::Ramp => todo!(),
                Terrain::RampLargeDetailed => todo!(),
                Terrain::RampLarge => todo!(),
                Terrain::RoadCorner => todo!(),
                Terrain::RoadCross => todo!(),
                Terrain::RoadEnd => todo!(),
                Terrain::RoadSplit => todo!(),
                Terrain::RoadStraight => todo!(),
                Terrain::SideCliff => todo!(),
                Terrain::SideCorner => todo!(),
                Terrain::SideCornerInner => todo!(),
                Terrain::SideEnd => todo!(),
                Terrain::Side => todo!(),
            },
            SpaceType::Turret(a) => match a {
                Turret::Double => todo!(),
                Turret::Single => todo!(),
            },
            SpaceType::Weapon(a) => match a {
                Weapon::Gun => todo!(),
                Weapon::Rifle => todo!(),
            },
        }
}

impl FromWorld for SpaceAssets {
    fn from_world(world: &mut World) -> Self {
        let ass = world.get_resource::<AssetServer>().unwrap();
        Self {
            alien: ass.load("space/alien.glb#Scene0"),
            astronautA: ass.load("space/astronautA.glb#Scene0"),
            astronautB: ass.load("space/astronautB.glb#Scene0"),
            barrel: ass.load("space/barrel.glb#Scene0"),
            barrels: ass.load("space/barrels.glb#Scene0"),
            barrels_rail: ass.load("space/barrels_rail.glb#Scene0"),
            bones: ass.load("space/bones.glb#Scene0"),
            chimney_detailed: ass.load("space/chimney_detailed.glb#Scene0"),
            chimney: ass.load("space/chimney.glb#Scene0"),
            corridor_corner: ass.load("space/corridor_corner.glb#Scene0"),
            corridor_cornerRound: ass.load("space/corridor_cornerRound.glb#Scene0"),
            corridor_cornerRoundWindow: ass.load("space/corridor_cornerRoundWindow.glb#Scene0"),
            corridor_cross: ass.load("space/corridor_cross.glb#Scene0"),
            corridor_detailed: ass.load("space/corridor_detailed.glb#Scene0"),
            corridor_end: ass.load("space/corridor_end.glb#Scene0"),
            corridor: ass.load("space/corridor.glb#Scene0"),
            corridor_open: ass.load("space/corridor_open.glb#Scene0"),
            corridor_roof: ass.load("space/corridor_roof.glb#Scene0"),
            corridor_split: ass.load("space/corridor_split.glb#Scene0"),
            corridor_wallCorner: ass.load("space/corridor_wallCorner.glb#Scene0"),
            corridor_wall: ass.load("space/corridor_wall.glb#Scene0"),
            corridor_windowClosed: ass.load("space/corridor_windowClosed.glb#Scene0"),
            corridor_window: ass.load("space/corridor_window.glb#Scene0"),
            craft_cargoA: ass.load("space/craft_cargoA.glb#Scene0"),
            craft_cargoB: ass.load("space/craft_cargoB.glb#Scene0"),
            craft_miner: ass.load("space/craft_miner.glb#Scene0"),
            craft_racer: ass.load("space/craft_racer.glb#Scene0"),
            craft_speederA: ass.load("space/craft_speederA.glb#Scene0"),
            craft_speederB: ass.load("space/craft_speederB.glb#Scene0"),
            craft_speederC: ass.load("space/craft_speederC.glb#Scene0"),
            craft_speederD: ass.load("space/craft_speederD.glb#Scene0"),
            crater: ass.load("space/crater.glb#Scene0"),
            craterLarge: ass.load("space/craterLarge.glb#Scene0"),
            desk_chairArms: ass.load("space/desk_chairArms.glb#Scene0"),
            desk_chair: ass.load("space/desk_chair.glb#Scene0"),
            desk_chairStool: ass.load("space/desk_chairStool.glb#Scene0"),
            desk_computerCorner: ass.load("space/desk_computerCorner.glb#Scene0"),
            desk_computer: ass.load("space/desk_computer.glb#Scene0"),
            desk_computerScreen: ass.load("space/desk_computerScreen.glb#Scene0"),
            gate_complex: ass.load("space/gate_complex.glb#Scene0"),
            gate_simple: ass.load("space/gate_simple.glb#Scene0"),
            hangar_largeA: ass.load("space/hangar_largeA.glb#Scene0"),
            hangar_largeB: ass.load("space/hangar_largeB.glb#Scene0"),
            hangar_roundA: ass.load("space/hangar_roundA.glb#Scene0"),
            hangar_roundB: ass.load("space/hangar_roundB.glb#Scene0"),
            hangar_roundGlass: ass.load("space/hangar_roundGlass.glb#Scene0"),
            hangar_smallA: ass.load("space/hangar_smallA.glb#Scene0"),
            hangar_smallB: ass.load("space/hangar_smallB.glb#Scene0"),
            machine_barrel: ass.load("space/machine_barrel.glb#Scene0"),
            machine_barrelLarge: ass.load("space/machine_barrelLarge.glb#Scene0"),
            machine_generator: ass.load("space/machine_generator.glb#Scene0"),
            machine_generatorLarge: ass.load("space/machine_generatorLarge.glb#Scene0"),
            machine_wirelessCable: ass.load("space/machine_wirelessCable.glb#Scene0"),
            machine_wireless: ass.load("space/machine_wireless.glb#Scene0"),
            meteor_detailed: ass.load("space/meteor_detailed.glb#Scene0"),
            meteor: ass.load("space/meteor.glb#Scene0"),
            meteor_half: ass.load("space/meteor_half.glb#Scene0"),
            monorail_trackCornerLarge: ass.load("space/monorail_trackCornerLarge.glb#Scene0"),
            monorail_trackCornerSmall: ass.load("space/monorail_trackCornerSmall.glb#Scene0"),
            monorail_trackSlope: ass.load("space/monorail_trackSlope.glb#Scene0"),
            monorail_trackStraight: ass.load("space/monorail_trackStraight.glb#Scene0"),
            monorail_trackSupportCorner: ass.load("space/monorail_trackSupportCorner.glb#Scene0"),
            monorail_trackSupport: ass.load("space/monorail_trackSupport.glb#Scene0"),
            monorail_trainBox: ass.load("space/monorail_trainBox.glb#Scene0"),
            monorail_trainCargo: ass.load("space/monorail_trainCargo.glb#Scene0"),
            monorail_trainEnd: ass.load("space/monorail_trainEnd.glb#Scene0"),
            monorail_trainFlat: ass.load("space/monorail_trainFlat.glb#Scene0"),
            monorail_trainFront: ass.load("space/monorail_trainFront.glb#Scene0"),
            monorail_trainPassenger: ass.load("space/monorail_trainPassenger.glb#Scene0"),
            pipe_cornerDiagonal: ass.load("space/pipe_cornerDiagonal.glb#Scene0"),
            pipe_corner: ass.load("space/pipe_corner.glb#Scene0"),
            pipe_cornerRound: ass.load("space/pipe_cornerRound.glb#Scene0"),
            pipe_cornerRoundLarge: ass.load("space/pipe_cornerRoundLarge.glb#Scene0"),
            pipe_cross: ass.load("space/pipe_cross.glb#Scene0"),
            pipe_end: ass.load("space/pipe_end.glb#Scene0"),
            pipe_entrance: ass.load("space/pipe_entrance.glb#Scene0"),
            pipe_open: ass.load("space/pipe_open.glb#Scene0"),
            pipe_rampLarge: ass.load("space/pipe_rampLarge.glb#Scene0"),
            pipe_rampSmall: ass.load("space/pipe_rampSmall.glb#Scene0"),
            pipe_ring: ass.load("space/pipe_ring.glb#Scene0"),
            pipe_ringHighEnd: ass.load("space/pipe_ringHighEnd.glb#Scene0"),
            pipe_ringHigh: ass.load("space/pipe_ringHigh.glb#Scene0"),
            pipe_ringSupport: ass.load("space/pipe_ringSupport.glb#Scene0"),
            pipe_split: ass.load("space/pipe_split.glb#Scene0"),
            pipe_straight: ass.load("space/pipe_straight.glb#Scene0"),
            pipe_supportHigh: ass.load("space/pipe_supportHigh.glb#Scene0"),
            pipe_supportLow: ass.load("space/pipe_supportLow.glb#Scene0"),
            platform_center: ass.load("space/platform_center.glb#Scene0"),
            platform_corner: ass.load("space/platform_corner.glb#Scene0"),
            platform_cornerOpen: ass.load("space/platform_cornerOpen.glb#Scene0"),
            platform_cornerRound: ass.load("space/platform_cornerRound.glb#Scene0"),
            platform_end: ass.load("space/platform_end.glb#Scene0"),
            platform_high: ass.load("space/platform_high.glb#Scene0"),
            platform_large: ass.load("space/platform_large.glb#Scene0"),
            platform_long: ass.load("space/platform_long.glb#Scene0"),
            platform_low: ass.load("space/platform_low.glb#Scene0"),
            platform_side: ass.load("space/platform_side.glb#Scene0"),
            platform_smallDiagonal: ass.load("space/platform_smallDiagonal.glb#Scene0"),
            platform_small: ass.load("space/platform_small.glb#Scene0"),
            platform_straight: ass.load("space/platform_straight.glb#Scene0"),
            rail_corner: ass.load("space/rail_corner.glb#Scene0"),
            rail_end: ass.load("space/rail_end.glb#Scene0"),
            rail: ass.load("space/rail.glb#Scene0"),
            rail_middle: ass.load("space/rail_middle.glb#Scene0"),
            rock_crystals: ass.load("space/rock_crystals.glb#Scene0"),
            rock_crystalsLargeA: ass.load("space/rock_crystalsLargeA.glb#Scene0"),
            rock_crystalsLargeB: ass.load("space/rock_crystalsLargeB.glb#Scene0"),
            rocket_baseA: ass.load("space/rocket_baseA.glb#Scene0"),
            rocket_baseB: ass.load("space/rocket_baseB.glb#Scene0"),
            rocket_finsA: ass.load("space/rocket_finsA.glb#Scene0"),
            rocket_finsB: ass.load("space/rocket_finsB.glb#Scene0"),
            rocket_fuelA: ass.load("space/rocket_fuelA.glb#Scene0"),
            rocket_fuelB: ass.load("space/rocket_fuelB.glb#Scene0"),
            rocket_sidesA: ass.load("space/rocket_sidesA.glb#Scene0"),
            rocket_sidesB: ass.load("space/rocket_sidesB.glb#Scene0"),
            rocket_topA: ass.load("space/rocket_topA.glb#Scene0"),
            rocket_topB: ass.load("space/rocket_topB.glb#Scene0"),
            rock: ass.load("space/rock.glb#Scene0"),
            rock_largeA: ass.load("space/rock_largeA.glb#Scene0"),
            rock_largeB: ass.load("space/rock_largeB.glb#Scene0"),
            rocks_smallA: ass.load("space/rocks_smallA.glb#Scene0"),
            rocks_smallB: ass.load("space/rocks_smallB.glb#Scene0"),
            rover: ass.load("space/rover.glb#Scene0"),
            satelliteDish_detailed: ass.load("space/satelliteDish_detailed.glb#Scene0"),
            satelliteDish: ass.load("space/satelliteDish.glb#Scene0"),
            satelliteDish_large: ass.load("space/satelliteDish_large.glb#Scene0"),
            stairs_corner: ass.load("space/stairs_corner.glb#Scene0"),
            stairs: ass.load("space/stairs.glb#Scene0"),
            stairs_short: ass.load("space/stairs_short.glb#Scene0"),
            structure_closed: ass.load("space/structure_closed.glb#Scene0"),
            structure_detailed: ass.load("space/structure_detailed.glb#Scene0"),
            structure_diagonal: ass.load("space/structure_diagonal.glb#Scene0"),
            structure: ass.load("space/structure.glb#Scene0"),
            supports_high: ass.load("space/supports_high.glb#Scene0"),
            supports_low: ass.load("space/supports_low.glb#Scene0"),
            terrain: ass.load("space/terrain.glb#Scene0"),
            terrain_ramp: ass.load("space/terrain_ramp.glb#Scene0"),
            terrain_rampLarge_detailed: ass.load("space/terrain_rampLarge_detailed.glb#Scene0"),
            terrain_rampLarge: ass.load("space/terrain_rampLarge.glb#Scene0"),
            terrain_roadCorner: ass.load("space/terrain_roadCorner.glb#Scene0"),
            terrain_roadCross: ass.load("space/terrain_roadCross.glb#Scene0"),
            terrain_roadEnd: ass.load("space/terrain_roadEnd.glb#Scene0"),
            terrain_roadSplit: ass.load("space/terrain_roadSplit.glb#Scene0"),
            terrain_roadStraight: ass.load("space/terrain_roadStraight.glb#Scene0"),
            terrain_sideCliff: ass.load("space/terrain_sideCliff.glb#Scene0"),
            terrain_sideCorner: ass.load("space/terrain_sideCorner.glb#Scene0"),
            terrain_sideCornerInner: ass.load("space/terrain_sideCornerInner.glb#Scene0"),
            terrain_sideEnd: ass.load("space/terrain_sideEnd.glb#Scene0"),
            terrain_side: ass.load("space/terrain_side.glb#Scene0"),
            turret_double: ass.load("space/turret_double.glb#Scene0"),
            turret_single: ass.load("space/turret_single.glb#Scene0"),
            weapon_gun: ass.load("space/weapon_gun.glb#Scene0"),
            weapon_rifle: ass.load("space/weapon_rifle.glb#Scene0"),
        }
    }
}
///////////////////////////////////////////////////////////////
// Below this point are just enums to make using these assets easier
///////////////////////////////////////////////////////////////

#[derive(Component, Debug, Inspectable, Default, Copy, Clone)]
pub enum SpaceType {
    #[default]
    Alien,
    Astronaut(Astronaut),
    Barrel(Barrel),
    Bones,
    Chimney(Chimney),
    Corridor(Corridor),
    Craft(Craft),
    Crater(Crater),
    Desk(Desk),
    Gate(Gate),
    Hanger(Hanger),
    Machine(Machine),
    Meteor(Meteor),
    Monorail(Monorail),
    Pipe(Pipe),
    Platform(Platform),
    Rail(Rail),
    Rock(Rock),
    Rocket(Rocket),
    Stairs(Stairs),
    SatelliteDish(SatelliteDish),
    Supports(Supports),
    Structure(Structure),
    Terrain(Terrain),
    Turret(Turret),
    Weapon(Weapon),
    Rover,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Astronaut {
    #[default]
    A,
    B,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Barrel {
    #[default]
    Normal,
    Multiple,
    Rail,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Corridor {
    #[default]
    Normal,
    Open,
    Corner,
    CornerRound,
    CornerRoundWindow,
    Cross,
    Detailed,
    End,
    Roof,
    Split,
    WallCorner,
    Wall,
    WindowClosed,
    Window,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Craft {
    CargoA,
    CargoB,
    Miner,
    Racer,
    #[default]
    SpeederA,
    SpeederB,
    SpeederC,
    SpeederD,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Crater {
    #[default]
    Normal,
    Large,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Chimney {
    #[default]
    Normal,
    Detailed,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Desk {
    ChairArms,
    Chair,
    ChairStool,
    ComputerCorner,
    #[default]
    Computer,
    ComputerScreen,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Gate {
    Complex,
    #[default]
    Simple,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Hanger {
    LargeA,
    #[default]
    LargeB,
    RoundA,
    RoundB,
    RoundGlass,
    SmallA,
    SmallB,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Machine {
    Barrel,
    BarrelLarge,
    #[default]
    Generator,
    GeneratorLarge,
    WirelessCable,
    Wireless,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Meteor {
    #[default]
    Normal,
    Detailed,
    Half,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Monorail {
    TrackCornerLarge,
    TrackCornerSmall,
    TrackSlope,
    #[default]
    TrackStraight,
    TrackSupportCorner,
    TrackSupport,
    TrainBox,
    TrainCargo,
    TrainEnd,
    TrainFlat,
    TrainFront,
    TrainPassenger,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Pipe {
    CornerDiagonal,
    Corner,
    CornerRound,
    CornerRoundLarge,
    Cross,
    End,
    Entrance,
    #[default]
    Open,
    RampLarge,
    RampSmall,
    Ring,
    RingHighEnd,
    RingHigh,
    RingSupport,
    Split,
    Straight,
    SupportHigh,
    SupportLow,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Platform {
    #[default]
    Center,
    Corner,
    CornerOpen,
    CornerRound,
    End,
    High,
    Large,
    Long,
    Low,
    Side,
    SmallDiagonal,
    Small,
    Straight,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Rail {
    Corner,
    End,
    #[default]
    Normal,
    Middle,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Rock {
    #[default]
    Normal,
    LargeA,
    LargeB,
    SmallA,
    SmallB,
    Crystals,
    CrystalsLargeA,
    CrystalsLargeB,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Rocket {
    #[default]
    BaseA,
    BaseB,
    FinsA,
    FinsB,
    FuelA,
    FuelB,
    SidesA,
    SidesB,
    TopA,
    TopB,
}


#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum SatelliteDish {
    Detailed,
    #[default]
    Noraml,
    Large,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Stairs {
    Corner,
    #[default]
    Normal,
    Short,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Structure {
    #[default]
    Normal,
    Closed,
    Detailed,
    Diagonal,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Supports {
    High,
    #[default]
    Low,
}

#[derive(Debug, Inspectable, Default, Copy, Clone)]
pub enum Terrain {
    #[default]
    Normal,
    Ramp,
    RampLargeDetailed,
    RampLarge,
    RoadCorner,
    RoadCross,
    RoadEnd,
    RoadSplit,
    RoadStraight,
    SideCliff,
    SideCorner,
    SideCornerInner,
    SideEnd,
    Side,
}

#[derive(Component, Debug, Inspectable, Default, Copy, Clone)]
pub enum Turret {
    Double,
    #[default]
    Single,
}

#[derive(Component, Debug, Inspectable, Default, Copy, Clone)]
pub enum Weapon {
    #[default]
    Gun,
    Rifle,
}

