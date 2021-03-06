mod ground;

use bevy::math::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use strum_macros::EnumIter;

pub use ground::*;

use super::GltfAssetType;
use super::init_asset_type_system;

pub struct SpacePlugin;
impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_asset_type_system::<SpaceType>);
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();
        // registering custom component to be able to edit it in inspector
        registry.register::<SpaceType>();
    }
}

#[derive(Bundle, Default)]
pub struct SpaceAssetBundle {
    pub space_type: SpaceType,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}


#[derive(Component, EnumIter, PartialEq, Debug, Inspectable, Copy, Clone)]
pub enum SpaceType {

    Character(Character),
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

impl Default for SpaceType {
    fn default() -> Self {
        SpaceType::Character(Character::AstronautA)
    }
}

// THis but using SpaceAssets lets us start a preload
// These clones are of the handle only,
impl GltfAssetType for SpaceType {
    fn get_path(&self) -> &str {
        match self {
            SpaceType::Character(c) => match c {
                Character::AstronautA => "space/astronautA.glb#Node-astronautA",
                Character::AstronautB => "space/astronautB.glb#Node-astronautB",
                Character::Alien => "space/alien.glb#Node-alien",
            },
            SpaceType::Barrel(b) => match b {
                Barrel::Normal => "space/barrel.glb#Node-barrel",
                Barrel::Multiple => "space/barrels.glb#Node-barrels",
                Barrel::Rail => "space/barrels_rail.glb#Node-barrels_rail",
            },
            SpaceType::Bones => "space/bones.glb#Node-bones",
            SpaceType::Chimney(c) => match c {
                Chimney::Normal => "space/chimney.glb#Node-chimney",
                Chimney::Detailed => "space/chimney_detailed.glb#Node-chimney_detailed",
            },
            SpaceType::Corridor(c) => match c {
                Corridor::Corner => "space/corridor_corner.glb#Node-corridor_corner",
                Corridor::CornerRound => "space/corridor_cornerRound.glb#Node-corridor_cornerRound",
                Corridor::CornerRoundWindow => "space/corridor_cornerRoundWindow.glb#Node-corridor_cornerRoundWindow",
                Corridor::Cross => "space/corridor_cross.glb#Node-corridor_cross",
                Corridor::Detailed => "space/corridor_detailed.glb#Node-corridor_detailed",
                Corridor::End => "space/corridor_end.glb#Node-corridor_end",
                Corridor::Normal => "space/corridor.glb#Node-corridor",
                Corridor::Open => "space/corridor_open.glb#Node-corridor_open",
                Corridor::Roof => "space/corridor_roof.glb#Node-corridor_roof",
                Corridor::Split => "space/corridor_split.glb#Node-corridor_split",
                Corridor::WallCorner => "space/corridor_wallCorner.glb#Node-corridor_wallCorner",
                Corridor::Wall => "space/corridor_wall.glb#Node-corridor_wall",
                Corridor::WindowClosed => "space/corridor_windowClosed.glb#Node-corridor_windowClosed",
                Corridor::Window => "space/corridor_window.glb#Node-corridor_window",
            },
            SpaceType::Crater(c) => match c {
                Crater::Normal => "space/crater.glb#Node-crater",
                Crater::Large => "space/craterLarge.glb#Node-crater",
            },

            SpaceType::Craft(c) => match c {
                Craft::CargoA => "space/craft_cargoA.glb#Node-craft_cargoA",
                Craft::CargoB => "space/craft_cargoB.glb#Node-craft_cargoB",
                Craft::Miner => "space/craft_miner.glb#Node-craft_miner",
                Craft::Racer => "space/craft_racer.glb#Node-craft_racer",
                Craft::SpeederA => "space/craft_speederA.glb#Node-craft_speederA",
                Craft::SpeederB => "space/craft_speederB.glb#Node-craft_speederB",
                Craft::SpeederC => "space/craft_speederC.glb#Node-craft_speederC",
                Craft::SpeederD => "space/craft_speederD.glb#Node-craft_speederD",
            },
            SpaceType::Desk(a) => match a {
                Desk::ChairArms => "space/desk_chairArms.glb#Node-desk_chairArms",
                Desk::Chair => "space/desk_chair.glb#Node-desk_chair",
                Desk::ChairStool => "space/desk_chairStool.glb#Node-desk_chairStool",
                Desk::ComputerCorner => "space/desk_computerCorner.glb#Node-desk_computerCorner",
                Desk::Computer => "space/desk_computer.glb#Node-desk_computer",
                Desk::ComputerScreen => "space/desk_computerScreen.glb#Node-desk_computerScreen",
            },
            SpaceType::Gate(a) => match a {
                Gate::Complex => "space/gate_complex.glb#Node-gate_complex",
                Gate::Simple => "space/gate_simple.glb#Node-gate_simple",
            },
            SpaceType::Hanger(a) => match a {
                Hanger::LargeA => "space/hangar_largeA.glb#Node-hangar_largeA",
                Hanger::LargeB => "space/hangar_largeB.glb#Node-hangar_largeB",
                Hanger::RoundA => "space/hangar_roundA.glb#Node-hangar_roundA",
                Hanger::RoundB => "space/hangar_roundB.glb#Node-hangar_roundB",
                Hanger::RoundGlass => "space/hangar_roundGlass.glb#Node-hangar_roundGlass",
                Hanger::SmallA => "space/hangar_smallA.glb#Node-hangar_smallA",
                Hanger::SmallB => "space/hangar_smallB.glb#Node-hangar_smallB",
            },
            SpaceType::Machine(a) => match a {
                Machine::Barrel => "space/machine_barrel.glb#Node-machine_barrel",
                Machine::BarrelLarge => "space/machine_barrelLarge.glb#Node-machine_barrelLarge",
                Machine::Generator => "space/machine_generator.glb#Node-machine_generator",
                Machine::GeneratorLarge => "space/machine_generatorLarge.glb#Node-machine_generatorLarge",
                Machine::WirelessCable => "space/machine_wirelessCable.glb#Node-machine_wirelessCable",
                Machine::Wireless => "space/machine_wireless.glb#Node-machine_wireless",
            },
            SpaceType::Meteor(a) => match a {
                Meteor::Normal => "space/meteor_detailed.glb#Node-meteor",
                Meteor::Detailed => "space/meteor.glb#Node-meteor_detailed",
                Meteor::Half => "space/meteor_half.glb#Node-meteor_half",
            },
            SpaceType::Monorail(a) => match a {
                Monorail::TrackCornerLarge => "space/monorail_trackCornerLarge.glb#Node-monorail_trackCornerLarge",
                Monorail::TrackCornerSmall => "space/monorail_trackCornerSmall.glb#Node-monorail_trackCornerSmall",
                Monorail::TrackSlope => "space/monorail_trackSlope.glb#Node-monorail_trackSlope",
                Monorail::TrackStraight => "space/monorail_trackStraight.glb#Node-monorail_trackStraight",
                Monorail::TrackSupportCorner => "space/monorail_trackSupportCorner.glb#Node-monorail_trackSupportCorner",
                Monorail::TrackSupport => "space/monorail_trackSupport.glb#Node-monorail_trackSupport",
                Monorail::TrainBox => "space/monorail_trainBox.glb#Node-monorail_trainBox",
                Monorail::TrainCargo => "space/monorail_trainCargo.glb#Node-monorail_trainCargo",
                Monorail::TrainEnd => "space/monorail_trainEnd.glb#Node-monorail_trainEnd",
                Monorail::TrainFlat => "space/monorail_trainFlat.glb#Node-monorail_trainFlat",
                Monorail::TrainFront => "space/monorail_trainFront.glb#Node-monorail_trainFront",
                Monorail::TrainPassenger => "space/monorail_trainPassenger.glb#Node-monorail_trainPassenger",
            },
            SpaceType::Pipe(a) => match a {
                Pipe::CornerDiagonal => "space/pipe_cornerDiagonal.glb#Node-pipe_cornerDiagonal",
                Pipe::Corner => "space/pipe_corner.glb#Node-pipe_corner",
                Pipe::CornerRound => "space/pipe_cornerRound.glb#Node-pipe_cornerRound",
                Pipe::CornerRoundLarge => "space/pipe_cornerRoundLarge.glb#Node-pipe_cornerRoundLarge",
                Pipe::Cross => "space/pipe_cross.glb#Node-pipe_cross",
                Pipe::End => "space/pipe_end.glb#Node-pipe_end",
                Pipe::Entrance => "space/pipe_entrance.glb#Node-pipe_entrance",
                Pipe::Open => "space/pipe_open.glb#Node-pipe_open",
                Pipe::RampLarge => "space/pipe_rampLarge.glb#Node-pipe_rampLarge",
                Pipe::RampSmall => "space/pipe_rampSmall.glb#Node-pipe_rampSmall",
                Pipe::Ring => "space/pipe_ring.glb#Node-pipe_ring",
                Pipe::RingHighEnd => "space/pipe_ringHighEnd.glb#Node-pipe_ringHighEnd",
                Pipe::RingHigh => "space/pipe_ringHigh.glb#Node-pipe_ringHigh",
                Pipe::RingSupport => "space/pipe_ringSupport.glb#Node-pipe_ringSupport",
                Pipe::Split => "space/pipe_split.glb#Node-pipe_split",
                Pipe::Straight => "space/pipe_straight.glb#Node-pipe_straight",
                Pipe::SupportHigh => "space/pipe_supportHigh.glb#Node-pipe_supportHigh",
                Pipe::SupportLow => "space/pipe_supportLow.glb#Node-pipe_supportLow",
            },
            SpaceType::Platform(a) => match a {
                Platform::Center => "space/platform_center.glb#Node-platform_center",
                Platform::Corner => "space/platform_corner.glb#Node-platform_corner",
                Platform::CornerOpen => "space/platform_cornerOpen.glb#Node-platform_cornerOpen",
                Platform::CornerRound => "space/platform_cornerRound.glb#Node-platform_cornerRound",
                Platform::End => "space/platform_end.glb#Node-platform_end",
                Platform::High => "space/platform_high.glb#Node-platform_high",
                Platform::Large => "space/platform_large.glb#Node-platform_large",
                Platform::Long => "space/platform_long.glb#Node-platform_long",
                Platform::Low => "space/platform_low.glb#Node-platform_low",
                Platform::Side => "space/platform_side.glb#Node-platform_side",
                Platform::SmallDiagonal => "space/platform_smallDiagonal.glb#Node-platform_smallDiagonal",
                Platform::Small => "space/platform_small.glb#Node-platform_small",
                Platform::Straight => "space/platform_straight.glb#Node-platform_straight",
            },
            SpaceType::Rail(a) => match a {
                Rail::Corner => "space/rail_corner.glb#Node-rail_corner",
                Rail::End => "space/rail_end.glb#Node-rail_end",
                Rail::Normal => "space/rail.glb#Node-rail",
                Rail::Middle => "space/rail_middle.glb#Node-rail_middle",
            },
            SpaceType::Rock(a) => match a {
                Rock::Normal => "space/rock.glb#Node-rock",
                Rock::LargeA => "space/rock_largeA.glb#Node-rock_largeA",
                Rock::LargeB => "space/rock_largeB.glb#Node-rock_largeB",
                Rock::SmallA => "space/rocks_smallA.glb#Node-rocks_smallA",
                Rock::SmallB => "space/rocks_smallB.glb#Node-rocks_smallB",
                Rock::Crystals => "space/rock_crystals.glb#Node-rock_crystals",
                Rock::CrystalsLargeA => "space/rock_crystalsLargeA.glb#Node-rock_crystalsLargeA",
                Rock::CrystalsLargeB => "space/rock_crystalsLargeB.glb#Node-rock_crystalsLargeB",
            },
            SpaceType::Rocket(a) => match a {
                Rocket::BaseA => "space/rocket_baseA.glb#Node-rocket_baseA",
                Rocket::BaseB => "space/rocket_baseB.glb#Node-rocket_baseB",
                Rocket::FinsA => "space/rocket_finsA.glb#Node-rocket_finsA",
                Rocket::FinsB => "space/rocket_finsB.glb#Node-rocket_finsB",
                Rocket::FuelA => "space/rocket_fuelA.glb#Node-rocket_fuelA",
                Rocket::FuelB => "space/rocket_fuelB.glb#Node-rocket_fuelB",
                Rocket::SidesA => "space/rocket_sidesA.glb#Node-rocket_sidesA",
                Rocket::SidesB => "space/rocket_sidesB.glb#Node-rocket_sidesB",
                Rocket::TopA => "space/rocket_topA.glb#Node-rocket_topA",
                Rocket::TopB => "space/rocket_topB.glb#Node-rocket_topB",
            },
            SpaceType::Rover => "space/rover.glb#Node-rover",
            SpaceType::Stairs(a) => match a {
                Stairs::Corner => "space/stairs_corner.glb#Node-stairs_corner",
                Stairs::Normal => "space/stairs.glb#Node-stairs",
                Stairs::Short => "space/stairs_short.glb#Node-stairs_short",
            },
            SpaceType::SatelliteDish(a) => match a {
                SatelliteDish::Detailed => "space/satelliteDish_detailed.glb#Node-satelliteDish_detailed",
                SatelliteDish::Normal => "space/satelliteDish.glb#Node-satelliteDish",
                SatelliteDish::Large => "space/satelliteDish_large.glb#Node-satelliteDish_large",
            },
            SpaceType::Supports(a) => match a {
                Supports::High => "space/supports_high.glb#Node-supports_high",
                Supports::Low => "space/supports_low.glb#Node-supports_low",
            },
            SpaceType::Structure(a) => match a {
                Structure::Normal => "space/structure.glb#Node-structure",
                Structure::Closed => "space/structure_closed.glb#Node-structure_closed",
                Structure::Detailed => "space/structure_detailed.glb#Node-structure_detailed",
                Structure::Diagonal => "space/structure_diagonal.glb#Node-structure_diagonal",
            },
            SpaceType::Terrain(a) => match a {
                Terrain::Normal => "space/terrain.glb#Node-terrain",
                Terrain::Ramp => "space/terrain_ramp.glb#Node-terrain_ramp",
                Terrain::RampLargeDetailed => "space/terrain_rampLarge_detailed.glb#Node-terrain_rampLarge_detailed",
                Terrain::RampLarge => "space/terrain_rampLarge.glb#Node-terrain_rampLarge",
                Terrain::RoadCorner => "space/terrain_roadCorner.glb#Node-terrain_roadCorner",
                Terrain::RoadCross => "space/terrain_roadCross.glb#Node-terrain_roadCross",
                Terrain::RoadEnd => "space/terrain_roadEnd.glb#Node-terrain_roadEnd",
                Terrain::RoadSplit => "space/terrain_roadSplit.glb#Node-terrain_roadSplit",
                Terrain::RoadStraight => "space/terrain_roadStraight.glb#Node-terrain_roadStraight",
                Terrain::SideCliff => "space/terrain_sideCliff.glb#Node-terrain_sideCliff",
                Terrain::SideCorner => "space/terrain_sideCorner.glb#Node-terrain_sideCorner",
                Terrain::SideCornerInner => "space/terrain_sideCornerInner.glb#Node-terrain_sideCornerInner",
                Terrain::SideEnd => "space/terrain_sideEnd.glb#Node-terrain_sideEnd",
                Terrain::Side => "space/terrain_side.glb#Node-terrain_side",
            },
            SpaceType::Turret(a) => match a {
                Turret::Double => "space/turret_double.glb#Node-turret_double",
                Turret::Single => "space/turret_single.glb#Node-turret_single",
            },
            SpaceType::Weapon(a) => match a {
                Weapon::Gun => "space/weapon_gun.glb#Node-weapon_gun",
                Weapon::Rifle => "space/weapon_rifle.glb#Node-weapon_rifle",
            },
        }
    }
}
///////////////////////////////////////////////////////////////
// Below this point are just enums to make using these assets easier
///////////////////////////////////////////////////////////////



#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Character {
    Alien,
    #[default]
    AstronautA,
    AstronautB,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Barrel {
    #[default]
    Normal,
    Multiple,
    Rail,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Crater {
    #[default]
    Normal,
    Large,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Chimney {
    #[default]
    Normal,
    Detailed,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Desk {
    ChairArms,
    Chair,
    ChairStool,
    ComputerCorner,
    #[default]
    Computer,
    ComputerScreen,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Gate {
    Complex,
    #[default]
    Simple,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Machine {
    Barrel,
    BarrelLarge,
    #[default]
    Generator,
    GeneratorLarge,
    WirelessCable,
    Wireless,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Meteor {
    #[default]
    Normal,
    Detailed,
    Half,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug,  EnumIter, PartialEq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Rail {
    Corner,
    End,
    #[default]
    Normal,
    Middle,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq, Inspectable, Default, Copy, Clone)]
pub enum SatelliteDish {
    Detailed,
    #[default]
    Normal,
    Large,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Stairs {
    Corner,
    #[default]
    Normal,
    Short,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Structure {
    #[default]
    Normal,
    Closed,
    Detailed,
    Diagonal,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Supports {
    High,
    #[default]
    Low,
}

#[derive(Debug, EnumIter, PartialEq, Inspectable, Default, Copy, Clone)]
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

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Turret {
    Double,
    #[default]
    Single,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Weapon {
    #[default]
    Gun,
    Rifle,
}
