use bevy::math::*;
use bevy::prelude::*;

use super::*;

pub fn build_ground(commands: &mut Commands, grid_size: [usize; 2]) -> Entity {
    let spacing = 1.0;
    let board = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Name::new("Ground"))
        .id();

    for x in 0..grid_size[0] {
        let pos_x = x as f32 * spacing; // - (grid_size.x as f32 * spacing * 0.5);
        for y in 0..grid_size[1] {
            let pos_y = y as f32 * spacing; // - (grid_size.y as f32 * spacing * 0.5);
            commands.entity(board).with_children(|parent| {
                parent
                    .spawn_bundle(SpaceAssetBundle {
                        space_type: SpaceType::Terrain(Terrain::Normal),
                        transform: Transform::from_xyz(pos_x, 0.0, pos_y),
                        ..Default::default()
                    })
                    .insert(Name::new(format!("Piece ({},{})", x, y)));
            });
        }
    }
    board
}
