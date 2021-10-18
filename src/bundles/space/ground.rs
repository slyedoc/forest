use bevy::prelude::*;
use bevy::math::*;

use super::*;

pub fn build_ground(commands: &mut Commands) -> Entity {

    let spacing = 1.0;
    let grid_size = UVec2::new(10, 6);
    let grid = [
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        [1, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];


    let board = commands.spawn()
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .insert(Name::new("Ground"))
            .id();

    for x in 0..grid_size.x {
        let pos_x = x as f32 * spacing;// - (grid_size.x as f32 * spacing * 0.5);
        for y in 0..grid_size.y {
            let pos_y = y as f32 * spacing;// - (grid_size.y as f32 * spacing * 0.5);
            let piece = grid[y as usize][x as usize]; // order is backwards to make editing easy
            
            let terrain: Option<SpaceType> = match piece {
                1 => Some(SpaceType::Terrain(Terrain::Normal)),
                _ => None,
            };

            if let Some(t) = terrain {
                commands.entity(board).with_children(|parent| {
                    parent.spawn_bundle(SpaceAssetBundle {
                        building: t,
                        transform: Transform::from_xyz( pos_x, 0.0, pos_y ),
                        ..Default::default()
                    })
                    
                    .insert(Name::new(format!("Piece ({},{})", x, y )));
                });
            }
        }
    }
    board
}

