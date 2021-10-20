use bevy::math::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;

use strum_macros::EnumIter;

use super::GltfAssetType;
use super::init_asset_type_system;

pub struct CityPlugin;
impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init_asset_type_system::<CityType>);
        // registering custom component to be able to edit it in inspector
        let mut registry = app.world.get_resource_mut::<InspectableRegistry>().unwrap();
        registry.register::<CityType>();
    }
}

#[derive(Bundle, Default)]
pub struct CityAssetBundle {
    pub city_type: CityType,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}


#[derive(Component, EnumIter, PartialEq, Debug, Inspectable, Copy, Clone)]
pub enum CityType {
    Detail(Detail),
    Large(Large),
    Low(Low),
    Roof(Roof),
    Sign(Sign),
    Skyscraper(Skyscraper),
    Small(Small),
    Wall(Wall)
}

impl GltfAssetType for CityType {
    fn get_path(&self) -> &str {
        match self {
            CityType::Detail(c) => match c {
                Detail::Awning => "city/detail_awning.glb#Node-detail_awning",
                Detail::AwningWide => "city/detail_awningWide.glb#Node-detail_awningWide",
                Detail::Overhang => "city/detail_overhang.glb#Node-detail_overhang",
                Detail::OverhandWide => "city/detail_overhangWide.glb#Node-detail_overhangWide",
                Detail::UmbrellaDetailed => "city/detail_umbrellaDetailed.glb#Node-detail_umbrellaDetailed",
                Detail::Umbrella => "city/detail_umbrella.glb#Node-detail_umbrella",
            },
            CityType::Large(l) => match l {
                Large::BuildingA => "city/large_buildingA.glb#Node-large_buildingA",
                Large::BuildingB => "city/large_buildingB.glb#Node-large_buildingB",
                Large::BuildingC => "city/large_buildingC.glb#Node-large_buildingC",
                Large::BuildingD => "city/large_buildingD.glb#Node-large_buildingD",
                Large::BuildingE => "city/large_buildingE.glb#Node-large_buildingE",
                Large::BuildingF => "city/large_buildingF.glb#Node-large_buildingF",
                Large::BuildingG => "city/large_buildingG.glb#Node-large_buildingG",
            },
            CityType::Low(l) => match l {
                Low::BuildingA => "city/low_buildingA.glb#Node-low_buildingA",
                Low::BuildingB => "city/low_buildingB.glb#Node-low_buildingB",
                Low::BuildingC => "city/low_buildingC.glb#Node-low_buildingC",
                Low::BuildingD => "city/low_buildingD.glb#Node-low_buildingD",
                Low::BuildingE => "city/low_buildingE.glb#Node-low_buildingE",
                Low::BuildingF => "city/low_buildingF.glb#Node-low_buildingF",
                Low::BuildingG => "city/low_buildingG.glb#Node-low_buildingG",
                Low::BuildingH => "city/low_buildingH.glb#Node-low_buildingH",
                Low::BuildingI => "city/low_buildingI.glb#Node-low_buildingI",
                Low::BuildingJ => "city/low_buildingJ.glb#Node-low_buildingJ",
                Low::BuildingK => "city/low_buildingK.glb#Node-low_buildingK",
                Low::BuildingL => "city/low_buildingL.glb#Node-low_buildingL",
                Low::BuildingM => "city/low_buildingM.glb#Node-low_buildingM",
                Low::BuildingN => "city/low_buildingN.glb#Node-low_buildingN",
                Low::WideA => "city/low_wideA.glb#Node-low_wideA",
                Low::WideB => "city/low_wideB.glb#Node-low_wideB",
            },
            CityType::Roof(r) => match r {
                Roof::Center => "city/roof_center.glb#Node-roof_center",
                Roof::Corner => "city/roof_corner.glb#Node-roof_corner",
                Roof::Overhang => "city/roof_overhang.glb#Node-roof_overhang",
                Roof::Side => "city/roof_side.glb#Node-roof_side",
            },
            CityType::Sign(s) => match s {
                Sign::Billboard => "city/sign_billboard.glb#Node-sign_billboard",
                Sign::Hospital => "city/sign_hospital.glb#Node-sign_hospital",
            },
            CityType::Skyscraper(s) => match s {
                Skyscraper::BuildingA => "city/skyscraperA.glb#Node-skyscraperA",
                Skyscraper::BuildingB => "city/skyscraperB.glb#Node-skyscraperB",
                Skyscraper::BuildingC => "city/skyscraperC.glb#Node-skyscraperC",
                Skyscraper::BuildingD => "city/skyscraperD.glb#Node-skyscraperD",
                Skyscraper::BuildingE => "city/skyscraperE.glb#Node-skyscraperE",
                Skyscraper::BuildingF => "city/skyscraperF.glb#Node-skyscraperF",
            },
            CityType::Small(s) => match s {
                Small::BuildingA => "city/small_buildingA.glb#Node-small_buildingA",
                Small::BuildingB => "city/small_buildingB.glb#Node-small_buildingB",
                Small::BuildingC => "city/small_buildingC.glb#Node-small_buildingC",
                Small::BuildingD => "city/small_buildingD.glb#Node-small_buildingD",
                Small::BuildingE => "city/small_buildingE.glb#Node-small_buildingE",
                Small::BuildingF => "city/small_buildingF.glb#Node-small_buildingF",
            },
            CityType::Wall(w) => match w {
                Wall::DoorA => "city/wall_doorA.glb#Node-wall_doorA",
                Wall::DoorB => "city/wall_doorB.glb#Node-wall_doorB",
                Wall::Solid => "city/wall_solid.glb#Node-wall_solid",
                Wall::WindowA => "city/wall_windowA.glb#Node-wall_windowA",
                Wall::WindowB => "city/wall_windowB.glb#Node-wall_windowB",
                Wall::WindowC => "city/wall_windowC.glb#Node-wall_windowC",
                Wall::WindowD => "city/wall_windowD.glb#Node-wall_windowD",
                Wall::WindowE => "city/wall_windowE.glb#Node-wall_windowE",
                Wall::WindowF => "city/wall_windowF.glb#Node-wall_windowF",
            },
        }
    }
}

impl Default for CityType {
    fn default() -> Self {
        CityType::Large(Large::BuildingA)
    }
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Detail {
    #[default]
    Awning,
    AwningWide,
    Overhang,
    OverhandWide,
    Umbrella,
    UmbrellaDetailed,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Large {
    #[default]
    BuildingA,
    BuildingB,
    BuildingC,
    BuildingD,
    BuildingE,
    BuildingF,
    BuildingG,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Low {
    #[default]
    BuildingA,
    BuildingB,
    BuildingC,
    BuildingD,
    BuildingE,
    BuildingF,
    BuildingG,
    BuildingH,
    BuildingI,
    BuildingJ,
    BuildingK,
    BuildingL,
    BuildingM,
    BuildingN,
    WideA,
    WideB,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Roof {
    #[default]
    Center,
    Corner,
    Overhang,
    Side,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Sign {
    #[default]
    Billboard,
    Hospital,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Skyscraper {
    #[default]
    BuildingA,
    BuildingB,
    BuildingC,
    BuildingD,
    BuildingE,
    BuildingF,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Small {
    #[default]
    BuildingA,
    BuildingB,
    BuildingC,
    BuildingD,
    BuildingE,
    BuildingF,
}

#[derive(Debug, EnumIter, PartialEq,  Inspectable, Default, Copy, Clone)]
pub enum Wall {
    DoorA,
    DoorB,
    #[default]
    Solid,
    WindowA,
    WindowB,
    WindowC,
    WindowD,
    WindowE,
    WindowF,
}