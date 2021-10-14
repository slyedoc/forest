use bevy::gltf::Gltf;
use bevy::prelude::*;
use crate::prelude::Circle;
use crate::prelude::CityAssets;
use crate::prelude::ProceduralBundle;
use crate::prelude::spawn_city_debug;



pub fn spawner_system(mut commands: Commands, mut spawn_event: EventReader<SpawnEvent>,
    city: Res<CityAssets>,
    assets_gltf: Res<Assets<Gltf>>,
 ) {
    for e in spawn_event.iter() {

        match e.0 {
            SpawnType::Circle => { commands.spawn_bundle(ProceduralBundle {
                data: Circle::default(),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..Default::default()
            })
                .insert(Name::new("Circle"));
            },
            SpawnType::City => spawn_city_debug(&mut commands, &city, &assets_gltf),
            SpawnType::CityBuilding => {},
        }
    }
}



pub struct SpawnEvent(pub SpawnType);

#[derive(Debug, Clone)]
pub enum SpawnType {
    Circle,
    CityBuilding,
    City,
}
